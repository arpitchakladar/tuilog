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
	ResizedView,
};
use cursive::traits::*;
use cursive::theme::{
	PaletteColor,
	Theme,
	Color,
	BaseColor,
	BorderStyle,
};
use cursive::align::HAlign;
use cursive::{
	Cursive,
	CursiveExt,
};

mod background;
use background::draw_background_ascii_art;

const INPUT_LENGTH: usize = 24;

fn get_edit_view_theme(siv: &Cursive) -> Theme {
	let mut edit_view_theme = siv.current_theme().clone();
	let secondary_color = edit_view_theme.palette[PaletteColor::Secondary];
	edit_view_theme.palette[PaletteColor::Secondary] = edit_view_theme.palette[PaletteColor::Background];
	edit_view_theme.palette[PaletteColor::Background] = secondary_color;
	edit_view_theme.palette[PaletteColor::View] = edit_view_theme.palette[PaletteColor::Primary];
	edit_view_theme
}

fn get_theme() -> Theme {
	let mut theme = Theme::terminal_default();

	theme.shadow = false;
	theme.borders = BorderStyle::Simple;
	theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
	theme.palette[PaletteColor::Shadow] = Color::Dark(BaseColor::Black);
	theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
	theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::Secondary] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::Tertiary] = Color::Light(BaseColor::White);
	theme.palette[PaletteColor::TitlePrimary] = Color::Dark(BaseColor::Yellow);
	theme.palette[PaletteColor::TitleSecondary] = Color::Dark(BaseColor::Yellow);
	theme.palette[PaletteColor::Highlight] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::HighlightInactive] = Color::Dark(BaseColor::White);
	theme.palette[PaletteColor::HighlightText] = Color::Dark(BaseColor::Black);

	theme
}

fn get_error_theme(siv: &Cursive) -> Theme {
	let mut error_theme = siv.current_theme().clone();
	error_theme.palette[PaletteColor::Primary] = Color::Dark(BaseColor::Red);
	error_theme
}

fn draw_error_dialog(siv: &mut Cursive, text: &str) {
	let ascii_art = fs::read_to_string("assets/logos/error.txt")
		.expect("Failed to read ASCII art file");
	let error_theme = get_error_theme(siv);
	siv.add_layer(
		Dialog::around(
			PaddedView::lrtb(0, 0, 1, 1,
				LinearLayout::vertical()
					.child(
						ThemedView::new(
							error_theme,
							PaddedView::lrtb(0, 0, 0, 1,
								TextView::new(ascii_art)
									.h_align(HAlign::Center)
							)
						)
					)
					.child(
						ResizedView::with_fixed_size(
							(50, 1),
							TextView::new(text)
								.h_align(HAlign::Center)
						)
					)
					.child(
						PaddedView::lrtb(0, 0, 1, 0,
							Button::new_raw("[OK]", |siv| { siv.pop_layer(); })
						)
					)
			)
		)
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
									draw_error_dialog(siv, "Couldn't log you in.");
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
	siv.set_theme(get_theme());

	draw_background_ascii_art(&mut siv);
	draw_content_box(&mut siv);

	siv.run();
}
