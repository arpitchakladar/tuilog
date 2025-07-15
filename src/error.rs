use cursive::Cursive;

use crate::tui::draw_error_message;

pub type TUILogResult<T> = Result<T, TUILogError>;

#[derive(Clone)]
pub enum TUILogError {
	AuthenticationFailed,
	Unauthorized,
	UserNotFound,
	InvalidSessionOption,
	ShellSessionFailed,
	ShellInputOutputSetupFailed,
	BackgroundArtFailed,
	PrivilegeDropFailed,
	EnvironmentSetupFailed,
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
			TUILogError::UserNotFound => {
				"No user found with the given username."
			}
			TUILogError::ShellSessionFailed => {
				"Failed to start terminal session."
			}
			TUILogError::ShellInputOutputSetupFailed => {
				"Failed to redirect standard input and output to terminal."
			}
			TUILogError::InvalidSessionOption => "Invalid session selected.",
			TUILogError::PrivilegeDropFailed => {
				"Failed to drop user priviledges for session."
			}
			TUILogError::EnvironmentSetupFailed => {
				"Failed to setup user environment for session."
			}
			TUILogError::BackgroundArtFailed => {
				"Failed to draw background art."
			}
			TUILogError::ShutdownFailed => "Failed to shutdown system.",
			TUILogError::RebootFailed => "Failed to reboot.",
			TUILogError::DBUSConnectionFailed => {
				"Failed to open DBUS connection."
			}
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
