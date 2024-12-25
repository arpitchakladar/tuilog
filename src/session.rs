use std::process::{
	Command,
	Child,
};
use std::sync::Arc;
use users::{
	get_user_by_name,
	User,
	os::unix::UserExt,
};
use cursive::{
	Cursive,
	views::EditView,
};
use nix::unistd::{
	setgid,
	setuid,
	Gid,
	Uid,
};

use crate::error::{
	TUILogError,
	TUILogErrorMap,
	TUILogResult,
};

fn auth_user(
	username: &str,
	password: &str,
) -> TUILogResult<User> {
	let mut client = pam::Client::with_password("tuilog")
		.tuilog_err(TUILogError::AuthenticationFailed)?;

	// Preset the login & password we will use for authentication
	client
		.conversation_mut()
		.set_credentials(username, password);
	client
		.authenticate()
		.tuilog_err(TUILogError::Unauthorized)?;

	let user = get_user_by_name(&*username)
		.tuilog_err(TUILogError::UserNotFound)?;

	client
		.open_session()
		.tuilog_err(TUILogError::AuthenticationFailed)?;

	Ok(user)
}

fn spawn_shell(user: &User) -> TUILogResult<Child> {
	let shell_path = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::LoginShellFailed)?;

	let child = Command::new(&shell_path)
		.current_dir(user.home_dir())
		.arg("-l")  // '-l' to start as a login shell
		.arg("-c") // Run an initialization command
		.arg(
			format!(
				"stty sane; tput sgr0; tput cnorm; clear; exec {} -l",
				&shell_path
			)
		)
		.stdin(std::process::Stdio::inherit())
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.spawn()  // Launch the process
		.tuilog_err(TUILogError::LoginShellFailed)?;

	Ok(child)
}

fn set_process_ids(user: &User) -> TUILogResult<()> {
	let uid = Uid::from_raw(user.uid());
	let gid = Gid::from_raw(user.primary_group_id());

	// Change the process UID and GID to the authenticated user
	setgid(gid)
		.tuilog_err(TUILogError::LoginSessionFailed)?;
	setuid(uid)
		.tuilog_err(TUILogError::LoginSessionFailed)?;

	Ok(())
}

pub fn start_session(siv: &mut Cursive) -> TUILogResult<()> {
	fn get_view_content(view: &mut EditView) -> Arc<String> {
		view.get_content()
	}
	let username = siv
		.call_on_name("username", get_view_content)
		.tuilog_err(TUILogError::AuthenticationFailed)?;
	let password = siv
		.call_on_name("password", get_view_content)
		.tuilog_err(TUILogError::AuthenticationFailed)?;

	let user = auth_user(&username, &password)?;
	set_process_ids(&user)?;
	let mut child = spawn_shell(&user)?;

	siv.quit();

	if let Err(err) = child.wait() {
		eprintln!("Failed to start user session: {}", err);
	}

	Ok(())
}
