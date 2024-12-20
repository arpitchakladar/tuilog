use std::process::{
	Command,
	Stdio
};
use std::os::unix::process::CommandExt;
use pam::Client;
use users::os::unix::UserExt;
use users::get_user_by_name;
use libc;

pub fn auth_user(username: &str, password: &str) -> Result<(), String> {
	let mut client = Client::with_password("login")
		.map_err(|_| {
			"Authentication failed!".to_string()
		})?;
	// Preset the login & password we will use for authentication
	client.conversation_mut().set_credentials(username, password);
	// Actually try to authenticate:
	client.authenticate().map_err(|_| {
		"Authentication failed!".to_string()
	})?;
	// Now that we are authenticated, it's possible to open a sesssion:
	client.open_session().map_err(|_| {
		"Failed to open a session!".to_string()
	})?;

	Ok(())
}

pub fn launch_session(username: &str) -> Result<(), String> {
	match get_user_by_name(username) {
		Some(user_info) => {
			let _ = Command::new("clear").status();
			let home_dir = user_info.home_dir();
			let shell = user_info.shell();

			unsafe {
				if libc::setsid() == -1 {
					return Err("Failed to create a new session using setsid!".to_string());
				}
			}

			Command::new(shell)
				.arg("-l") // Login shell
				.env("HOME", home_dir)
				.env("USER", username)
				.env("LOGNAME", username)
				.uid(user_info.uid())
				.gid(user_info.primary_group_id())
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.spawn()
				.map_err(|e| format!("Failed to start user session: {}", e))?
				.wait()
				.map_err(|e| format!("Failed to start user session: {}", e))?;

			Err("Failed to launch session".to_string())
		},
		None => Err("Failed to access user!".to_string()),
	}
}
