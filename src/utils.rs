use unicode_width::UnicodeWidthStr;

pub fn longest_line(input: &str) -> usize {
	input
		.lines()
		.map(|line| UnicodeWidthStr::width(line))
		.max()
		.unwrap_or(0)
}
