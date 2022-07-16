pub fn find_matches(
	content: &str,
	pattern: &str,
	mut writer: impl std::io::Write,
) -> std::io::Result<()> {
	for line in content.lines() {
		if line.contains(pattern) {
			writeln!(writer, "{}", line)?
		}
	}

	Ok(())
}
