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
	StackView,
	stack_view::LayerAt,
};
use cursive::view::{
	Nameable,
	View,
	Resizable,
	Offset,
};
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::XY;

use crate::session::{
	get_sessions,
	start_session,
};
use crate::utils::{
	longest_line_length,
	get_current_tty,
};
use crate::theme::{
	get_accent_message_theme,
	get_edit_view_theme,
	get_hostname_art_theme,
	get_error_message_theme,
};
use crate::config::title;
use crate::error::DrawTUILogResult;
use crate::system_control::{
	shutdown,
	reboot,
};
use crate::cache::get_default_options;

const INPUT_LENGTH: usize = 24;

fn draw_input_field<T: View>(
	label: &str,
	left_spacing: usize,
	edit_view: T
) -> impl View {
	PaddedView::lrtb(left_spacing, 0, 0, 0,
		LinearLayout::horizontal()
			.child(
				ThemedView::new(
					get_accent_message_theme(),
					TextView::new(format!("{}: [", label)),
				),
			)
			.child(
				ThemedView::new(
					get_edit_view_theme(),
					edit_view,
				),
			)
			.child(
				ThemedView::new(
					get_accent_message_theme(),
					TextView::new("]"),
				),
			)
	)
}

fn draw_button<T: 'static + Fn(&mut Cursive) + Send + Sync>(
	label: &str,
	callback: T
) -> impl View {
	PaddedView::lrtb(2, 0, 0, 0,
		ThemedView::new(
			get_accent_message_theme(),
			Button::new_raw(format!("[{}]", label), callback),
		),
	)
}

pub fn draw_content_box(stack: &mut StackView) {
	let hostname_art =
		match to_art(
			title.to_string(),
			"standard", 0, 1, 0
		) {
			Ok(art) => art,
			Err(_) => title.to_string(),
		};

	let hostname_art_width = longest_line_length(&hostname_art);
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

	stack.add_layer(
		LayerAt(
			XY::new(Offset::Absolute(2), Offset::Absolute(1)),
			match get_current_tty() {
				Some(tty) =>
					ThemedView::new(
						get_accent_message_theme(),
						TextView::new(tty),
					),
				None =>
					ThemedView::new(
						get_error_message_theme(),
						TextView::new("no tty"),
					)
			},
		),
	);

	stack.add_layer(
		LayerAt(
			XY::center(),
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
								LinearLayout::horizontal()
									.child(
										ThemedView::new(
											get_accent_message_theme(),
											TextView::new("< "),
										),
									)
									.child(
										session_select
											.decorators("", "")
											.autojump()
											.on_submit(|siv, _| {
												siv.focus_name("username").ok();
											})
											.popup()
											.with_name("session")
									).child(
										ThemedView::new(
											get_accent_message_theme(),
											TextView::new(" >"),
										),
									),
							)
						)
						.child(
							draw_input_field(
								"USERNAME",
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
								"PASSWORD",
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
		)
	);
}

pub fn set_default_values(siv: &mut Cursive) {
	let default_options = get_default_options();
	match default_options.username {
		Some(ref username) => {
			siv.call_on_name(
				"username",
				|view: &mut EditView| {
					view.set_content(username);
				},
			);

			siv.focus_name("password").ok();
		},
		None => {
			siv.focus_name("username").ok();
		},
	};

	if let Some(ref session_type) = default_options.session_type {
		siv.call_on_name(
			"session",
			|view: &mut SelectView<u8>| {
				view.set_selection(*session_type as usize);
			},
		);
	}
}
