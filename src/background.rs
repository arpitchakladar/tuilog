use std::fs;
use cursive::traits::*;
use cursive::views::{
	Canvas,
	Layer,
};
use cursive::theme::{
	PaletteColor,
	ColorType,
	Color,
	BaseColor,
	ColorStyle,
};
use cursive::{
	Cursive,
	Printer,
};

pub fn draw_background_ascii_art(siv: &mut Cursive) {
	match fs::read_to_string("assets/background/nixos.txt") {
		Ok(ascii_art) => {
			let background_color = siv.current_theme().palette[PaletteColor::Background];
			// Create a Canvas to render ASCII art
			let ascii_view = Canvas::new(ascii_art.to_string())
				.with_draw(move |ascii_art, printer: &Printer| {
					let lines: Vec<&str> = ascii_art.lines().collect();
					let art_height = lines.len();
					let art_width = lines.iter().map(|line| line.chars().count()).max().unwrap_or(0);

					let screen_height = printer.size.y;
					let screen_width = printer.size.x;

					// Calculate centered position
					let start_y = (screen_height.saturating_sub(art_height)) / 2;
					let start_x = (screen_width.saturating_sub(art_width)) / 2;

					for (i, line) in lines.iter().enumerate() {
						printer.with_color(ColorStyle::new(
							ColorType::Color(Color::Dark(BaseColor::Blue)),
							ColorType::Color(background_color)
						), |printer| {
							printer.print((start_x, start_y + i), line);
						})
					}
				});

			// Add the ASCII art as the background
			siv.add_layer(Layer::new(ascii_view.full_screen()));
		},
		Err(_) => println!("Failed to read ASCII art file. Ensure the file exists and is readable.")
	}
}
