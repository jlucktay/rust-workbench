fn main() -> anyhow::Result<()> {
	let pair = find_pair_whose_sum_is_2020(
		include_str!("input.txt")
			.split('\n')
			.map(str::parse::<i64>)
			.collect::<Result<Vec<_>, _>>()?,
	);

	dbg!(pair);

	Ok(())
}

fn find_pair_whose_sum_is_2020(s: Vec<i64>) -> Option<(i64, i64)> {
	for i in 0..s.len() {
		for j in 0..s.len() {
			if i == j {
				continue;
			}
			if s[i] + s[j] == 2020 {
				return Some((s[i], s[j]));
			}
		}
	}

	None
}
