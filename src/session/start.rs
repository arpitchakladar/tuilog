use cursive::{
	views::{EditView, SelectView},
	Cursive,
};
use std::sync::Arc;
use users::{get_user_by_name, User};

use crate::error::{TUILogError, TUILogErrorMap, TUILogResult};
use crate::session::spawn_shell_session;
use crate::state::{sessions, set_default_options, Session};

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
	let session = siv
		.call_on_name(
			"session",
			|view: &mut SelectView<usize>| -> Option<&'static Session> {
				match view.selection() {
					Some(i) => sessions.get(*i),
					None => None,
				}
			},
		)
		.tuilog_err(TUILogError::InvalidSessionOption)?
		.tuilog_err(TUILogError::InvalidSessionOption)?;

	// has to be before auth_user as the cache file is only accessible to root
	set_default_options(username.to_string(), session.name.clone());
	let (pam_client, user) = auth_user(&username, &password)?;
	siv.quit();

	spawn_shell_session(&user, session)?;

	drop(pam_client); // Close the PAM session
	Ok(())
}
