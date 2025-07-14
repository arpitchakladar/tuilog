use indexmap::IndexMap;
use ini::Ini;
use std::fs;

use crate::session::{Session, SessionType};
use crate::state::xsessions_dir;

pub fn add_xsessions(sessions: &mut IndexMap<String, Session>) {
	if let Ok(entries) = fs::read_dir(xsessions_dir.as_path()) {
		for entry in entries.flatten() {
			let path = entry.path();
			if path.extension().map(|s| s == "desktop").unwrap_or(false) {
				if let Ok(conf) = Ini::load_from_file(&path) {
					if let Some(section) = conf.section(Some("Desktop Entry")) {
						let mut name = section
							.get("Name")
							.unwrap_or("")
							.trim()
							.to_string();
						let exec = section
							.get("Exec")
							.unwrap_or("")
							.trim()
							.to_string();
						if !name.is_empty() && !exec.is_empty() {
							name.push_str(" (Xsession)");
							sessions.insert(
								name.clone(),
								Session {
									name,
									exec,
									session_type: SessionType::Xsession,
								},
							);
						}
					}
				}
			}
		}
	}
}
