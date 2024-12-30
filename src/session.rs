use std::process::{
	Command,
	Child,
	Stdio,
};
use std::sync::Arc;
use users::{
	get_user_by_name,
	User,
	os::unix::UserExt,
};
use cursive::{
	Cursive,
	views::{
		EditView,
		SelectView,
	},
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
use crate::cache::set_default_options;

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

	let user = get_user_by_name(username)
		.tuilog_err(TUILogError::UserNotFound)?;

	client
		.open_session()
		.tuilog_err(TUILogError::AuthenticationFailed)?;

	Ok(user)
}

fn spawn_shell(user: &User, session_type: u8) -> TUILogResult<Child> {
	let shell_path = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::LoginShellFailed)?;

	let child = match session_type {
		0 => Command::new(&shell_path)
				.current_dir(user.home_dir())
				.arg("-l")  // '-l' to start as a login shell
				.arg("-c") // Run an initialization command
				.arg(
					format!(
						"stty sane; tput sgr0; tput cnorm; clear; exec {} -l",
						&shell_path
					)
				)
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.spawn()  // Launch the process
				.tuilog_err(TUILogError::LoginShellFailed)?,
		1 => Command::new(&shell_path) // use the user's login shell to run startx
				.current_dir(user.home_dir())
				.arg("-l")
				.arg("-c")
				.arg("startx")
				.env("HOME", user.home_dir())
				.env("USER", user.name())
				.env("LOGNAME", user.name())
				.stdin(Stdio::inherit())
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.spawn() // Launch the process
				.tuilog_err(TUILogError::X11SessionFailed)?,
		_ => {
			return Err(TUILogError::InvalidSessionOption);
		},
	};

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
	let session_id =
		siv.call_on_name(
			"session",
			|view: &mut SelectView<u8>| {
				match view.selection() {
					Some(selected) => *selected,
					None => 0, // default to shell
				}
			}
		)
		.tuilog_err(TUILogError::InvalidSessionOption)?;

	// has to be before auth_user as the cache file is only accessible to root
	set_default_options(
		username.to_string(),
		session_id,
	);
	let user = auth_user(&username, &password)?;
	set_process_ids(&user)?;

	let mut child = spawn_shell(&user, session_id)?;

	siv.quit();

	if let Err(err) = child.wait() {
		eprintln!("Failed to start user session: {}", err);
	}

	Ok(())
}

pub fn get_sessions() -> Vec<(String, u8)> {
	let mut sessions = Vec::new();

	sessions.push((
		"shell".to_string(),
		0,
	));

	sessions.push((
		"startx".to_string(),
		1,
	));

	sessions
}
