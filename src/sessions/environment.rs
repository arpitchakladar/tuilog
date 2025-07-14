use nix::fcntl::{open, OFlag};
use nix::libc;
use nix::sys::stat::Mode;
use nix::unistd::{
	chown, dup2, initgroups, setgid, setsid, setuid, Gid,
	Uid,
};
use std::ffi::CString;
use users::{os::unix::UserExt, User};

use crate::error::{
	TUILogError, TUILogErrorMap, TUILogResult,
};
use crate::utils::get_current_tty_path;

pub fn set_environment(user: &User) -> TUILogResult<()> {
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
	std::env::set_var(
		"XDG_RUNTIME_DIR",
		format!("/run/user/{}", user.uid()),
	);
	std::env::set_current_dir(std::path::Path::new(
		user.home_dir(),
	))
	.tuilog_err(TUILogError::LoginSessionFailed)?;

	Ok(())
}

pub fn set_process_ids(user: &User) -> TUILogResult<()> {
	let uid = Uid::from_raw(user.uid());
	let gid = Gid::from_raw(user.primary_group_id());

	// Change the process UID and GID to the authenticated user
	match get_current_tty_path() {
		Ok(tty_path) => {
			setsid().tuilog_err(
				TUILogError::LoginSessionFailed,
			)?;
			chown(&tty_path, Some(uid), Some(gid))
				.tuilog_err(
					TUILogError::LoginSessionFailed,
				)?;

			// Open the tty
			let tty_fd = open(
				&tty_path,
				OFlag::O_RDWR,
				Mode::empty(),
			)
			.tuilog_err(TUILogError::LoginSessionFailed)?;

			// Set it as controlling terminal
			unsafe {
				if libc::ioctl(tty_fd, libc::TIOCSCTTY, 1)
					< 0
				{
					return Err(
						TUILogError::LoginSessionFailed,
					);
				}
			}

			// Redirect stdin, stdout, stderr to the TTY
			dup2(tty_fd, 0).tuilog_err(
				TUILogError::LoginSessionFailed,
			)?; // stdin
			dup2(tty_fd, 1).tuilog_err(
				TUILogError::LoginSessionFailed,
			)?; // stdout
			dup2(tty_fd, 2).tuilog_err(
				TUILogError::LoginSessionFailed,
			)?; // stderr

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
	initgroups(&c_username, gid)
		.tuilog_err(TUILogError::LoginSessionFailed)?;
	setgid(gid)
		.tuilog_err(TUILogError::LoginSessionFailed)?;
	setuid(uid)
		.tuilog_err(TUILogError::LoginSessionFailed)?;

	Ok(())
}
