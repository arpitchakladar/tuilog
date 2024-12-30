use std::fs;
use serde::{
	Serialize,
	Deserialize,
};

use crate::config::{
	cache_dir,
	cache_file,
};

#[derive(Serialize, Deserialize)]
pub struct Cache {
	pub default: Option<DefaultOptions>,
}

#[derive(Serialize, Deserialize)]
pub struct DefaultOptions {
	pub username: Option<String>,
	pub session_type: Option<u8>,
}

pub fn set_default_options(username: String, session_type: u8) {
	let cache = Cache {
		default: Some(DefaultOptions {
			username: Some(username),
			session_type: Some(session_type),
		}),
	};

	if let Err(_) = fs::create_dir_all(&*cache_dir) {
		return;
	}

	fs::write(
		&*cache_file,
		toml::to_string(&cache).unwrap(),
	).ok();
}

pub fn get_default_options() -> DefaultOptions {
	let default_options =
		match fs::read_to_string(&*cache_file) {
			Ok(cache_content) => {
				match toml::from_str::<Cache>(&cache_content) {
					Ok(cache) => cache.default,
					Err(_) => None,
				}
			},
			Err(_) => None
		};

	match default_options {
		Some(default_options) => default_options,
		None => DefaultOptions {
			username: None,
			session_type: None,
		},
	}
}
