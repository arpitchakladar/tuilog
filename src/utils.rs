pub fn longest_line(input: &str) -> Option<&str> {
	input.lines().max_by_key(|line| line.len())
}
