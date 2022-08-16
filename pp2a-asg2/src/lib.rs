pub trait WordCollection<'a> {
	/// # Errors
	///
	/// The default implementation always fails.
	fn make_collection(_words: &'a [&'a str]) -> Result<Self, WordCollectionCreateError>
	where
		Self: std::marker::Sized,
	{
		Err(WordCollectionCreateError::Failure)
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum WordCollectionCreateError {
	Failure,
}

pub struct OrderedLinearArray<'a> {
	words: &'a [&'a str],
}

impl<'a> WordCollection<'a> for OrderedLinearArray<'a> {
	fn make_collection(words: &'a [&'a str]) -> Result<Self, WordCollectionCreateError> {
		Ok(OrderedLinearArray { words })
	}
}
