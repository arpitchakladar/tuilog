use pam::Client;

pub fn auth_user(username: &str, password: &str) -> Result<(), String> {
	let mut client = Client::with_password("login")
		.expect("Failed to init PAM client.");
	// Preset the login & password we will use for authentication
	client.conversation_mut().set_credentials(username, password);
	// Actually try to authenticate:
	client.authenticate().expect("Authentication error!");
	// Now that we are authenticated, it's possible to open a sesssion:
	client.open_session().expect("Failed to open a session!");

	Ok(())
}
