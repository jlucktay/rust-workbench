use std::fmt::Display;

use crate::WordCollection;

/// `OrderedLinkedList` is an ordered linked list with linear insert and search.
///
/// Will probably blag the implementation from this
/// [fantastic write-up](https://rust-unofficial.github.io/too-many-lists/).
/// The third one in there seems like the way to go.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedLinkedList {}

impl WordCollection for OrderedLinkedList {
	fn new() -> Self
	where
		Self: Sized,
	{
		todo!()
	}

	fn add(&mut self, _word: &str) {
		todo!()
	}

	fn search(&self, _word: &str) -> bool {
		todo!()
	}

	fn size(&self) -> usize {
		todo!()
	}
}

impl Display for OrderedLinkedList {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
