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

fn auth_user<'a>(
	username: &'a str,
	password: &'a str
) -> Result<pam::Client<'a, pam::PasswordConv>, String> {
	let mut client = pam::Client::with_password("tuilog")
		.map_err(|err| format!("Authentication failed! {}", err))?;

	// Preset the login & password we will use for authentication
	client
		.conversation_mut()
		.set_credentials(username, password);
	client
		.authenticate()
		.map_err(|err| format!("Authentication failed! {}", err))?;
	client
		.open_session()
		.map_err(|err| format!("Failed to open PAM session. {}", err))?;

	Ok(client)
}

fn spawn_shell(user: &User) -> Result<(), String> {
	let shell_path = user
		.shell()
		.to_str()
		.ok_or("Failed to start user's shell.".to_string())?;
	Command::new(&shell_path)
		.current_dir(user.home_dir())
		.arg("-l")  // '-l' to start as a login shell
		.arg("-c") // Run an initialization command
		.arg(format!("stty sane; tput sgr0; tput cnorm; clear; exec {} -l", &shell_path))
		.stdin(std::process::Stdio::inherit())
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.spawn()  // Launch the process
		.map_err(|err| format!("Failed to start shell: {}", err))?
		.wait()  // Wait for the shell process to finish
		.map_err(|err| format!("Failed to wait for shell: {}", err))?;

	Ok(())
}

fn set_process_ids(user: &User) -> Result<(), String> {
	let uid = Uid::from_raw(user.uid());
	let gid = Gid::from_raw(user.primary_group_id());

	// Change the process UID and GID to the authenticated user
	setgid(gid)
		.map_err(|err| err.to_string())?;
	setuid(uid)
		.map_err(|err| err.to_string())?;

	Ok(())
}

pub fn start_session(siv: &mut Cursive) {
	let username = siv
		.call_on_name("username", |view: &mut EditView| view.get_content())
		.unwrap();
	let password = siv
		.call_on_name("password", |view: &mut EditView| view.get_content())
		.unwrap();

	match auth_user(&username, &password) {
		Ok(_client) => {
			// TODO: Handle if user not found
			let user = get_user_by_name(&*username).unwrap();
			set_process_ids(&user).unwrap();

			siv.quit();
			if let Err(err) = spawn_shell(&user) {
				eprintln!("Error starting user session: {}", err);
			}
		}
		Err(message) => draw_error_message(siv, &message),
	};
}
