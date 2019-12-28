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
	// let input =	"3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
	let memory: Vec<i64> = input
		.split_terminator(",")
		.map(|x| x.trim().parse::<i64>().expect("Expected integer"))
		.collect();
	return memory;
}

pub fn run_program(memory: &Vec<i64>, permutations: Vec<i64>, loopback_mode: bool) -> i64 {
	// Create 5 VM instances
	let mut machines: Vec<intcode::VM> = Vec::new();
	let num_machines: usize = 5;

	// Enable logging for the VM
	intcode::enable_logging();

	// Create each VM
	for _i in 0..num_machines {
		machines.push(intcode::VM::from_memory(memory));
	}

	// Create a vector containing all permutations of the input
	let perms = permute::permute(permutations);
	println!("Total # permutations: {}", perms.len());

	// Track the largest permutation index
	let mut largest_perm_index = 0;
	let mut largest_perm_value = 0;

	// For each permutation, run each vm in sequence and log the result
	for (p, perm) in perms.iter().enumerate() {
		// Log the current perm
		// println!("Current perm: {:?}", perm);

		// Reset the memory in the VM between runs,
		let mut vm_output = 0;
		let mut first_run = true;
		for (i, vm) in machines.iter_mut().enumerate() {
			vm.reset(memory);
			vm.input.push(*perm.get(i).expect("Invalid index"));
		}

		// Set the first VM's 2nd input instruction to 0
		machines[0].input.push(0);

		let mut status = intcode::Status::Halt;
		loop {
			for i in 0..num_machines {
				// println!("vm_index: {}", i);
				let vm = &mut machines[i];
				if first_run {
					if i > 0 {
						// println!("VM #{} (first run) - Pushing input {}", i, vm_output);
						vm.input.push(vm_output);
					}
				} else {
					// println!("VM #{} - Pushing input {}", i, vm_output);
					vm.input.push(vm_output);
				}
				loop {
					status = vm.run_intcode();
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
						intcode::Status::NewOutput => {
							vm_output = vm.output.remove(0);
							// println!("VM #{} - Output: {}", i, vm_output);
							break;
						}
						intcode::Status::Halt => {
							// println!("VM #{} - Has finshed", i);
							break;
						}
					}
				}
			}
			if loopback_mode {
				first_run = false;
				if status == intcode::Status::Halt {
					break;
				}
			} else {
				break;
			}
		}
		if vm_output > largest_perm_value {
			largest_perm_value = vm_output;
			largest_perm_index = p;
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
	return run_program(memory, vec![0, 1, 2, 3, 4], false);
}

#[aoc(day7, part2)]
pub fn solve_part2(memory: &Vec<i64>) -> i64 {
	return run_program(memory, vec![5, 6, 7, 8, 9], true);
}
