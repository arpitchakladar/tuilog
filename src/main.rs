use cursive::views::{
	Dialog,
	EditView,
	LinearLayout,
	TextView,
	ThemedView,
	Button,
	PaddedView
};
use cursive::traits::*;
use cursive::theme::{PaletteColor, Theme};
use cursive::{Cursive, CursiveExt};

fn get_edit_view_theme(siv: &Cursive) -> Theme {
	let mut edit_view_theme = siv.current_theme().clone();
	let secondary_color = edit_view_theme.palette[PaletteColor::Secondary];
	edit_view_theme.palette[PaletteColor::Secondary] = edit_view_theme.palette[PaletteColor::Background];
	edit_view_theme.palette[PaletteColor::Background] = secondary_color;
	edit_view_theme.palette[PaletteColor::View] = edit_view_theme.palette[PaletteColor::Primary];
	edit_view_theme.palette[PaletteColor::Highlight] = edit_view_theme.palette[PaletteColor::Background];
	edit_view_theme.palette[PaletteColor::HighlightText] = edit_view_theme.palette[PaletteColor::Background];
	edit_view_theme.palette[PaletteColor::HighlightInactive] = edit_view_theme.palette[PaletteColor::Background];
	edit_view_theme
}

fn main() {
	let mut siv = Cursive::new();
	siv.load_toml(include_str!("../assets/theme.toml")).unwrap();

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
									.with_name("username")
									.fixed_width(20)
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
									.with_name("password")
									.fixed_width(20)
							))
						)
						.child(Button::new_raw("[LOGIN]", |s| s.quit())
					)
				)
			)
			.title("TuiLog")
		);

	siv.run();
}
