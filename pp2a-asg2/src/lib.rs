pub trait WordCollection {
	/// The creation of the various implementations of `WordCollection` won't be object safe, so we add this bound on the
	/// `Sized` trait to mark it as explicitly unavailable to trait objects.
	///
	/// Further reading [here](https://doc.rust-lang.org/error-index.html#method-references-the-self-type-in-its-parameters-or-return-type).
	fn make_collection() -> Self
	where
		Self: Sized;

	fn add_collection(&mut self, word: &str);

	fn search_collection(&self, word: &str) -> bool;

	fn size_collection(&self) -> usize;

	fn display_collection(&self);
}

const WC_SIZE: usize = 250_000;

pub mod ord_array_linear;
