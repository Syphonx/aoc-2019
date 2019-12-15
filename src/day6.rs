/*
	--- Day 6: Universal Orbit Map ---
*/

use std::collections::HashMap;

type Orbit = (String, String);

#[derive(Debug)]
struct OrbitMap {
	map: HashMap<String, String>,
}

impl OrbitMap {
	fn from_orbits(orbits: Vec<Orbit>) -> OrbitMap {
		let map: HashMap<String, String> = orbits
			.iter()
			.cloned()
			.map(|(primary, satellite)| (satellite, primary))
			.collect();
		return OrbitMap { map };
	}
	fn total_orbits(&self) -> usize {
		self.map
			.keys()
			.map(|object| self.count_orbits_of(object))
			.sum()
	}

	fn count_orbits_of(&self, object: &String) -> usize {
		match self.map.get(object) {
			None => 0,
			Some(center) => 1 + self.count_orbits_of(center),
		}
	}
}

/*
Visually, the above map of orbits looks like this:
		G - H       J - K - L
	   /           /
COM - B - C - D - E - F
			   \
				I
*/
#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
	let orbits: Vec<Orbit> = input
		.lines()
		.map(|line| {
			let orbit: Vec<&str> = line.trim().split(')').collect();
			(
				orbit.get(0).expect("Expected a valid string").to_string(),
				orbit.get(1).expect("Expected a valid string").to_string(),
			)
		})
		.collect();
	let orbit_map = OrbitMap::from_orbits(orbits);
	return orbit_map.total_orbits();
}

#[aoc(day6, part2)]
pub fn solve_part2(_input: &str) -> i32 {
	0
}
