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
use cursive::align::HAlign;
use cursive::Cursive;

use crate::session::start_session;
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
	// The padded space to the left of each input fields to center it
	let input_left_space =
		if input_combined_length > hostname_art_width { 0 }
		else { (hostname_art_width - input_combined_length) / 2 };

	siv.add_layer(
		Dialog::around(
			PaddedView::lrtb(2, 2, 1, 1,
				LinearLayout::vertical()
					.child(
						ThemedView::new(
							get_hostname_art_theme(),
							PaddedView::lrtb(0, 0, 0, 1,
								TextView::new(hostname_art)
									.h_align(HAlign::Center)
							),
						)
					)
					.child(
						draw_input_field(
							"Username",
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
							input_left_space,
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
						PaddedView::lrtb((hostname_art_width - 36)/2, 0, 1, 0,
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
}
