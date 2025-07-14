use zbus::blocking::Connection;

use crate::error::{
	TUILogError, TUILogErrorMap, TUILogResult,
};

pub fn shutdown() -> TUILogResult<()> {
	let connection = Connection::system()
		.tuilog_err(TUILogError::DBUSConnectionFailed)?;
	let proxy = zbus::blocking::Proxy::new(
		&connection,
		"org.freedesktop.login1",
		"/org/freedesktop/login1",
		"org.freedesktop.login1.Manager",
	)
	.tuilog_err(TUILogError::DBUSConnectionFailed)?;

	proxy
		.call_method("PowerOff", &(true))
		.tuilog_err(TUILogError::ShutdownFailed)?;

	Ok(())
}

pub fn reboot() -> TUILogResult<()> {
	let connection = Connection::system()
		.tuilog_err(TUILogError::DBUSConnectionFailed)?;
	let proxy = zbus::blocking::Proxy::new(
		&connection,
		"org.freedesktop.login1",
		"/org/freedesktop/login1",
		"org.freedesktop.login1.Manager",
	)
	.tuilog_err(TUILogError::DBUSConnectionFailed)?;

	proxy
		.call_method("Reboot", &(true))
		.tuilog_err(TUILogError::RebootFailed)?;

	Ok(())
}
