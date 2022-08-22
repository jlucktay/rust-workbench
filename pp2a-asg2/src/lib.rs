use std::fmt::Display;

/// A collection of words that has various implementations, each of which will be benchmarked against the others.
pub trait WordCollection
where
	Self: Display,
{
	/// The creation of the various implementations of `WordCollection` won't be object safe, so we add this bound on the
	/// `Sized` trait to mark it as explicitly unavailable to trait objects.
	///
	/// Further reading [here](https://doc.rust-lang.org/error-index.html#method-references-the-self-type-in-its-parameters-or-return-type).
	fn new() -> Self
	where
		Self: Sized;

	/// Adds the word to the `WordCollection`.
	/// The word is added so that the `WordCollection` is kept in alphabetical order at all times.
	fn add(&mut self, word: &str);

	/// Searches for the word in the `WordCollection`.
	fn search(&self, word: &str) -> bool;

	/// Returns the number of words in the `WordCollection`.
	fn size(&self) -> usize;
}

/// The default capacity of a new `WordCollection`.
const WC_SIZE: usize = 250_000;

pub mod binary;
pub mod linear;
pub mod linked;
pub mod ubst;
