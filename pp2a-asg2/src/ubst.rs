extern crate binary_search_tree;
use binary_search_tree::BinarySearchTree;

use crate::WordCollection;

/// `UnbalancedBinarySearchTree` is an unbalanced binary search tree with binary insert and search.
pub type UnbalancedBinarySearchTree = BinarySearchTree<String>;

impl WordCollection for UnbalancedBinarySearchTree {
	/// Initialises the `UnbalancedBinarySearchTree` implementation of `WordCollection` with no nodes.
	fn new() -> Self {
		unimplemented!()
	}

	/// The word is added so that the `WordCollection` is kept in alphabetical order at all times.
	fn add(&mut self, word: &str) {
		self.insert(word.to_string());
	}

	/// Searches for the word in the `WordCollection`.
	/// This utilises a binary search algorithm.
	fn search(&self, word: &str) -> bool {
		self.contains(&word.to_string())
	}

	/// Returns the number of words in the `WordCollection`.
	fn size(&self) -> usize {
		self.size
	}
}
