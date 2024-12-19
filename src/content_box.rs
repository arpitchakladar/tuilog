use gethostname::gethostname;
use cursive::views::{
	Dialog,
	EditView,
	LinearLayout,
	TextView,
	ThemedView,
	Button,
	PaddedView,
};
use cursive::traits::*;
use cursive::Cursive;

use crate::theme::get_edit_view_theme;
use crate::message::draw_error_message;

const INPUT_LENGTH: usize = 24;

pub fn draw_content_box(siv: &mut Cursive) {
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
									draw_error_message(siv, "Couldn't log you in.");
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

