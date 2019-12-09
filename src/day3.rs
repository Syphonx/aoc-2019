/*
	--- Day 3: Crossed Wires ---
*/

use failure::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WireDirection {
	Invalid,
	Up,
	Right,
	Down,
	Left,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
	x: i32,
	y: i32,
}

impl Point {
	const INVALID_POINT: Point = Point { x: 0, y: 0 };

	pub fn new(x: i32, y: i32) -> Self {
		Point { x, y }
	}

	fn manhattan_distance(&self, other: &Point) -> usize {
		((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Line {
	from: Point,
	to: Point,
}

impl Line {
	fn length(&self) -> usize {
		self.from.manhattan_distance(&self.to)
	}

	fn length_to_point(&self, point: &Point) -> usize {
		self.from.manhattan_distance(point)
	}

	fn x_min(&self) -> i32 {
		std::cmp::min(self.from.x, self.to.x)
	}

	fn x_max(&self) -> i32 {
		std::cmp::max(self.from.x, self.to.x)
	}

	fn y_min(&self) -> i32 {
		std::cmp::min(self.from.y, self.to.y)
	}

	fn y_max(&self) -> i32 {
		std::cmp::max(self.from.y, self.to.y)
	}

	fn intersection(&self, other: &Line) -> Option<Point> {
		let f = |l1: &Line, l2: &Line| {
			if (l1.x_min() >= l2.x_min() && l1.x_min() <= l2.x_max())
				&& (l2.y_min() >= l1.y_min() && l2.y_min() <= l1.y_max())
			{
				return Some(Point::new(l1.x_min(), l2.y_min()));
			} else {
				return None;
			}
		};

		let (f1, f2) = (f(self, other), f(other, self));
		let res = f1.or(f2);

		if let Some(p) = res {
			if p != Point::new(0, 0) {
				return res;
			} else {
				return None;
			}
		} else {
			return res;
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CrossingPoint {
	point: Point,
	steps: usize,
}

impl CrossingPoint {
	fn new(point: &Point, steps: usize) -> Self {
		CrossingPoint {
			point: *point,
			steps,
		}
	}
}

pub struct Wire {
	direction: WireDirection,
	length: i32,
	index: i32,
	line: Line,
}

pub fn char_to_direction(input: &char) -> WireDirection {
	match input {
		'U' => WireDirection::Up,
		'D' => WireDirection::Down,
		'L' => WireDirection::Left,
		'R' => WireDirection::Right,
		_ => WireDirection::Invalid,
	}
}

pub fn enum_to_str(input: &WireDirection) -> &'static str {
	match input {
		WireDirection::Up => "Up",
		WireDirection::Down => "Down",
		WireDirection::Left => "Left",
		WireDirection::Right => "Right",
		WireDirection::Invalid => "Invalid",
	}
}

pub fn direction_to_move(direction: &WireDirection, length: i32) -> (i32, i32) {
	match direction {
		WireDirection::Up => (0, length),
		WireDirection::Down => (0, -length),
		WireDirection::Left => (-length, 0),
		WireDirection::Right => (length, 0),
		WireDirection::Invalid => (0, 0),
	}
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Wire> {
	let mut wires: Vec<Wire> = Vec::new();
	let mut marker: (i32, i32) = (0, 0);

	for (index, line) in input.lines().enumerate() {
		for wire in line.trim().split(',') {
			let (dir, len) = wire.trim().split_at(1);
			let direction: WireDirection = char_to_direction(&dir.chars().next().unwrap());
			let length: i32 = len.parse::<i32>().expect("Expected integer");

			let new_wire = Wire {
				direction,
				length,
				index: index as i32,
				line: Line {
					from: Point::new(marker.0, marker.1),
					to: Point::new(
						marker.0 + direction_to_move(&direction, length).0,
						marker.1 + direction_to_move(&direction, length).1,
					),
				},
			};

			marker.0 = new_wire.line.to.x;
			marker.1 = new_wire.line.to.y;

			wires.push(new_wire);
		}

		marker = (0, 0);
	}

	return wires;
}

pub fn print_path(path: &Wire) {
	println!(
		"Wire: {} - ({}, {}) -> ({}, {}) = {} - [{}]",
		enum_to_str(&path.direction),
		path.line.from.x,
		path.line.from.y,
		path.line.to.x,
		path.line.to.y,
		path.length,
		path.index
	);
}

pub fn print_intersection(intersection: &Point, path: &Line, other: &Line) {
	println!(
		"Intersection {:?} between {:?} and {:?}",
		intersection, path, other
	);
}

pub fn collect_intersections<'a, 'b>(
	wire_0: &'a Vec<&Wire>,
	wire_1: &'b Vec<&Wire>,
) -> Result<Vec<CrossingPoint>, Error> {
	// Keep track of the intersections
	let intersections = wire_0
		.iter()
		.enumerate()
		.flat_map(|(i, wire)| {
			wire_1
				.iter()
				.enumerate()
				.filter_map(move |(j, other_wire)| {
					let intersection = wire.line.intersection(&other_wire.line);
					match intersection {
						Some(x) => {
							// Don't match intersections at (0, 0)
							if x != Point::INVALID_POINT {
								wire.line.intersection(&other_wire.line).map(|a| {
									CrossingPoint::new(
										&a,
										wire_0
											.iter()
											.take(i)
											.map(|a| a.line.length())
											.sum::<usize>() + wire.line.length_to_point(&a)
											+ wire_1
												.iter()
												.take(j)
												.map(|a| a.line.length())
												.sum::<usize>() + other_wire.line.length_to_point(&a),
									)
								})
							} else {
								None
							}
						}
						None => None,
					}
				})
		})
		.collect::<Vec<CrossingPoint>>();
	if intersections.is_empty() {
		return Err(failure::format_err!("No crossed wires found."));
	} else {
		return Ok(intersections);
	}
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Wire]) -> i32 {
	// Define our origin point
	let origin = Point::new(0, 0);

	// Split the input into wire 1 and wire 2
	let wire_1: Vec<&Wire> = input.into_iter().filter(|x| x.index == 0).collect();
	let wire_2: Vec<&Wire> = input.into_iter().filter(|x| x.index == 1).collect();

	// Collect intersections
	let intersections: Vec<CrossingPoint> = collect_intersections(&wire_1, &wire_2).unwrap();

	// Parse the intersections and find the closest
	let min_distance = intersections
		.iter()
		.map(|i| (origin.manhattan_distance(&i.point), i))
		.min_by_key(|t| t.0)
		.expect("Could not find a minimum manhattan distance.");

	// Finally, return our value
	return min_distance.0 as i32;
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Wire]) -> i32 {
	// Split the input into wire 1 and wire 2
	let wire_1: Vec<&Wire> = input.into_iter().filter(|x| x.index == 0).collect();
	let wire_2: Vec<&Wire> = input.into_iter().filter(|x| x.index == 1).collect();

	// Collect intersections
	let intersections: Vec<CrossingPoint> = collect_intersections(&wire_1, &wire_2).unwrap();

	// Parse the intersections and find the closest
	let min_distance = intersections
		.iter()
		.map(|i| (i.steps, i.point))
		.min_by_key(|t| t.0)
		.expect("Could not find a minimum number of steps.");

	// Finally, return our value
	return min_distance.0 as i32;
}
