use clap::Parser;

/// # Errors
///
/// Will return `Err` if `std::io::Write` has a problem.
pub fn _find_matches(
	content: &str,
	pattern: &str,
	mut writer: impl std::io::Write,
) -> std::io::Result<()> {
	for line in content.lines() {
		if line.contains(pattern) {
			writeln!(writer, "{}", line)?;
		}
	}

	Ok(())
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
	/// The pattern to look for
	pub pattern: String,

	/// The path to the file to read
	#[clap(parse(from_os_str))]
	pub path: std::path::PathBuf,
}
