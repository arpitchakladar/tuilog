use gethostname::gethostname;
use text_to_ascii_art::to_art;
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

use crate::theme::{
	get_edit_view_theme,
	get_hostname_art_theme,
};
use crate::message::draw_error_message;

const INPUT_LENGTH: usize = 24;

pub fn draw_content_box(siv: &mut Cursive) {
	let edit_view_theme = get_edit_view_theme(&siv);

	let hostname = gethostname().into_string().unwrap();
	let hostname_art = match to_art(hostname.clone(), "standard", 0, 1, 0) {
		Ok(art) => art,
		Err(_) => hostname,
	};

	siv.add_layer(
		Dialog::around(
			PaddedView::lrtb(2, 2, 1, 1,
				LinearLayout::vertical()
					.child(
						ThemedView::new(
							get_hostname_art_theme(siv),
							PaddedView::lrtb(0, 0, 0, 1,
								TextView::new(hostname_art)
							),
						)
					)
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
	);
}

