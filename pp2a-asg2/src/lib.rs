use std::fmt::Display;

/// This is what holds the allocated data.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WordCollection {
	words: Vec<String>,
}

impl WordCollection {
	/// Create a new instance of the WordCollection
	pub fn new() -> Self {
		Self { words: Vec::new() }
	}
	/// Allocate the max size at once (24 bytes * capacity).
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			words: Vec::with_capacity(capacity),
		}
	}

	pub fn add_word(&mut self, c: &str) {
		let i = self
			.words
			.iter()
			.position(|s| s.as_str() > c)
			.unwrap_or(self.words.len());
		self.words.insert(i, c.to_string());
	}

	pub fn search(&self, c: &str) -> bool {
		self.words.iter().any(|s| s.as_str() == c)
	}

	pub fn to_vec(&self) -> Vec<String> {
		self.words.clone()
	}
}

/// Implementing Display automatically gives you ToString (.to_string())
/// and a bunch of other goodies.
impl Display for WordCollection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for (i, word) in self.words.iter().enumerate() {
			f.write_fmt(format_args!("Element #{}:\t{}\n", i + 1, word))?;
		}
		Ok(())
	}
}

impl Default for WordCollection {
	fn default() -> Self {
		Self::new()
	}
}
