use cursive::Cursive;

use crate::message::draw_error_message;

pub type TUILogResult<T> = Result<T, TUILogError>;

#[derive(Clone)]
pub enum TUILogError {
	AuthenticationFailed,
	Unauthorized,
	UserNotFound,
	LoginShellFailed,
	X11SessionFailed,
	InvalidSessionOption,
	LoginSessionFailed,
	BackgroundArtFailed,
	ShutdownFailed,
	RebootFailed,
	DBUSConnectionFailed,
}

pub trait TUILogErrorMap<T> {
	type Return;
	fn tuilog_err(self, error: TUILogError) -> Self::Return;
}

pub trait DrawTUILogResult<T> {
	fn draw_on_err(self, siv: &mut Cursive);
}

impl<T, E> TUILogErrorMap<T> for Result<T, E> {
	type Return = TUILogResult<T>;
	fn tuilog_err(self, error: TUILogError) -> Self::Return {
		self.map_err(|_| error)
	}
}

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
			TUILogError::X11SessionFailed => "Failed to start xserver.",
			TUILogError::InvalidSessionOption => "Invalid session selected.",
			TUILogError::LoginSessionFailed => "Failed to start session.",
			TUILogError::BackgroundArtFailed => "Failed to draw background art.",
			TUILogError::ShutdownFailed => "Failed to shutdown system.",
			TUILogError::RebootFailed => "Failed to reboot.",
			TUILogError::DBUSConnectionFailed => "Failed to open DBUS connection.",
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
