use cursive::theme::{Color, ColorStyle, ColorType, PaletteColor};
use cursive::traits::*;
use cursive::views::{Canvas, Layer, StackView};
use cursive::Printer;
use std::fs;

use crate::error::{TUILogError, TUILogResult};
use crate::state::{background_ascii_art_color, background_ascii_art_path};
use crate::ui::get_base_theme_ref;

pub fn draw_background_ascii_art(stack: &mut StackView) -> TUILogResult<()> {
	match &*background_ascii_art_path {
		Some(ref path) => match fs::read_to_string(path) {
			Ok(ascii_art) => {
				let background_color =
					get_base_theme_ref().palette[PaletteColor::Background];
				// Create a Canvas to render ASCII art
				let ascii_view = Canvas::new(ascii_art.to_string()).with_draw(
					move |ascii_art, printer: &Printer| {
						let lines: Vec<&str> = ascii_art.lines().collect();
						let art_height = lines.len();
						let art_width = lines
							.iter()
							.map(|line| line.chars().count())
							.max()
							.unwrap_or(0);

						let screen_height = printer.size.y;
						let screen_width = printer.size.x;

						// Calculate centered position
						let start_y =
							(screen_height.saturating_sub(art_height)) / 2;
						let start_x =
							(screen_width.saturating_sub(art_width)) / 2;

						for (i, line) in lines.iter().enumerate() {
							printer.with_color(
								ColorStyle::new(
									ColorType::Color(Color::Dark(
										background_ascii_art_color.clone(),
									)),
									ColorType::Color(background_color),
								),
								|printer| {
									printer.print((start_x, start_y + i), line);
								},
							)
						}
					},
				);

				// Add the ASCII art as the background
				stack
					.add_fullscreen_layer(Layer::new(ascii_view.full_screen()));
			}
			// TODO: Give better error message
			Err(_) => return Err(TUILogError::BackgroundArtFailed),
		},
		None => {}
	};

	Ok(())
}
