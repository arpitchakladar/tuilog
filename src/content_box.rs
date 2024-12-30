use std::cmp::max;
use text_to_ascii_art::to_art;
use cursive::views::{
	Dialog,
	EditView,
	LinearLayout,
	TextView,
	ThemedView,
	Button,
	PaddedView,
	SelectView,
};
use cursive::view::{
	Nameable,
	View,
	Resizable,
};
use cursive::align::HAlign;
use cursive::Cursive;

use crate::session::{
	get_sessions,
	start_session,
};
use crate::utils::longest_line;
use crate::theme::{
	get_edit_view_theme,
	get_hostname_art_theme,
};
use crate::config::title;
use crate::error::DrawTUILogResult;
use crate::system_control::{
	shutdown,
	reboot,
};

const INPUT_LENGTH: usize = 24;

fn draw_input_field<T: View>(
	label: &str,
	left_spacing: usize,
	edit_view: T
) -> impl View {
	PaddedView::lrtb(left_spacing, 0, 0, 0,
		LinearLayout::horizontal()
			.child(TextView::new(format!("{}: [", label)))
			.child(
				ThemedView::new(
					get_edit_view_theme(),
					edit_view
				)
			)
			.child(TextView::new("]"))
	)
}

fn draw_button<T: 'static + Fn(&mut Cursive) + Send + Sync>(
	label: &str,
	callback: T
) -> impl View {
	PaddedView::lrtb(2, 0, 0, 0,
		Button::new_raw(format!("[{}]", label), callback)
	)
}

pub fn draw_content_box(siv: &mut Cursive) {
	let hostname_art =
		match to_art(
			title.to_string(),
			"standard", 0, 1, 0
		) {
			Ok(art) => art,
			Err(_) => title.to_string(),
		};

	let hostname_art_width = longest_line(&hostname_art);
	// NOTE: the number 12 is very sensitive, any change in length of label or
	// text field length must be noted
	let input_combined_length = INPUT_LENGTH + 12;

	let max_width = max(input_combined_length, hostname_art_width);
	// The padded space to the left of each input fields to center it
	let input_left_padding = (max_width - input_combined_length) / 2;

	let mut session_select = SelectView::new()
		.h_align(HAlign::Left);

	let mut session_select_width = 0;

	for (label, session_id) in get_sessions() {
		session_select_width = max(label.len(), session_select_width);
		session_select.add_item(label, session_id);
	}

	// NOTE: the end decorators update this on changing decorators
	session_select_width += 4;

	let session_select_left_padding = (max_width - session_select_width) / 2;
	let session_select_right_padding = max_width - session_select_left_padding - session_select_width;

	siv.add_layer(
		Dialog::around(
			PaddedView::lrtb(2, 2, 1, 1,
				LinearLayout::vertical()
					.child(
						ThemedView::new(
							get_hostname_art_theme(),
							TextView::new(hostname_art)
								.h_align(HAlign::Center),
						)
					)
					.child(
						PaddedView::lrtb(
							session_select_left_padding, session_select_right_padding, 1, 1,
							session_select
								.decorators("< ", " >")
								.autojump()
								.on_submit(|siv, _| {
									siv.focus_name("username").ok();
								})
								.popup()
								.with_name("session"),
						)
					)
					.child(
						draw_input_field(
							"Username",
							input_left_padding,
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
							input_left_padding,
							EditView::new()
								.secret()
								.filler(" ")
								.on_submit(|siv, _| {
									start_session(siv)
										.draw_on_err(siv);
								})
								.with_name("password")
								.fixed_width(INPUT_LENGTH),
						)
					)
					.child(
						// NOTE: The number 36 has been precisely caliberated,
						// anychanges should be noted
						PaddedView::lrtb((max_width - 36)/2 + 1, 0, 1, 0,
							LinearLayout::horizontal()
								.child(
									draw_button(
										"LOGIN",
										|siv: &mut Cursive| {
											start_session(siv)
												.draw_on_err(siv);
										},
									),
								)
								.child(
									draw_button(
										"SHUTDOWN",
										|siv: &mut Cursive| {
											shutdown()
												.draw_on_err(siv);
										},
									),
								)
								.child(
									draw_button(
										"REBOOT",
										|siv: &mut Cursive| {
											reboot()
												.draw_on_err(siv);
										},
									),
								)
						)
					)
			)
		)
	);

	siv.focus_name("username").ok();
}
