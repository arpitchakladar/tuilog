use std::fs;
use gethostname::gethostname;
use cursive::views::{
	Dialog,
	EditView,
	LinearLayout,
	TextView,
	ThemedView,
	Button,
	PaddedView,
	Canvas,
	Layer
};
use cursive::traits::*;
use cursive::theme::{
	PaletteColor,
	Theme,
	ColorType,
	Color,
	ColorStyle
};
use cursive::{
	Cursive,
	Printer,
	CursiveExt
};

const INPUT_LENGTH: usize = 24;

fn get_edit_view_theme(siv: &Cursive) -> Theme {
	let mut edit_view_theme = siv.current_theme().clone();
	let secondary_color = edit_view_theme.palette[PaletteColor::Secondary];
	edit_view_theme.palette[PaletteColor::Secondary] = edit_view_theme.palette[PaletteColor::Background];
	edit_view_theme.palette[PaletteColor::Background] = secondary_color;
	edit_view_theme.palette[PaletteColor::View] = edit_view_theme.palette[PaletteColor::Primary];
	edit_view_theme
}

fn draw_background(siv: &mut Cursive) {
	let ascii_art = fs::read_to_string("assets/ascii-art.txt")
		.expect("Failed to read ASCII art file. Ensure the file exists and is readable.");

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
					ColorType::Color(Color::parse("#2bbac5").unwrap()),
					ColorType::Color(background_color)
				), |printer| {
					printer.print((start_x, start_y + i), line);
				})
			}
		});

	// Add the ASCII art as the background
	siv.add_layer(Layer::new(ascii_view.full_screen()));
}

fn draw_dialog(siv: &mut Cursive, text: &str) {
	siv.add_layer(
		Dialog::around(
			PaddedView::lrtb(2, 2, 1, 1,
				LinearLayout::vertical()
					.child(TextView::new(text))
					.child(
						PaddedView::lrtb(0, 0, 1, 0,
							Button::new_raw("[OK]", |siv| { siv.pop_layer(); })
						)
					)
			)
		)
		.title("ERROR")
	);
}

fn draw_content_box(siv: &mut Cursive) {
	let edit_view_theme = get_edit_view_theme(&siv);
	// Creates a dialog with a single "Quit" button
	siv.add_layer(
		Dialog::around(
			PaddedView::lrtb(2, 2, 1, 1,
				LinearLayout::vertical()
					.child(LinearLayout::horizontal()
						.child(TextView::new("Username: "))
						.child(ThemedView::new(
							edit_view_theme.clone(),
							EditView::new()
								.filler(" ")
								.on_submit(|siv, _| {
									siv.focus_name("password").ok();
								})
								.with_name("username")
								.fixed_width(INPUT_LENGTH)
							)
						)
					)
					.child(LinearLayout::horizontal()
						.child(TextView::new("Password: "))
						.child(ThemedView::new(
							edit_view_theme,
							EditView::new()
								.secret()
								.filler(" ")
								.on_submit(|siv, _| {
									draw_dialog(siv, "Couldn't log you in login.");
								})
								.with_name("password")
								.fixed_width(INPUT_LENGTH)
						))
					)
					.child(
						PaddedView::lrtb(0, 0, 1, 0,
							Button::new_raw("[LOGIN]", |siv| { siv.quit(); })
						)
					)
			)
		)
		.title(gethostname().into_string().unwrap())
	);
}

fn main() {
	let mut siv = Cursive::new();
	siv.load_toml(include_str!("../assets/theme.toml")).unwrap();

	draw_background(&mut siv);
	draw_content_box(&mut siv);

	siv.run();
}
