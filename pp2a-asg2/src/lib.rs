pub trait WordCollection {
	fn make_collection() -> Self;

	fn add_collection(&mut self, word: &str);

	/// # Errors
	fn search_collection(&self, word: &str) -> Result<(), WordCollectionError>;

	fn size_collection(&self) -> usize;

	fn display_collection();
}

pub enum WordCollectionError {
	Failure,
}

const WC_SIZE: usize = 250_000;

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
	/// Initialises the `OrderedLinearArray` implementation of `WordCollection` and returns it.
	fn make_collection() -> Self {
		Self { words: Vec::new() }
	}

	/// Adds the word to the `WordCollection`.
	/// It returns `WordCollectionError::Failure` if there is a problem.
	/// The word is added so that the `WordCollection` is in alphabetical order.
	fn add_collection(&mut self, word: &str) {
		let i = self
			.words
			.iter()
			.position(|s| s.as_str() > word)
			.unwrap_or(self.words.len());

		self.words.insert(i, word.to_string());
	}

	/// Searches for the word in the `WordCollection`.
	/// This utilises a linear search algorithm.
	fn search_collection(&self, _word: &str) -> Result<(), WordCollectionError> {
		todo!()
	}

	/// Returns the number of words in the `WordCollection`.
	fn size_collection(&self) -> usize {
		todo!()
	}

	/// Prints the contents of the `WordCollection` to standard output.
	fn display_collection() {
		todo!()
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
