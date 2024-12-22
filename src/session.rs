use std::process::Command;
use std::os::unix::process::CommandExt;
use zbus::{
	Connection,
	Proxy,
	zvariant::{OwnedObjectPath, Value},
};
use users::{
	get_user_by_name,
	User,
};
use users::os::unix::UserExt;
use cursive::Cursive;
use cursive::views::EditView;

use crate::message::draw_error_message;


pub async fn get_session_id() -> Result<String, String> {
	// Connect to the system bus
	let conn = Connection::system()
		.await
		.map_err(|e| format!("Failed to connect to D-Bus: {}", e))?;

	// Create a proxy for the org.freedesktop.login1.Manager interface
	let proxy = Proxy::new(
		&conn,
		"org.freedesktop.login1",
		"/org/freedesktop/login1",
		"org.freedesktop.login1.Manager",
	)
	.await
	.map_err(|e| format!("Failed to create proxy: {}", e))?;

	// Get the current process ID
	let pid = std::process::id();

	// Call GetSessionByPID to retrieve the session object path
	let message = proxy
		.call_method("GetSessionByPID", &(pid as u32))
		.await
		.map_err(|e| format!("Failed to call GetSessionByPID: {}", e))?;

	// Deserialize the message body into OwnedObjectPath
	let session_path: OwnedObjectPath = message
		.body()
		.deserialize::<OwnedObjectPath>()
		.map_err(|e| format!("Failed to deserialize message body: {}", e))?;

	// Extract the session ID from the object path
	let session_path_str = session_path.as_str();
	let session_id = session_path_str
		.rsplit('/')
		.next()
		.ok_or_else(|| "Failed to parse session ID from object path".to_string())?;

	Ok(session_id.to_string())
}

pub async fn set_session_class(session_id: &str, class: &str) -> Result<(), String> {
	let conn = Connection::system()
		.await
		.map_err(|e| format!("{}", e))?;

	let proxy = zbus::Proxy::new(
		&conn,
		"org.freedesktop.login1",
		format!("/org/freedesktop/login1/session/{}", session_id),
		"org.freedesktop.login1.Session",
	)
	.await
	.map_err(|e| format!("{}", e))?;

	proxy
		.call_method("SetClass", &(class))
		.await
		.map_err(|e| format!("{}", e))?;

	Ok(())
}

fn auth_user<'a>(
	username: &'a str,
	password: &'a str
) -> Result<pam::Client<'a, pam::PasswordConv>, String> {
	let mut client = pam::Client::with_password("login")
		.map_err(|_| "Authentication failed!".to_string())?;

	// Preset the login & password we will use for authentication
	client
		.conversation_mut()
		.set_credentials(username, password);
	client
		.authenticate()
		.map_err(|_| "Authentication failed!".to_string())?;

	Ok(client)
}

fn get_active_tty() -> Result<String, String> {
	let fd_path = "/proc/self/fd/0";
	let tty_path = std::fs::read_link(fd_path)
		.map_err(|_| format!("Failed to read link for {}.", fd_path))?;

	let tty_str = tty_path
		.to_str()
		.ok_or("Failed to convert TTY path to string.")?;

	if tty_str.starts_with("/dev/") {
		Ok(tty_str.to_string())
	} else {
		Err("Active TTY not found.".to_string())
	}
}

fn change_tty_ownership(user: &User) -> Result<(), String> {
	// Get the current TTY
	let tty_path = get_active_tty()?;

	// Change the TTY ownership
	let tty_cstr = std::ffi::CString::new(tty_path.clone())
		.map_err(|_| "Failed to convert TTY path.".to_string())?;

	let chown_result = unsafe {
		libc::chown(tty_cstr.as_ptr(), user.uid(), user.primary_group_id())
	};

	if chown_result != 0 {
		return Err(format!("Failed to change ownership of TTY '{}'.", tty_path));
	}

	Ok(())
}

fn start_user_session(user: &User) -> Result<(), String> {
	// Change to the user's home directory
	let home_dir = user.home_dir().to_str().ok_or("Invalid home directory.")?;
	std::env::set_var("HOME", home_dir);
	let shell = user
		.shell()
		.to_str()
		.ok_or("Failed to start user's shell.".to_string())?;

	// Set up other environment variables
	std::env::set_var("USER", user.name());
	std::env::set_var("LOGNAME", user.name());
	std::env::set_var("SHELL", shell);

	// Attempt to switch to the user's UID and GID
	unsafe {
		if libc::setgid(user.primary_group_id()) != 0 {
			return Err("Failed to set group ID.".to_string());
		}

		if libc::setuid(user.uid()) != 0 {
			return Err("Failed to set user ID.".to_string());
		}
	}

	// Execute the user's default shell
	Command::new(shell)
		.arg("-l") // Login shell
		.exec(); // Replace the current process with the shell

	Err("Failed to start the shell session.".to_string()) // This point should never be reached
}

pub fn start_session(siv: &mut Cursive) {
	let username = siv
		.call_on_name("username", |view: &mut EditView| view.get_content())
		.unwrap();
	let password = siv
		.call_on_name("password", |view: &mut EditView| view.get_content())
		.unwrap();

	match auth_user(&username, &password) {
		Ok(mut client) => {
			if client.open_session().is_ok() {
				let user = get_user_by_name(&*username).unwrap();

				if let Err(err) = change_tty_ownership(&user) {
					draw_error_message(
						siv,
						&format!("Error changing TTY ownership: {}", err)
					);
					return;
				}

				tokio::runtime::Runtime::new().unwrap().block_on(async {
					match get_session_id().await {
						Ok(session_id) => {
							if let Err(e) = set_session_class(&session_id, "user").await {
								draw_error_message(
									siv,
									&format!("Failed to set session class: {}", e),
								);
								return;
							}
						},
						Err(e) => {
							eprintln!("{}", &e);
							draw_error_message(siv, &e);
						},
					};
				});

				siv.quit();

				if let Err(err) = start_user_session(&user) {
					eprintln!("Error starting user session: {}", err);
				}
			} else {
				eprintln!("Failed to open PAM session.");
			}
		}
		Err(message) => draw_error_message(siv, &message),
	};
}
