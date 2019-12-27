/*
	--- Day 6: Universal Orbit Map ---
*/

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::usize;

struct Graph {
	nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
struct Node {
	data: String,
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct WeightedEdge {
	start: usize,
	end: usize,
	weight: usize,
}

impl Graph {
	fn new() -> Self {
		Graph { nodes: Vec::new() }
	}

	fn add_node(&mut self, data: &String) -> usize {
		let node = Node {
			edges: Vec::new(),
			data: data.clone(),
		};
		self.nodes.push(node);
		return self.nodes.len() - 1;
	}

	fn find_node(&self, data: &String) -> Option<usize> {
		for (i, node) in self.nodes.iter().enumerate() {
			if node.data == *data {
				return Some(i);
			}
		}
		return None;
	}

	fn add_unique(&mut self, data: &String) -> usize {
		let index = self.find_node(data);
		match index {
			Some(value) => return value,
			None => {
				return self.add_node(data);
			}
		}
	}

	fn create_edge(&mut self, edge: &WeightedEdge) {
		self.nodes[edge.start].edges.push((edge.end, edge.weight));
		self.nodes[edge.end].edges.push((edge.start, edge.weight));
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

#[derive(Debug, Clone)]
pub struct Orbit {
	primary: String,
	satellite: String,
}

#[derive(Debug)]
pub struct OrbitMap {
	map: HashMap<String, String>,
}

impl OrbitMap {
	fn from_orbits(orbits: &Vec<Orbit>) -> OrbitMap {
		let map: HashMap<String, String> = orbits
			.iter()
			.cloned()
			.map(|orbit| (orbit.satellite, orbit.primary))
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

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Orbit> {
	let orbits: Vec<Orbit> = input
		.lines()
		.map(|line| {
			let orbit: Vec<&str> = line.trim().split(')').collect();
			Orbit {
				primary: orbit.get(0).expect("Expected a valid string").to_string(),
				satellite: orbit.get(1).expect("Expected a valid string").to_string(),
			}
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
	// Build a graph based on the orbital connections
	let mut graph = Graph::new();
	let weight = 1;
	for orbit in orbits {
		let start = graph.add_unique(&orbit.primary);
		let end = graph.add_unique(&orbit.satellite);
		graph.create_edge(&WeightedEdge { start, end, weight });
	}

	// Find YOU & SAN in the graph, then find the shortest path
	let you = graph.find_node(&"YOU".to_string()).unwrap();
	let san = graph.find_node(&"SAN".to_string()).unwrap();
	let (path, cost) = graph.find_path(you, san).unwrap();

	// For debugging.
	//
	// print!("{}", graph.nodes[path[0]].data);
	// for i in path.iter() {
	// 	print!(" -> {}", graph.nodes[*i].data);
	// }
	// println!();

	// Remove the cost of transfer from YOU & SAN
	return cost - 2;
}
