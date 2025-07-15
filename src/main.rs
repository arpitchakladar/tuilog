pub mod error;
pub mod session;
pub mod state;
pub mod system_control;
pub mod tui;
pub mod utils;

use crate::error::DrawTUILogResult;
use crate::tui::{
	draw_background_ascii_art, draw_content_box, get_base_theme,
	set_default_values,
};

use cursive::view::Resizable;
use cursive::views::StackView;

fn main() {
	let mut siv = cursive::default();
	siv.set_theme(get_base_theme());

	let mut stack = StackView::new();

	let draw_background_result = draw_background_ascii_art(&mut stack);
	draw_content_box(&mut stack);
	draw_background_result.draw_on_err(&mut siv);

	siv.add_fullscreen_layer(stack.full_screen());
	set_default_values(&mut siv);

	siv.run();
}
