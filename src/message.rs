use std::fs;
use cursive::views::{
	Dialog,
	LinearLayout,
	TextView,
	ThemedView,
	Button,
	PaddedView,
	ResizedView,
};
use cursive::align::HAlign;
use cursive::Cursive;

use crate::theme::get_error_message_theme;
use crate::config::error_icon_ascii_art_path;

pub fn draw_error_message(siv: &mut Cursive, text: &str) {
	match &*error_icon_ascii_art_path {
		Some(ref path) => match fs::read_to_string(path) {
			Ok(ascii_art) => siv.add_layer(
				Dialog::around(
					PaddedView::lrtb(0, 0, 1, 1,
						LinearLayout::vertical()
							.child(
								ThemedView::new(
									get_error_message_theme(),
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
			),
			Err(_) => eprintln!("Failed to draw error icon."),
		},
		None => {},
	}
}
