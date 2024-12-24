use std::process::Command;
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
) -> Result<(), String> {
	let mut client = pam::Client::with_password("tuilog")
		.map_err(|_| "Unable to authenticate.".to_string())?;

	// Preset the login & password we will use for authentication
	client
		.conversation_mut()
		.set_credentials(username, password);
	client
		.authenticate()
		.map_err(|_| "Incorrect username or password.".to_string())?;
	client
		.open_session()
		.map_err(|_| "Unable to authenticate.".to_string())?;

	Ok(())
}

fn spawn_shell(user: &User) -> Result<(), String> {
	let shell_path = user
		.shell()
		.to_str()
		.ok_or_else(|| "Failed to open shell.".to_string())?;

	Command::new(&shell_path)
		.current_dir(user.home_dir())
		.arg("-l")  // '-l' to start as a login shell
		.arg("-c") // Run an initialization command
		.arg(format!("stty sane; tput sgr0; tput cnorm; clear; exec {} -l", &shell_path))
		.stdin(std::process::Stdio::inherit())
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.spawn()  // Launch the process
		.map_err(|_| "Failed to open shell.".to_string())?
		.wait()  // Wait for the shell process to finish
		.map_err(|_| "Failed to open shell.".to_string())?;

	Ok(())
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
	let username = siv
		.call_on_name("username", |view: &mut EditView| view.get_content())
		.ok_or_else(|| "Unexpected error occured.".to_string())?;
	let password = siv
		.call_on_name("password", |view: &mut EditView| view.get_content())
		.ok_or_else(|| "Unexcepted error occured.".to_string())?;

	match auth_user(&username, &password) {
		Ok(_) => {
			let user = get_user_by_name(&*username)
				.ok_or_else(|| format!("Failed to fetch user named {}.", &username))?;

			set_process_ids(&user)?;
			siv.quit();

			if let Err(err) = spawn_shell(&user) {
				eprintln!("Error starting user session: {}", err);
			}
		}
		Err(message) => draw_error_message(siv, &message),
	};

	Ok(())
}
