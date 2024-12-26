pub mod background;
pub mod theme;
pub mod content_box;
pub mod message;
pub mod utils;
pub mod session;
pub mod config;
pub mod error;
pub mod system_control;

use background::draw_background_ascii_art;
use theme::get_base_theme;
use content_box::draw_content_box;
use crate::error::DrawTUILogResult;

fn main() {
	let mut siv = cursive::default();
	siv.set_theme(get_base_theme());

	let draw_background_result = draw_background_ascii_art(&mut siv);

	draw_content_box(&mut siv);

	draw_background_result.draw_on_err(&mut siv);

	siv.run();
}
