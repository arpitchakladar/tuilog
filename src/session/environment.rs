use nix::fcntl::{open, OFlag};
use nix::libc;
use nix::sys::stat::Mode;
use nix::unistd::{
	chown, close, dup2, initgroups, setgid, setsid, setuid, Gid, Uid,
};
use std::env;
use std::ffi::CString;
use std::path::Path;
use users::os::unix::UserExt;
use users::User;

use crate::error::{TUILogError, TUILogErrorMap, TUILogResult};
use crate::utils::get_current_tty_path;

pub fn set_environment(user: &User) -> TUILogResult<()> {
	let shell_path = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::EnvironmentSetupFailed)?;
	env::set_var("USER", user.name());
	env::set_var("LOGNAME", user.name());
	env::set_var("HOME", user.home_dir());
	env::set_var("SHELL", shell_path);
	env::set_var("TERM", "linux");

	// Set XDG variables if running a desktop session
	env::set_var("XDG_SESSION_TYPE", "tty");
	env::set_var("XDG_RUNTIME_DIR", format!("/run/user/{}", user.uid()));
	env::set_current_dir(Path::new(user.home_dir()))
		.tuilog_err(TUILogError::EnvironmentSetupFailed)?;

	Ok(())
}

pub fn set_process_ids(user: &User) -> TUILogResult<()> {
	let uid = Uid::from_raw(user.uid());
	let gid = Gid::from_raw(user.primary_group_id());

	// Change the process UID and GID to the authenticated user
	match get_current_tty_path() {
		Ok(tty_path) => {
			setsid().tuilog_err(TUILogError::PrivilegeDropFailed)?;
			chown(&tty_path, Some(uid), Some(gid))
				.tuilog_err(TUILogError::PrivilegeDropFailed)?;

			// Open the tty
			let tty_fd = open(&tty_path, OFlag::O_RDWR, Mode::empty())
				.tuilog_err(TUILogError::PrivilegeDropFailed)?;

			// Set it as controlling terminal
			unsafe {
				if libc::ioctl(tty_fd, libc::TIOCSCTTY, 1) < 0 {
					return Err(TUILogError::PrivilegeDropFailed);
				}
			}

			// Redirect stdin, stdout, stderr to the TTY
			dup2(tty_fd, 0)
				.tuilog_err(TUILogError::TerminalInputOutputSetupFailed)?; // stdin
			dup2(tty_fd, 1)
				.tuilog_err(TUILogError::TerminalInputOutputSetupFailed)?; // stdout
			dup2(tty_fd, 2)
				.tuilog_err(TUILogError::TerminalInputOutputSetupFailed)?; // stderr

			// Optional: close extra tty_fd if it's not 0,1,2
			if tty_fd > 2 {
				let _ = close(tty_fd);
			}
		}
		Err(_) => {}
	};
	let c_username = CString::new(
		user.name()
			.to_str()
			.tuilog_err(TUILogError::PrivilegeDropFailed)?,
	)
	.tuilog_err(TUILogError::PrivilegeDropFailed)?;
	initgroups(&c_username, gid)
		.tuilog_err(TUILogError::PrivilegeDropFailed)?;
	setgid(gid).tuilog_err(TUILogError::PrivilegeDropFailed)?;
	setuid(uid).tuilog_err(TUILogError::PrivilegeDropFailed)?;

	Ok(())
}
