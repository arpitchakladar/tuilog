use cursive::theme::BaseColor;
use gethostname::gethostname;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::path::{Path, PathBuf};

fn default_cache_dir() -> String {
	"/var/cache/tuilog".to_string()
}
fn default_ascii_art_config() -> AsciiArt {
	AsciiArt {
		background: None,
		background_art_color: BaseColor::White,
		error_icon: None,
	}
}
fn default_base_color() -> BaseColor {
	BaseColor::White
}

#[derive(Deserialize)]
struct Config {
	title: Option<String>,
	#[serde(default = "default_cache_dir")]
	cache_dir: String,
	#[serde(default = "default_ascii_art_config")]
	ascii_art: AsciiArt,
	#[serde(default)]
	sessions: Vec<Session>,
}

#[derive(Deserialize)]
struct AsciiArt {
	background: Option<String>,
	#[serde(
		deserialize_with = "deserialize_base_color",
		default = "default_base_color"
	)]
	background_art_color: BaseColor,
	error_icon: Option<String>,
}

#[derive(Deserialize)]
pub struct Session {
	pub name: String,
	pub exec: String,
}

fn deserialize_base_color<'de, D>(
	deserializer: D,
) -> Result<BaseColor, D::Error>
where
	D: serde::Deserializer<'de>,
{
	match String::deserialize(deserializer)?.as_str() {
		"Black" => Ok(BaseColor::Black),
		"Red" => Ok(BaseColor::Red),
		"Green" => Ok(BaseColor::Green),
		"Yellow" => Ok(BaseColor::Yellow),
		"Blue" => Ok(BaseColor::Blue),
		"Magenta" => Ok(BaseColor::Magenta),
		"Cyan" => Ok(BaseColor::Cyan),
		"White" => Ok(BaseColor::White),
		color => Err(serde::de::Error::custom(format!(
			"Invalid color: {}",
			color
		))),
	}
}

lazy_static! {
	static ref base_path: PathBuf =
		PathBuf::from(
			std::env::var("TUILOG_CONFIG_DIR")
				.unwrap_or("/etc/tuilog".to_string())
		);

	static ref config: Config = {
		let config_dir = (*base_path)
			.join(Path::new("config.toml"))
			.display()
			.to_string();
		let contents = std::fs::read_to_string(config_dir)
			.unwrap_or("".to_string());
		toml::from_str(&contents)
			.unwrap_or(
				Config {
					title: None,
					cache_dir: default_cache_dir(),
					ascii_art: default_ascii_art_config(),
					sessions: Vec::new(),
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

	pub static ref cache_dir: PathBuf = {
		PathBuf::from(&config.cache_dir)
	};

	pub static ref sessions: &'static Vec<Session> = {
		&config.sessions
	};

	pub static ref cache_file: PathBuf = (*cache_dir)
		.join("cache.toml");

	pub static ref background_ascii_art_path: Option<PathBuf> = {
		match config.ascii_art.background {
			Some(ref background) => {
				let background_path = Path::new(background);

				Some(
					if background_path.is_absolute() {
						background_path.to_path_buf()
					} else {
						(*base_path)
							.join(background_path)
					}
				)
			},
			None => None,
		}
	};

	pub static ref background_ascii_art_color: BaseColor = {
		config.ascii_art.background_art_color
	};

	pub static ref error_icon_ascii_art_path: Option<PathBuf> = {
		match config.ascii_art.error_icon {
			Some(ref error_icon) => {
				let error_icon_path = Path::new(error_icon);

				Some(
					if error_icon_path.is_absolute() {
						error_icon_path.to_path_buf()
					} else {
						(*base_path)
							.join(error_icon_path)
					}
				)
			},
			None => None,
		}
	};
}
