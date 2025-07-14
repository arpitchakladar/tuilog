use cursive::{
	views::{EditView, SelectView},
	Cursive,
};
use nix::fcntl::{open, OFlag};
use nix::libc;
use nix::sys::stat::Mode;
use nix::sys::wait::waitpid;
use nix::unistd::{
	chown, dup2, execvp, fork, initgroups, setgid, setsid, setuid, ForkResult,
	Gid, Uid,
};
use std::ffi::CString;
use std::sync::Arc;
use users::{get_user_by_name, os::unix::UserExt, User};

use crate::cache::set_default_options;
use crate::error::{TUILogError, TUILogErrorMap, TUILogResult};
use crate::utils::get_current_tty_path;

fn auth_user<'a>(
	username: &'a str,
	password: &'a str,
) -> TUILogResult<(pam::Client<'a, pam::PasswordConv>, User)> {
	let mut client = pam::Client::with_password("tuilog")
		.tuilog_err(TUILogError::AuthenticationFailed)?;

	// Preset the login & password we will use for authentication
	client
		.conversation_mut()
		.set_credentials(username, password);
	client
		.authenticate()
		.tuilog_err(TUILogError::Unauthorized)?;

	let user =
		get_user_by_name(username).tuilog_err(TUILogError::UserNotFound)?;

	client
		.open_session()
		.tuilog_err(TUILogError::AuthenticationFailed)?;

	Ok((client, user))
}

fn set_environment(user: &User) -> TUILogResult<()> {
	let shell_path = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::LoginShellFailed)?;
	std::env::set_var("USER", user.name());
	std::env::set_var("LOGNAME", user.name());
	std::env::set_var("HOME", user.home_dir());
	std::env::set_var("SHELL", shell_path);
	std::env::set_var("TERM", "linux");

	// Set XDG variables if running a desktop session
	std::env::set_var("XDG_SESSION_TYPE", "tty");
	std::env::set_var("XDG_RUNTIME_DIR", format!("/run/user/{}", user.uid()));
	std::env::set_current_dir(std::path::Path::new(user.home_dir()))
		.tuilog_err(TUILogError::LoginSessionFailed)?;

	Ok(())
}

fn spawn_shell(user: &User, session_type: u8) -> TUILogResult<()> {
	let shell_path = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::LoginShellFailed)?;
	let c_shell_path = CString::new(shell_path).unwrap();

	let args = match session_type {
		0 => vec![
			c_shell_path,
			CString::new("-l").unwrap(),
			CString::new("-c").unwrap(),
			CString::new(format!(
				"stty sane; tput sgr0; tput cnorm; clear; exec {} -l",
				&shell_path,
			))
			.unwrap(),
		],
		1 => vec![
			c_shell_path,
			CString::new("-l").unwrap(),
			CString::new("-c").unwrap(),
			CString::new("exec startx").unwrap(),
		],
		_ => return Err(TUILogError::InvalidSessionOption),
	};

	execvp(&args[0], &args).tuilog_err(TUILogError::LoginSessionFailed)?;

	Ok(())
}

fn set_process_ids(user: &User) -> TUILogResult<()> {
	let uid = Uid::from_raw(user.uid());
	let gid = Gid::from_raw(user.primary_group_id());

	// Change the process UID and GID to the authenticated user
	match get_current_tty_path() {
		Ok(tty_path) => {
			setsid().tuilog_err(TUILogError::LoginSessionFailed)?;
			chown(&tty_path, Some(uid), Some(gid))
				.tuilog_err(TUILogError::LoginSessionFailed)?;

			// Open the tty
			let tty_fd = open(&tty_path, OFlag::O_RDWR, Mode::empty())
				.tuilog_err(TUILogError::LoginSessionFailed)?;

			// Set it as controlling terminal
			unsafe {
				if libc::ioctl(tty_fd, libc::TIOCSCTTY, 1) < 0 {
					return Err(TUILogError::LoginSessionFailed);
				}
			}

			// Redirect stdin, stdout, stderr to the TTY
			dup2(tty_fd, 0).tuilog_err(TUILogError::LoginSessionFailed)?; // stdin
			dup2(tty_fd, 1).tuilog_err(TUILogError::LoginSessionFailed)?; // stdout
			dup2(tty_fd, 2).tuilog_err(TUILogError::LoginSessionFailed)?; // stderr

			// Optional: close extra tty_fd if it's not 0,1,2
			if tty_fd > 2 {
				let _ = nix::unistd::close(tty_fd);
			}
		}
		Err(_) => {}
	};
	let c_username = CString::new(
		user.name()
			.to_str()
			.tuilog_err(TUILogError::LoginSessionFailed)?,
	)
	.tuilog_err(TUILogError::LoginSessionFailed)?;
	initgroups(&c_username, gid).tuilog_err(TUILogError::LoginSessionFailed)?;
	setgid(gid).tuilog_err(TUILogError::LoginSessionFailed)?;
	setuid(uid).tuilog_err(TUILogError::LoginSessionFailed)?;

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
	let session_id = siv
		.call_on_name("session", |view: &mut SelectView<u8>| {
			match view.selection() {
				Some(selected) => *selected,
				None => 0, // default to shell
			}
		})
		.tuilog_err(TUILogError::InvalidSessionOption)?;

	// has to be before auth_user as the cache file is only accessible to root
	set_default_options(username.to_string(), session_id);
	let (pam_client, user) = auth_user(&username, &password)?;
	siv.quit();

	let proc_type =
		unsafe { fork().tuilog_err(TUILogError::LoginSessionFailed)? };

	match proc_type {
		ForkResult::Parent { child } => {
			waitpid(child, None).tuilog_err(TUILogError::LoginSessionFailed)?;
			drop(pam_client); // Close the PAM session
		}
		ForkResult::Child => {
			set_process_ids(&user)?;
			set_environment(&user)?;
			spawn_shell(&user, session_id)?;
		}
	}

	Ok(())
}

pub fn get_sessions() -> Vec<(String, u8)> {
	let mut sessions = Vec::new();

	sessions.push(("shell".to_string(), 0));

	sessions.push(("startx".to_string(), 1));

	sessions
}
