use std::{
	fs::File,
	io::{self, BufRead, BufReader, Write},
};

use clap::Parser;
use log::debug;

fn main() -> std::io::Result<()> {
	env_logger::init();

	let args = jlucktay_grrs::Cli::parse();

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

	// get the global stdout entity, and lock it
	let stdout = io::stdout();
	let mut locked = stdout.lock();

	// write matching lines out to the locked reference
	for line_result in reader.lines() {
		let line = line_result.unwrap_or_default();

		if line.contains(&args.pattern) {
			writeln!(locked, "{}", line)?;
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

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}

#[test]
fn find_a_match() {
	let mut output = Vec::new();
	let _result = jlucktay_grrs::_find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut output);
	assert_eq!(output, b"lorem ipsum\n");
}
