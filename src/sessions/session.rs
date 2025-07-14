use indexmap::IndexMap;
use lazy_static::lazy_static;

use crate::sessions::add_xsessions;

#[derive(Clone)]
pub enum SessionType {
	DefaultShell,
	Xorg,
}

#[derive(Clone)]
pub struct Session {
	pub name: String,
	pub exec: String,
	pub session_type: SessionType,
}

lazy_static! {
	pub static ref sessions: IndexMap<String, Session> = {
		let mut cur_sessions = IndexMap::new();
		cur_sessions.insert(
			"shell".to_string(),
			Session {
				name: "shell".to_string(),
				exec: "".to_string(),
				session_type: SessionType::DefaultShell,
			},
		);
		add_xsessions(&mut cur_sessions);
		cur_sessions
	};
}
