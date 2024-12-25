use cursive::Cursive;

use crate::message::draw_error_message;

pub type TUILogResult<T> = Result<T, TUILogError>;

#[derive(Clone)]
pub enum TUILogError {
	AuthenticationFailed,
	Unauthorized,
	UserNotFound,
	LoginShellFailed,
	LoginSessionFailed,
	BackgroundArtFailed,
}

// Define a trait with the `err` method
pub trait TUILogErrorMap<T> {
	type Return;
	fn tuilog_err(self, error: TUILogError) -> Self::Return;
}

pub trait DrawTUILogResult<T> {
	fn draw_on_err(self, siv: &mut Cursive);
}

// Implement `TUILogError` for `Result<T, E>`
impl<T, E> TUILogErrorMap<T> for Result<T, E> {
	type Return = TUILogResult<T>;
	fn tuilog_err(self, error: TUILogError) -> Self::Return {
		self.map_err(|_| error)
	}
}

// Implement `TUILogError` for `Option<T>`
impl<T> TUILogErrorMap<T> for Option<T> {
	type Return = TUILogResult<T>;
	fn tuilog_err(self, error: TUILogError) -> Self::Return {
		self.ok_or_else(|| error)
	}
}

impl TUILogError {
	pub fn message(self) -> &'static str {
		match self {
			TUILogError::AuthenticationFailed => "Failed to authenticate.",
			TUILogError::Unauthorized => "Invalid username or password.",
			TUILogError::UserNotFound => "No user found with the given username.",
			TUILogError::LoginShellFailed => "Failed to start login shell.",
			TUILogError::LoginSessionFailed => "Failed to start session.",
			TUILogError::BackgroundArtFailed => "Failed to draw background art.",
		}
	}
}

impl<T> DrawTUILogResult<T> for TUILogResult<T> {
	fn draw_on_err(self, siv: &mut Cursive) {
		if let Err(error) = self {
			draw_error_message(siv, error.message());
		}
	}
}
