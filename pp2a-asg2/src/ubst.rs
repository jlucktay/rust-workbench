use std::fmt::Display;

use crate::WordCollection;

/// `UnbalancedBinarySearchTree` is an unbalanced binary search tree with binary insert and search.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnbalancedBinarySearchTree {}

impl WordCollection for UnbalancedBinarySearchTree {
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

impl Display for UnbalancedBinarySearchTree {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}
