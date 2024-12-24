use std::path::Path;
use gethostname::gethostname;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Deserialize)]
struct Config {
	title: Option<String>,
	ascii_art: Option<AsciiArt>,
}

#[derive(Deserialize)]
struct AsciiArt {
	background: Option<String>,
	error_icon: Option<String>,
}

lazy_static! {
	static ref base_path: String = std::env::var("TUILOG_CONFIG_DIR")
		.unwrap_or("/etc/tuilog".to_string());

	static ref config: Config = {
		let config_dir = Path::new(&*base_path)
			.join(Path::new("config.toml"))
			.display()
			.to_string();
		let contents = std::fs::read_to_string(config_dir)
			.unwrap_or("".to_string());
		toml::from_str(&contents)
			.unwrap_or(
				Config {
					title: None,
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

	pub static ref background_ascii_art_path: Option<String> = {
		if let Some(ref ascii_art) = config.ascii_art {
			eprintln!("{:?}", ascii_art.background);
			if let Some(ref background) = ascii_art.background {
				let background_path = Path::new(background);

				return Some(
					if background_path.is_absolute() {
						background.to_string()
					} else {
						Path::new(&*base_path)
							.join(background_path)
							.display()
							.to_string()
					}
				);
			}
		}

		None
	};

	pub static ref error_icon_ascii_art_path: Option<String> = {
		if let Some(ref ascii_art) = config.ascii_art {
			if let Some(ref error_icon) = ascii_art.error_icon {
				let error_icon_path = Path::new(error_icon);

				return Some(
					if error_icon_path.is_absolute() {
						error_icon.to_string()
					} else {
						Path::new(&*base_path)
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
