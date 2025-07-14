use unicode_width::UnicodeWidthStr;

pub fn longest_line_length(input: &str) -> usize {
	input
		.lines()
		.map(|line| UnicodeWidthStr::width(line))
		.max()
		.unwrap_or(0)
}

// checks for the format tty{number}
fn is_tty(s: &str) -> bool {
	s.starts_with("tty")
		&& s[3..].chars().all(|c| c.is_digit(10))
}

pub fn get_current_tty_path(
) -> std::io::Result<std::path::PathBuf> {
	std::fs::read_link("/proc/self/fd/0")
}

pub fn get_current_tty() -> Option<String> {
	if let Ok(path) = get_current_tty_path() {
		let tty = path
			.file_name()
			.unwrap()
			.to_string_lossy()
			.to_string();

		if is_tty(&tty) {
			return Some(tty);
		}
	}

	None
}
