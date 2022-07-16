use std::{
	fs::File,
	io::{BufRead, BufReader},
};

use clap::Parser;
use log::debug;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
	/// The pattern to look for
	pattern: String,

	/// The path to the file to read
	#[clap(parse(from_os_str))]
	path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
	env_logger::init();

	let args = Cli::parse();

	// Exercise for the reader: Make this program output its arguments!
	debug!("pattern: '{}'", args.pattern);
	debug!("path:    '{}'", args.path.display());

	/*
	Exercise for the reader:
	This is not the best implementation:
	It will read the whole file into memory – however large the file may be.
	Find a way to optimize it!
	(One idea might be to use a BufReader instead of read_to_string().)
	*/
	let file = File::open(args.path)?;
	let reader = BufReader::new(file);

	for line_result in reader.lines() {
		let line = line_result.unwrap();

		if line.contains(&args.pattern) {
			println!("{}", line);
		}
	}

	Ok(())
}

pub fn answer() -> i32 {
	42
}

#[test]
fn check_answer_validity() {
	assert_eq!(answer(), 42);
}
