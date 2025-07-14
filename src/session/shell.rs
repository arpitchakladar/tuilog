use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;
use users::{os::unix::UserExt, User};

use crate::error::{TUILogError, TUILogErrorMap, TUILogResult};
use crate::session::{set_environment, set_process_ids, Session};

fn spawn_session(user: &User, session: &Session) -> TUILogResult<()> {
	let shell_path = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::LoginShellFailed)?;
	let c_shell_path = CString::new(shell_path).unwrap();

	let args = vec![
		c_shell_path,
		CString::new("-l").unwrap(),
		CString::new("-c").unwrap(),
		CString::new(format!(
			"stty sane; tput sgr0; tput cnorm; clear; exec {} -l",
			&session.exec,
		))
		.unwrap(),
	];

	execvp(&args[0], &args).tuilog_err(TUILogError::LoginSessionFailed)?;

	Ok(())
}

pub fn spawn_shell_session(user: &User, session: Session) -> TUILogResult<()> {
	let proc_type =
		unsafe { fork().tuilog_err(TUILogError::LoginSessionFailed)? };

	match proc_type {
		ForkResult::Parent { child } => {
			waitpid(child, None).tuilog_err(TUILogError::LoginSessionFailed)?;
		}
		ForkResult::Child => {
			set_process_ids(&user)?;
			set_environment(&user)?;
			spawn_session(&user, &session)?;
		}
	};

	Ok(())
}

pub fn spawn_default_shell_session(
	user: &User,
	mut session: Session,
) -> TUILogResult<()> {
	session.exec = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::LoginShellFailed)?
		.to_string();

	spawn_shell_session(user, session)
}
