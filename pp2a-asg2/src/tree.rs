use crate::WordCollection;

/// `UnbalancedBinarySearchTree` is an unbalanced binary search tree with binary insert and search.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnbalancedBinarySearchTree {}

impl WordCollection for UnbalancedBinarySearchTree {
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
