use pam::Client;

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
