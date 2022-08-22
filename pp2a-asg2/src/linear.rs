use std::fmt::Display;

use crate::{WordCollection, WC_SIZE};

/// `OrderedLinearArray` is an ordered array with linear insert and search.
///
/// The [description of `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) is as follows:
///
/// > A contiguous growable array type, written as `Vec<T>`, short for 'vector'.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedLinearArray {
	words: Vec<String>,
}

impl WordCollection for OrderedLinearArray {
	/// Initialises the `OrderedLinearArray` implementation of `WordCollection` with a default capacity of `WC_SIZE`.
	fn new() -> Self {
		Self::default()
	}

	/// Adds the word to the `WordCollection`.
	/// The word is added so that the `WordCollection` is kept in alphabetical order at all times.
	fn add(&mut self, word: &str) {
		let i = self
			.words
			.iter()
			.position(|s| s.as_str() > word)
			.unwrap_or(self.words.len());

		self.words.insert(i, word.to_string());
	}

	/// Searches for the word in the `WordCollection`.
	/// This utilises a linear search algorithm.
	fn search(&self, word: &str) -> bool {
		self.words.iter().any(|s| s.as_str() == word)
	}

	/// Returns the number of words in the `WordCollection`.
	fn size(&self) -> usize {
		self.words.len()
	}
}

impl Default for OrderedLinearArray {
	fn default() -> Self {
		Self::make_collection_with_capacity(None)
	}
}

impl OrderedLinearArray {
	fn make_collection_with_capacity(cap: Option<usize>) -> Self {
		Self {
			words: Vec::with_capacity(cap.unwrap_or(WC_SIZE)),
		}
	}
}

/// Implementing Display automatically gives you `.to_string()` and a bunch of other goodies.
impl Display for OrderedLinearArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("[")?;

		let mut words = self.words.iter().peekable();

		while let Some(word) = words.next() {
			f.write_fmt(format_args!("\"{}\"", word))?;

			if words.peek().is_some() {
				f.write_str(", ")?;
			}
		}

		f.write_str("]")
	}
}
