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

use crate::message::draw_error_message;

fn auth_user(
	username: &str,
	password: &str,
) -> Result<User, String> {
	let mut client = pam::Client::with_password("tuilog")
		.map_err(|_| "Unable to authenticate.".to_string())?;

	// Preset the login & password we will use for authentication
	client
		.conversation_mut()
		.set_credentials(username, password);
	client
		.authenticate()
		.map_err(|_| "Incorrect username or password.".to_string())?;

	let user = get_user_by_name(&*username)
		.ok_or_else(|| format!("Failed to fetch user named {}.", &username))?;

	client
		.open_session()
		.map_err(|_| "Unable to authenticate.".to_string())?;

	Ok(user)
}

fn spawn_shell(user: &User) -> Result<Child, String> {
	let shell_path = user
		.shell()
		.to_str()
		.ok_or_else(|| "Failed to open shell.".to_string())?;

	let child = Command::new(&shell_path)
		.current_dir(user.home_dir())
		.arg("-l")  // '-l' to start as a login shell
		.arg("-c") // Run an initialization command
		.arg(format!("stty sane; tput sgr0; tput cnorm; clear; exec {} -l", &shell_path))
		.stdin(std::process::Stdio::inherit())
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.spawn()  // Launch the process
		.map_err(|_| "Failed to open shell.".to_string())?;

	Ok(child)
}

fn set_process_ids(user: &User) -> Result<(), String> {
	let uid = Uid::from_raw(user.uid());
	let gid = Gid::from_raw(user.primary_group_id());

	// Change the process UID and GID to the authenticated user
	setgid(gid)
		.map_err(|_| "Failed to start login session.".to_string())?;
	setuid(uid)
		.map_err(|_| "Failed to start login session.".to_string())?;

	Ok(())
}

pub fn start_session(siv: &mut Cursive) -> Result<(), String> {
	fn get_view_content(view: &mut EditView) -> Arc<String> {
		view.get_content()
	}
	let username = siv
		.call_on_name("username", get_view_content)
		.ok_or_else(|| "Unexpected error occured.".to_string())?;
	let password = siv
		.call_on_name("password", get_view_content)
		.ok_or_else(|| "Unexcepted error occured.".to_string())?;

	match auth_user(&username, &password) {
		Ok(user) => {
			set_process_ids(&user)?;
			siv.quit();
			let mut child = spawn_shell(&user)?;
			if let Err(err) = child.wait() {
				eprintln!("Error starting user session: {}", err);
			}
		},
		Err(message) => draw_error_message(siv, &message),
	};

	Ok(())
}
