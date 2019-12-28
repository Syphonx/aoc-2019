/*
	--- Day 5: Sunny with a Chance of Asteroids ---
*/

use intcode;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
	let memory: Vec<i32> = input
		.split_terminator(",")
		.map(|x| x.trim().parse::<i32>().expect("Expected integer"))
		.collect();
	return memory;
}

#[aoc(day5, part1)]
pub fn solve_part1(memory: &Vec<i32>) -> i32 {
	let mut vm = intcode::VM::from_memory(memory);
	vm.set_input(1);
	// 7157989
	return vm.run_intcode();
}

#[aoc(day5, part2)]
pub fn solve_part2(memory: &Vec<i32>) -> i32 {
	let mut vm = intcode::VM::from_memory(memory);
	vm.set_input(5);
	// 7873292
	return vm.run_intcode();
}
