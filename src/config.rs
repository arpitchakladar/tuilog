use std::path::{
	Path,
	PathBuf,
};
use gethostname::gethostname;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Deserialize)]
struct Config {
	title: Option<String>,
	base: Option<String>,
	ascii_art: Option<AsciiArt>,
}

#[derive(Deserialize)]
struct AsciiArt {
	background: Option<String>,
	error_icon: Option<String>,
}

lazy_static! {
	static ref config: Config = {
		let config_dir = std::env::var("TUILOG_CONFIG_DIR")
			.unwrap_or("/etc/tuilog".to_string());
		let contents = std::fs::read_to_string(config_dir)
			.unwrap_or("".to_string());
		toml::from_str(&contents)
			.unwrap_or(
				Config {
					title: None,
					base: None,
					ascii_art: None,
				}
			)
	};

	/* TODO: Add the edge case when the title
	isn't given and hostname can't be found */
	pub static ref title: String = {
		match config.title {
			Some(ref login_title) => login_title.to_string(),
			None => gethostname().into_string().unwrap(),
		}
	};

	static ref base_path: PathBuf = {
		PathBuf::from(
			match config.base {
				Some(ref base) => base.as_str(),
				None => "/etc/nixos",
			}
		)
	};

	pub static ref background_ascii_art: Option<String> = {
		if let Some(ref ascii_art) = config.ascii_art {
			if let Some(ref background) = ascii_art.background {
				let background_path = Path::new(background);

				return Some(
					if background_path.is_absolute() {
						background.to_string()
					} else {
						base_path
							.join(background_path)
							.display()
							.to_string()
					}
				);
			}
		}

		None
	};

	pub static ref error_icon_ascii_art: Option<String> = {
		if let Some(ref ascii_art) = config.ascii_art {
			if let Some(ref error_icon) = ascii_art.error_icon {
				let error_icon_path = Path::new(error_icon);

				return Some(
					if error_icon_path.is_absolute() {
						error_icon.to_string()
					} else {
						base_path
							.join(error_icon_path)
							.display()
							.to_string()
					}
				);
			}
		}

		None
	};
}
