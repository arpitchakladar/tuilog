use std::fs;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::{Cursive};

fn main() {
	let mut siv = cursive::default();

	// Creates a dialog with a single "Quit" button
	siv.add_layer(
			Dialog::around(
				LinearLayout::horizontal()
					.child(TextView::new("Username "))
					.child(EditView::new()
						.with_name("username")
						.fixed_width(20)
					)
			)
			.title("TuiLog")
			.button("Quit", |s| s.quit())
		);

	// Starts the event loop.
	siv.run();
}
