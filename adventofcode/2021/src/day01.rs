#[allow(dead_code)] // Give this function a pass.
fn part1(input: &str) -> i32 {
	let v: Vec<&str> = input.split('\n').collect();

	let mut first: i32;
	let mut second: i32 = 0;
	let mut counter: i32 = -1;

	for val in &v {
		first = second;
		second = val.trim().parse().unwrap_or(0);

		if second > first {
			counter += 1;
		}
	}

	counter
}

#[cfg(test)]
mod test {
	#[test]
	fn day01_part1() {
		const REPORT: &str = "199
			200
			208
			210
			200
			207
			240
			269
			260
			263";

		assert_eq!(super::part1(REPORT), 7);
	}

	#[test]
	fn answer_part1() {
		println!("{}", super::part1(include_str!("day01-input.txt")));
	}
}
