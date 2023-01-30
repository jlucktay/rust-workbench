use day14::{Grid, Polyline};

fn main() {
	for line in include_str!("sample-input.txt").lines() {
		let polyline = Polyline::parse(line);
		println!("{polyline:?}");

		for p in polyline.path_points() {
			println!("{p:?}");
		}
	}

	let grid = Grid::parse(include_str!("sample-input.txt"));
	println!("{grid:?}");
}
