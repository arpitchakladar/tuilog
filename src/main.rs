pub mod background;
pub mod theme;
pub mod content_box;
pub mod message;
pub mod utils;
pub mod session;
pub mod config;

use background::draw_background_ascii_art;
use theme::get_base_theme;
use content_box::draw_content_box;

fn main() {
	let mut siv = cursive::default();
	siv.set_theme(get_base_theme());

	draw_background_ascii_art(&mut siv);
	draw_content_box(&mut siv);

	siv.run();
}
