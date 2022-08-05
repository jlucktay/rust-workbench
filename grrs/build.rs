#![forbid(unsafe_code)]

use clap::CommandFactory;

#[path = "src/lib.rs"]
mod cli;

fn main() -> std::io::Result<()> {
	let out_dir =
		std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
	let cmd = cli::Cli::command();

	let man = clap_mangen::Man::new(cmd);
	let mut buffer: Vec<u8> = std::vec::Vec::default();
	man.render(&mut buffer)?;

	std::fs::write(out_dir.join("jlucktay-grrs.1"), buffer)?;

	Ok(())
}
