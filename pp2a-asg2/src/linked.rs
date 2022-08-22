use std::{collections::LinkedList, fmt::Display};

use crate::WordCollection;

/// `OrderedLinkedList` is an ordered linked list with linear insert and search.
///
/// This is basically a wrapper around the linked list implementation in the standard library, for better and worse.
/// Jeez, there is some *real* animosity towards linked lists though, even coming from Clippy as well. 😂
///
/// Honourable mention to this
/// [fantastic write-up](https://rust-unofficial.github.io/too-many-lists/).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedLinkedList {
	// The fact that there is a Clippy lint just for using this is hilarious.
	#[allow(clippy::linkedlist)]
	ll: LinkedList<String>,
}

impl WordCollection for OrderedLinkedList {
	/// Initialises the `OrderedLinkedList` implementation of `WordCollection` with no nodes.
	fn new() -> Self {
		Self {
			ll: LinkedList::new(),
		}
	}

	// Adds the string to the `WordCollection`.
	// The string is added so that the `WordCollection` is in alphabetical order at all times.
	fn add(&mut self, word: &str) {
		self.ll.push_front(word.to_string());

		// The extra clone/sort/etc legwork here isn't nearly as bad as it seems.
		// Hey, we're already dealing with linked lists anyway, so how can this even compare in the first place? 🤪
		// cf: https://stackoverflow.com/a/1525419/380599
		let mut temporary: Vec<String> = self.ll.clone().into_iter().collect();
		temporary.sort();
		self.ll = temporary.into_iter().collect();
	}

	// Searches for the word in the `WordCollection`.
	// This utilises a linear search algorithm.
	fn search(&self, word: &str) -> bool {
		self.ll.contains(&word.to_string())
	}

	/// Returns the number of words in the `WordCollection`.
	fn size(&self) -> usize {
		self.ll.len()
	}
}

impl Display for OrderedLinkedList {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("[")?;

		let mut words = self.ll.iter().peekable();

		while let Some(word) = words.next() {
			f.write_fmt(format_args!("\"{}\"", word))?;

			if words.peek().is_some() {
				f.write_str(", ")?;
			}
		}

		f.write_str("]")
	}
}
