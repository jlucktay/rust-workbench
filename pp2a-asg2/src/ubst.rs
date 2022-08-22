extern crate binary_search_tree;
use binary_search_tree::BinarySearchTree;

use crate::WordCollection;

/// `UnbalancedBinarySearchTree` is an unbalanced binary search tree with binary insert and search.
pub type UnbalancedBinarySearchTree = BinarySearchTree<String>;

impl WordCollection for UnbalancedBinarySearchTree {
	fn new() -> Self {
		unimplemented!()
	}

	fn add(&mut self, word: &str) {
		self.insert(word.to_string());
	}

	fn search(&self, word: &str) -> bool {
		self.contains(&word.to_string())
	}

	fn size(&self) -> usize {
		self.size
	}
}
