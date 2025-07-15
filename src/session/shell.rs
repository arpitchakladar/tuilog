use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;
use users::os::unix::UserExt;
use users::User;

use crate::error::{TUILogError, TUILogErrorMap, TUILogResult};
use crate::session::{set_environment, set_process_ids};
use crate::state::Session;

fn spawn_session(user: &User, session: &Session) -> TUILogResult<()> {
	let shell_path = user
		.shell()
		.to_str()
		.tuilog_err(TUILogError::ShellSessionFailed)?;
	let c_shell_path = CString::new(shell_path).unwrap();

	let args = vec![
		c_shell_path,
		CString::new("-l").unwrap(),
		CString::new("-c").unwrap(),
		CString::new(format!(
			"stty sane; tput sgr0; tput cnorm; clear; {}",
			if session.exec.is_empty() {
				"".to_string()
			} else {
				format!("exec {} -l", session.exec)
			},
		))
		.unwrap(),
	];

	execvp(&args[0], &args).tuilog_err(TUILogError::ShellSessionFailed)?;

	Ok(())
}

pub fn spawn_shell_session(
	user: &User,
	session: &Session,
) -> TUILogResult<()> {
	let proc_type =
		unsafe { fork().tuilog_err(TUILogError::ShellSessionFailed)? };

	match proc_type {
		ForkResult::Parent { child } => {
			waitpid(child, None).tuilog_err(TUILogError::ShellSessionFailed)?;
		}
		ForkResult::Child => {
			set_process_ids(&user)?;
			set_environment(&user)?;
			spawn_session(&user, &session)?;
		}
	};

	Ok(())
}
