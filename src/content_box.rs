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
use cursive::view::{
	Nameable,
	View,
	Resizable,
};
use cursive::theme::Theme;
use cursive::Cursive;

use crate::utils::longest_line;
use crate::theme::{
	get_edit_view_theme,
	get_hostname_art_theme,
};
use crate::message::draw_error_message;

const INPUT_LENGTH: usize = 24;

fn draw_input_field<T: View>(label: &str, theme: Theme, left_spacing: usize, edit_view: T) -> PaddedView<LinearLayout> {
	PaddedView::lrtb(left_spacing, 0, 0, 0,
		LinearLayout::horizontal()
			.child(TextView::new(format!("{}: [", label)))
			.child(
				ThemedView::new(
					theme,
					edit_view
				)
			)
			.child(TextView::new("]"))
	)
}

pub fn draw_content_box(siv: &mut Cursive) {
	let edit_view_theme = get_edit_view_theme(&siv);

	let hostname = gethostname().into_string().unwrap();
	let hostname_art = match to_art(hostname.clone(), "standard", 0, 1, 0) {
		Ok(art) => art,
		Err(_) => hostname,
	};

	let hostname_art_width = longest_line(&hostname_art);
	// The padded space to the left of each input fields to center it
	let input_left_space = (hostname_art_width - INPUT_LENGTH - 12) / 2;

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
					.child(
						draw_input_field(
							"Username",
							edit_view_theme.clone(),
							input_left_space,
							EditView::new()
								.filler(" ")
								.on_submit(|siv, _| {
									siv.focus_name("password").ok();
								})
								.with_name("username")
								.fixed_width(INPUT_LENGTH)
						),
					)
					.child(
						draw_input_field(
							"Password",
							edit_view_theme.clone(),
							input_left_space,
							EditView::new()
								.secret()
								.filler(" ")
								.on_submit(|siv, _| {
									draw_error_message(siv, "Could not log you in!");
								})
								.with_name("password")
								.fixed_width(INPUT_LENGTH),
						)
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

