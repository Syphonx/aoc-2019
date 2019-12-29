/*
	--- Day 5: Sunny with a Chance of Asteroids ---
*/

use intcode_vm;
use std::io::{self, BufRead};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i64> {
	let memory: Vec<i64> = input
		.split_terminator(",")
		.map(|x| x.trim().parse::<i64>().expect("Expected integer"))
		.collect();
	return memory;
}

pub fn run_program(memory: &Vec<i64>, input_value: i64) -> i64 {
	// Enable logging for the VM
	intcode_vm::enable_logging();
	// Load the program into the VMs memory
	let mut vm = intcode_vm::VM::from_memory(memory);
	// Queue our debug input
	vm.input.push(input_value);
	// Debug: Expected output: 7873292
	let mut result = 0;
	// Loop VM untill a HALT instruction is received
	loop {
		let status = vm.run_intcode();
		match status {
			intcode_vm::Status::WaitForInput => {
				println!("VM - Waiting for input...");
				let stdin = io::stdin();
				let input = stdin
					.lock()
					.lines()
					.next()
					.unwrap()
					.unwrap()
					.parse::<i64>()
					.expect("Expected integer");
				vm.input.push(input);
				vm.process_input();
			}
			intcode_vm::Status::NewOutput => loop {
				if vm.output.is_empty() {
					break;
				}
				result = vm.output.pop().expect("No elements remaining");
				// println!("VM #{} - Output: {}", i, result);
			},
			intcode_vm::Status::Halt => {
				// println!("VM #{} - Has finshed", i);
				break;
			}
		}
	}

	return result;
}

#[aoc(day5, part1)]
pub fn solve_part1(memory: &Vec<i64>) -> i64 {
	return run_program(memory, 1);
}

#[aoc(day5, part2)]
pub fn solve_part2(memory: &Vec<i64>) -> i64 {
	return run_program(memory, 5);
}
