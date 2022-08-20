use crate::WordCollection;

/// `OrderedLinkedList` is an ordered linked list with linear insert and search.
///
/// Will probably blag the implementation from this
/// [fantastic write-up](https://rust-unofficial.github.io/too-many-lists/).
/// The third one in there seems like the way to go.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedLinkedList {}

impl WordCollection for OrderedLinkedList {
	fn make_collection() -> Self
	where
		Self: Sized,
	{
		todo!()
	}

	fn add_collection(&mut self, _word: &str) {
		todo!()
	}

	fn search_collection(&self, _word: &str) -> bool {
		todo!()
	}

	fn size_collection(&self) -> usize {
		todo!()
	}

	fn display_collection(&self) {
		todo!()
	}
}
