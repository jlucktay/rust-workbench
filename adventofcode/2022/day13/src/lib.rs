use {
	ansi_term::{Colour, Style},
	core::fmt::{Debug, Formatter, Result},
	serde::Deserialize,
	std::cmp::{Ord, Ordering, Ordering::Equal, PartialOrd},
};

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Node {
	Number(u64),
	List(Vec<Node>),
}

impl Debug for Node {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			Self::Number(n) => write!(f, "{n}"),
			Self::List(n) => f.debug_list().entries(n).finish(),
		}
	}
}

impl Node {
	fn with_slice<T>(&self, f: impl FnOnce(&[Self]) -> T) -> T {
		match self {
			Self::List(n) => f(&n[..]),
			Self::Number(n) => f(&[Self::Number(*n)]),
		}
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Self::Number(a), Self::Number(b)) => a.partial_cmp(b),
			(l, r) => Some(l.with_slice(|l| {
				r.with_slice(|r| {
					l.iter()
						.zip(r.iter())
						.map(|(aa, bb)| aa.cmp(bb))
						// Return the first ordering that isn't `Equal`...
						.find(|&ord| ord != Equal)
						// ...or compare the lengths.
						.unwrap_or_else(|| l.len().cmp(&r.len()))
				})
			})),
		}
	}
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap_or(Equal)
	}
}

impl Default for Node {
	fn default() -> Self {
		todo!()
	}
}

#[must_use]
pub fn part1(input: &str, verbose: bool) -> usize {
	let mut sum = 0;

	for (i, groups) in input.split("\n\n").enumerate() {
		let i = i + 1;

		let mut nodes = groups
			.lines()
			.map(|line| serde_json::from_str::<Node>(line).unwrap_or_default());
		let l = nodes.next().unwrap_or_default();
		let r = nodes.next().unwrap_or_default();

		let result = l < r;

		if verbose {
			println!("== Pair {i} ==");
			println!("- Compare {l:?} vs {r:?}");

			if result {
				println!("  - {}", Colour::Green.paint("in the right order"));
			} else {
				println!(
					"  - {} {}",
					Style::new().reverse().paint("not"),
					Colour::Red.paint("in the right order")
				);
			}

			println!();
		}

		if result {
			sum += i;
		}
	}

	sum
}

#[must_use]
pub fn part2(input: &str) -> usize {
	let dividers = vec![
		Node::List(vec![Node::Number(2)]),
		Node::List(vec![Node::Number(6)]),
	];

	let mut packets = input
		.lines()
		.filter(|s| !s.is_empty())
		.map(|line| serde_json::from_str::<Node>(line).unwrap_or_default())
		.chain(dividers.iter().cloned())
		.collect::<Vec<_>>();

	packets.sort();

	let decoder_key = dividers
		.iter()
		.map(|d| packets.binary_search(d).unwrap_or_default() + 1)
		.product::<usize>();

	decoder_key
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_works() {
		let result = part1(include_str!("sample-input.txt"), true);
		assert_eq!(result, 13);
	}

	#[test]
	fn part2_works() {
		let result = part2(include_str!("sample-input.txt"));
		assert_eq!(result, 140);
	}
}
