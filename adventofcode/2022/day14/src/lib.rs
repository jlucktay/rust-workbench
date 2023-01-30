#![feature(drain_filter)]
#![feature(generators)]
#![feature(iter_from_generator)]

use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::ImageData;

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, derive_more::Add, derive_more::AddAssign, derive_more::Sub,
)]
pub struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn parse(s: &str) -> Self {
		let mut tokens = s.split(',');
		let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());

		Self {
			x: x.parse().unwrap(),
			y: y.parse().unwrap(),
		}
	}

	const fn signum(self) -> Self {
		Self {
			x: self.x.signum(),
			y: self.y.signum(),
		}
	}
}

#[derive(Debug)]
pub struct Polyline {
	points: Vec<Point>,
}

impl Polyline {
	pub fn parse(s: &str) -> Self {
		Self {
			points: s.split(" -> ").map(Point::parse).collect(),
		}
	}

	/// # Panics
	pub fn path_points(&self) -> impl Iterator<Item = Point> + '_ {
		std::iter::from_generator(|| {
			let mut points = self.points.iter().copied();
			let Some(mut a) = points.next() else {return};
			yield a;

			loop {
				let Some(b) = points.next() else {return};
				let delta = (b - a).signum();
				assert!((delta.x == 0) ^ (delta.y == 0));

				loop {
					a += delta;
					yield a;

					if a == b {
						break;
					}
				}
			}
		})
	}
}

#[derive(Debug, Clone, Copy)]
enum Cell {
	Air,
	Rock,
	Sand,
}

#[wasm_bindgen]
pub struct Grid {
	origin: Point,
	width: usize,
	height: usize,
	cells: Vec<Cell>,
	settled: usize,
	grains: Vec<Point>,
}

#[wasm_bindgen]
impl Grid {
	#[wasm_bindgen(constructor)]
	#[allow(clippy::new_without_default)]
	#[must_use]
	pub fn new() -> Self {
		Self::parse(include_str!("input.txt"))
	}

	/// # Panics
	pub fn render(&self, canvas_id: &str) {
		let document = web_sys::window().unwrap().document().unwrap();
		let canvas = document.get_element_by_id(canvas_id).unwrap();
		let canvas: web_sys::HtmlCanvasElement = canvas
			.dyn_into::<web_sys::HtmlCanvasElement>()
			.map_err(|_| ())
			.unwrap();

		canvas.set_width(self.width as _);
		canvas.set_height(self.height as _);

		let context = canvas
			.get_context("2d")
			.unwrap()
			.unwrap()
			.dyn_into::<web_sys::CanvasRenderingContext2d>()
			.unwrap();

		let mut pixels = vec![0u8; 4 * self.width * self.height];
		#[allow(clippy::identity_op)]
		for y in 0..self.height {
			for x in 0..self.width {
				let base_index = 4 * (y * self.width + x);
				pixels[base_index + 0] = 255;
				pixels[base_index + 1] = 20;
				pixels[base_index + 2] = 20;
				pixels[base_index + 3] = 255;
			}
		}

		let air_color: [u8; 3] = [77, 180, 227];
		let rock_color: [u8; 3] = [51, 48, 45];
		let sand_color: [u8; 3] = [130, 127, 88];
		let current_color: [u8; 3] = [245, 206, 49];

		for y in 0..self.height {
			for x in 0..self.width {
				let point = Point {
					x: x as _,
					y: y as _,
				} + self.origin;

				let cell = self.cell(point).unwrap();

				let color = match cell {
					Cell::Air => &air_color,
					Cell::Rock => &rock_color,
					Cell::Sand => &sand_color,
				};

				let base_index = 4 * (y * self.width + x);
				pixels[base_index] = color[0];
				pixels[base_index + 1] = color[1];
				pixels[base_index + 2] = color[2];
			}
		}

		for grain in self.grains.iter().copied() {
			let Point { x, y } = grain - self.origin;

			let color = &current_color;
			let base_index = 4 * (y as usize * self.width + x as usize);
			pixels[base_index] = color[0];
			pixels[base_index + 1] = color[1];
			pixels[base_index + 2] = color[2];
		}

		let imagedata =
			ImageData::new_with_u8_clamped_array(Clamped(&pixels[..]), self.width as _).unwrap();
		context.put_image_data(&imagedata, 0.0, 0.0).unwrap();
	}

	/// # Panics
	pub fn parse(input: &str) -> Self {
		let mut polylines: Vec<_> = input.lines().map(Polyline::parse).collect();

		let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

		for point in polylines
			.iter()
			.flat_map(|p| p.points.iter())
			.chain(std::iter::once(&SPAWN_POINT))
		{
			min_x = min_x.min(point.x);
			min_y = min_y.min(point.y);
			max_x = max_x.max(point.x);
			max_y = max_y.max(point.y);
		}

		let floor_y = max_y + 2;
		let min_x = 300;
		let max_x = 700;
		max_y = floor_y;

		polylines.push(Polyline {
			points: vec![
				Point {
					x: min_x,
					y: floor_y,
				},
				Point {
					x: max_x,
					y: floor_y,
				},
			],
		});

		dbg!(min_x, max_x);
		dbg!(min_y, max_y);

		let origin = Point { x: min_x, y: min_y };
		let width: usize = (max_x - min_x + 1).try_into().unwrap();
		let height: usize = (max_y - min_y + 1).try_into().unwrap();

		dbg!(origin, width, height);

		let mut grid = Self {
			origin,
			width,
			height,
			cells: vec![Cell::Air; width * height],
			settled: 0,
			grains: vec![],
		};

		for point in polylines.iter().flat_map(Polyline::path_points) {
			*grid.cell_mut(point).unwrap() = Cell::Rock;
		}

		grid
	}

	fn cell_index(&self, point: Point) -> Option<usize> {
		let Point { x, y } = point - self.origin;

		let x: usize = x.try_into().ok()?;
		let y: usize = y.try_into().ok()?;

		if x < self.width && y < self.height {
			Some(y * self.width + x)
		} else {
			None
		}
	}

	fn cell(&self, point: Point) -> Option<&Cell> {
		Some(&self.cells[self.cell_index(point)?])
	}

	fn cell_mut(&mut self, point: Point) -> Option<&mut Cell> {
		let cell_index = self.cell_index(point)?;
		Some(&mut self.cells[cell_index])
	}

	#[wasm_bindgen]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn num_settled(&self) -> usize {
		self.settled
	}

	/// # Panics
	#[wasm_bindgen]
	pub fn step(&mut self) {
		if matches!(self.cell(SPAWN_POINT).unwrap(), Cell::Sand) {
			return;
		}

		let mut grains = std::mem::take(&mut self.grains);
		let _ = grains
			.drain_filter(|grain| {
				let straight_down = *grain + Point { x: 0, y: 1 };
				let down_left = *grain + Point { x: -1, y: 1 };
				let down_right = *grain + Point { x: 1, y: 1 };
				let options = [straight_down, down_left, down_right];

				if let Some(pos) = options
					.into_iter()
					.find(|pos| matches!(self.cell(*pos), Some(Cell::Air)))
				{
					*grain = pos;
					return false;
				}

				if options.into_iter().any(|pos| self.cell(pos).is_none()) {
					return true;
				}

				self.settled += 1;
				*self.cell_mut(*grain).unwrap() = Cell::Sand;
				true
			})
			.count();
		self.grains = grains;
		self.grains.push(SPAWN_POINT);
	}
}

impl std::fmt::Debug for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..self.height {
			for x in 0..self.width {
				let point = Point {
					x: x as _,
					y: y as _,
				} + self.origin;

				let cell = self.cell(point).unwrap();

				let c = match cell {
					Cell::Air => '.',
					Cell::Rock => '#',
					Cell::Sand => 'o',
				};

				write!(f, "{c}")?;
			}

			writeln!(f)?;
		}

		Ok(())
	}
}

const SPAWN_POINT: Point = Point { x: 500, y: 0 };
