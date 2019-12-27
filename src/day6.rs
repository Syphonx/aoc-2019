/*
	--- Day 6: Universal Orbit Map ---
*/

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::usize;

struct Grid<T> {
	nodes: Vec<Node<T>>,
}

struct Node<T> {
	data: T,
	edges: Vec<(usize, usize)>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
	node: usize,
	cost: usize,
}

// Manually implement Ord so we get a min-heap instead of a max-heap
impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		other.cost.cmp(&self.cost)
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

type WeightedEdge = (usize, usize, usize);

impl<T> Grid<T> {
	fn new() -> Self {
		Grid { nodes: Vec::new() }
	}

	fn add_node(&mut self, data: T) -> usize {
		let node = Node {
			edges: Vec::new(),
			data,
		};
		self.nodes.push(node);
		return self.nodes.len() - 1;
	}

	fn create_edges<'a, I>(&mut self, iterator: I)
	where
		I: IntoIterator<Item = &'a WeightedEdge>,
	{
		for &(start, end, weight) in iterator.into_iter() {
			self.nodes[start].edges.push((end, weight));
			self.nodes[end].edges.push((start, weight));
		}
	}

	fn find_path(&self, start: usize, end: usize) -> Option<(Vec<usize>, usize)> {
		let mut dist = vec![(usize::MAX, None); self.nodes.len()];

		let mut heap = BinaryHeap::new();
		dist[start] = (0, None);
		heap.push(State {
			node: start,
			cost: 0,
		});

		while let Some(State { node, cost }) = heap.pop() {
			if node == end {
				let mut path = Vec::with_capacity(dist.len() / 2);
				let mut current_dist = dist[end];
				path.push(end);
				while let Some(prev) = current_dist.1 {
					path.push(prev);
					current_dist = dist[prev];
				}
				path.reverse();

				return Some((path, cost));
			}

			if cost > dist[node].0 {
				continue;
			}

			for edge in &self.nodes[node].edges {
				let next = State {
					node: edge.0,
					cost: cost + edge.1,
				};

				if next.cost < dist[next.node].0 {
					dist[next.node] = (next.cost, Some(node));
					heap.push(next);
				}
			}
		}

		return None;
	}
}

type Orbit = (String, String);

#[derive(Debug)]
struct OrbitMap {
	map: HashMap<String, String>,
}

impl OrbitMap {
	fn from_orbits(orbits: &Vec<Orbit>) -> OrbitMap {
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

	fn count_transfers(&self, from: String, to: String) -> usize {
		println!("Travelling from: {}, to: {}", from, to);
		return 0;
	}
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Orbit> {
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
	return orbits;
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
pub fn solve_part1(orbits: &Vec<Orbit>) -> usize {
	return OrbitMap::from_orbits(orbits).total_orbits();
}

/*
Visually, the above map of orbits looks like this:
						  YOU
						 /
		G - H       J - K - L
	   /           /
COM - B - C - D - E - F
			   \
				I - SAN
*/
#[aoc(day6, part2)]
pub fn solve_part2(orbits: &Vec<Orbit>) -> usize {
	let mut grid = Grid::new();
	let (com, b, g, h, c, d, i, san, e, j, k, you, l, f) = (
		grid.add_node("COM"),
		grid.add_node("B"),
		grid.add_node("G"),
		grid.add_node("H"),
		grid.add_node("C"),
		grid.add_node("D"),
		grid.add_node("I"),
		grid.add_node("SAN"),
		grid.add_node("E"),
		grid.add_node("J"),
		grid.add_node("K"),
		grid.add_node("YOU"),
		grid.add_node("L"),
		grid.add_node("F"),
	);

	grid.create_edges(&[
		(com, b, 1),
		(b, g, 1),
		(b, c, 1),
		(g, h, 1),
		(c, d, 1),
		(d, e, 1),
		(d, i, 1),
		(i, san, 1),
		(e, j, 1),
		(e, f, 1),
		(j, k, 1),
		(k, l, 1),
		(k, you, 1),
	]);

	let (path, cost) = grid.find_path(you, san).unwrap();

	print!("{}", grid.nodes[path[0]].data);
	for i in path.iter().skip(1) {
		print!(" -> {}", grid.nodes[*i].data);
	}
	println!("\nCost: {}", cost);

	return OrbitMap::from_orbits(orbits).count_transfers("YOU".to_string(), "SAN".to_string());
}
