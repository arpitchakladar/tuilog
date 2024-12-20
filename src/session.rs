use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use users::os::unix::UserExt;
use users::get_user_by_name;

pub fn auth_user(username: &str, password: &str) -> Result<(), String> {
	let mut client = pam::Client::with_password("login")
		.map_err(|_| "Authentication failed!".to_string())?;

	// Preset the login & password we will use for authentication
	client.conversation_mut().set_credentials(username, password);

	// Actually try to authenticate:
	client.authenticate().map_err(|_| "Authentication failed!".to_string())?;

	// Now that we are authenticated, it's possible to open a session:
	client.open_session().map_err(|_| "Failed to open a session!".to_string())?;

	Ok(())
}

pub fn launch_session(username: &str) -> Result<(), String> {
	match get_user_by_name(username) {
		Some(user_info) => {
			let home_dir = user_info.home_dir();
			let shell = user_info.shell();

			// unsafe {
			// 	// Step 1: Become session leader
			// 	if libc::setsid() == -1 {
			// 		return Err(format!(
			// 			"Failed to create a new session: {:?}",
			// 			std::io::Error::last_os_error()
			// 		));
			// 	}
			// }

			// Ensure a clean TTY for the new session
			Command::new("reset").status().ok();
			Command::new("clear").status().ok();

			let xauthority_path = format!("{}/.Xauthority", home_dir.display());
			let display = ":0"; // Adjust based on your system configuration

			// Set environment variables and launch the shell
			let mut child = Command::new(shell)
				.env("HOME", home_dir)
				.env("USER", username)
				.env("LOGNAME", username)
				.env("SHELL", shell)
				.env("DISPLAY", display) // X11 display
				.env("XAUTHORITY", xauthority_path) // X11 auth file
				.env("XDG_SESSION_TYPE", "tty")
				.uid(user_info.uid())
				.gid(user_info.primary_group_id())
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.spawn()
				.map_err(|e| format!("Failed to start user session: {}", e))?;

			// Wait for the user shell to exit
			let status = child
				.wait()
				.map_err(|e| format!("Failed to wait for user shell: {}", e))?;

			// Handle session cleanup after the shell exits
			if !status.success() {
				return Err("User shell exited with an error.".to_string());
			}

			Ok(())
		}
		None => Err("Failed to access user!".to_string()),
	}
}
