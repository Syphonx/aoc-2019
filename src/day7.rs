/*
	--- Day 7: Amplification Circuit ---
*/

use intcode;
use std::io::{self, BufRead};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
	// let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"; // 43210 (4,3,2,1,0)
	// let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23, 101,5,23,23,1,24,23,23,4,23,99,0,0"; // 54321 (0,1,2,3,4)
	// let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"; // 65210 (1,0,4,3,2)
	let memory: Vec<i64> = input
		.split_terminator(",")
		.map(|x| x.trim().parse::<i64>().expect("Expected integer"))
		.collect();
	return memory;
}

pub fn run_program(memory: &Vec<i64>) -> i64 {
	// Create 5 VM instances
	let mut machines: Vec<intcode::VM> = Vec::new();
	let num_machines: i32 = 5;

	// Enable logging for the VM
	intcode::enable_logging();

	// Create each VM
	for _i in 0..num_machines {
		machines.push(intcode::VM::from_memory(memory));
	}

	// Create a vector containing all permutations of the input
	let perms = permute::permute(vec![0, 1, 2, 3, 4]);
	let mut result = 0;
	println!("Total # permutations: {}", perms.len());

	// Track the largest permutation index
	let mut largest_perm_index = 0;
	let mut largest_perm_value = 0;

	// For each permutation, run each vm in sequence and log the result
	for (p, perm) in perms.iter().enumerate() {
		// Log the current perm
		// println!("Current perm: {:?}", perm);

		// Reset the memory in the VM between runs,
		for (i, vm) in machines.iter_mut().enumerate() {
			vm.reset(memory);
			vm.input.push(*perm.get(i).expect("Invalid index"));
		}

		// Set the first VM's 2nd input instruction to 0
		machines[0].input.push(0);

		for (i, vm) in machines.iter_mut().enumerate() {
			// println!("Running VM #{}", i);

			// If we're not at the first VM, each VMs 2nd instruction is the output of the previous
			if i > 0 {
				vm.input.push(result);
			}

			// Loop each VM untill a HALT instruction is received
			loop {
				let status = vm.run_intcode();
				match status {
					intcode::Status::WaitForInput => {
						println!("VM #{} - Waiting for input...", i);
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
					intcode::Status::NewOutput => loop {
						if vm.output.is_empty() {
							break;
						}
						result = vm.output.pop().expect("No elements remaining");
						// println!("VM #{} - Output: {}", i, result);
					},
					intcode::Status::Halt => {
						if result > largest_perm_value {
							largest_perm_value = result;
							largest_perm_index = p;
						}
						// println!("VM #{} - Has finshed", i);
						break;
					}
				}
			}
		}
	}

	println!(
		"Largest permutation: #{} {:?}",
		largest_perm_value, perms[largest_perm_index]
	);

	return largest_perm_value;
}

#[aoc(day7, part1)]
pub fn solve_part1(memory: &Vec<i64>) -> i64 {
	return run_program(memory);
}

#[aoc(day7, part2)]
pub fn solve_part2(memory: &Vec<i64>) -> i64 {
	return run_program(memory);
}