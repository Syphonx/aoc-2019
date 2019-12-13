/*
	--- Day 5: Sunny with a Chance of Asteroids ---
*/

use std::convert::TryInto;

#[allow(unused_imports)]
use std::io::{self, BufRead};

const NUM_REG: usize = 4;
static mut REGISTERS: [u32; NUM_REG] = [0; NUM_REG];

struct Digits {
	n: usize,
	divisor: usize,
}

impl Digits {
	fn new(n: usize) -> Self {
		let mut divisor = 1;
		while n >= divisor * 10 {
			divisor *= 10;
		}

		Digits { n, divisor }
	}
}

impl Iterator for Digits {
	type Item = usize;
	fn next(&mut self) -> Option<Self::Item> {
		if self.divisor == 0 {
			None
		} else {
			let v = Some(self.n / self.divisor);
			self.n %= self.divisor;
			self.divisor /= 10;
			v
		}
	}
}

pub fn mem_set(memory: &mut Vec<i32>, index: i32, value: i32) {
	let address = memory[index as usize];
	println!("mem_set({}) = {}", index, value);
	memory[address as usize] = value;
}

pub fn mem_set_addr(memory: &mut Vec<i32>, address: i32, value: i32) {
	println!("mem_set_addr({}) = {}", address, value);
	memory[address as usize] = value;
}

pub fn mem_get(memory: &mut Vec<i32>, index: i32, param: usize) -> i32 {
	let mode = reg_get(param);
	match mode {
		0 => {
			return mem_get_pos(memory, index);
		}
		1 => {
			return mem_get_addr(memory, index);
		}
		_ => {
			return 0;
		}
	}
}

pub fn mem_get_pos(memory: &mut Vec<i32>, address: i32) -> i32 {
	let value = memory[address as usize];
	println!("mem_get_pos({}) |0| = {}", address, memory[value as usize]);
	return memory[value as usize];
}

pub fn mem_get_addr(memory: &mut Vec<i32>, address: i32) -> i32 {
	let value = memory[address as usize];
	println!("mem_get_addr({}) |1| = {}", address, value);
	return value;
}

pub fn mem_get_noprint(memory: &mut Vec<i32>, index: i32, param: usize) -> i32 {
	let mode = reg_get(param);
	let address = memory[index as usize];
	match mode {
		0 => {
			return memory[address as usize];
		}
		1 => {
			return address;
		}
		_ => {
			return 0;
		}
	}
}

pub fn reg_set(index: usize, value: u32) {
	unsafe {
		REGISTERS[index] = value;
	}
}

pub fn reg_get(index: usize) -> u32 {
	unsafe {
		return REGISTERS[index];
	}
}

pub fn reg_clear() {
	for i in 0..NUM_REG {
		reg_set(i, 0);
	}
}

pub fn opcode_add(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!(
		"opcode_add {}+{}={}",
		mem_get_noprint(memory, pc, 0),
		mem_get_noprint(memory, pc + 1, 1),
		mem_get_noprint(memory, pc, 0) + mem_get_noprint(memory, pc + 1, 1)
	);
	let value = mem_get(memory, pc, 0) + mem_get(memory, pc + 1, 1);
	mem_set(memory, pc + 2, value);
	return 4;
}

pub fn opcode_mul(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!(
		"opcode_mul {}*{}={}",
		mem_get_noprint(memory, pc, 0),
		mem_get_noprint(memory, pc + 1, 1),
		mem_get_noprint(memory, pc, 0) * mem_get_noprint(memory, pc + 1, 1)
	);
	let value = mem_get(memory, pc, 0) * mem_get(memory, pc + 1, 1);
	mem_set(memory, pc + 2, value);
	return 4;
}

pub fn opcode_in(memory: &mut Vec<i32>, pc: i32) -> i32 {
	// println!("opcode_in... waiting for input");
	let stdin = io::stdin();
	let line1 = stdin
		.lock()
		.lines()
		.next()
		.unwrap()
		.unwrap()
		.parse::<i32>()
		.expect("Expected integer");
	// let line1 = 1;
	let value = mem_get_addr(memory, pc);
	mem_set_addr(memory, value, line1);
	// println!("opcode_in {} = {}", value, line1);
	return 2;
}

pub fn opcode_out(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!("opcode_out: {} ", mem_get(memory, pc, 0));
	return 2;
}

pub fn opcode_jnz(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!(
		"opcode_jnz ({} != 0, {}) = {}",
		mem_get_noprint(memory, pc, 0),
		mem_get_noprint(memory, pc, 0) != 0,
		mem_get_noprint(memory, pc + 1, 1),
	);
	if mem_get(memory, pc, 0) != 0 {
		return mem_get(memory, pc + 1, 1);
	} else {
		return pc + 2;
	}
}

pub fn opcode_jiz(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!(
		"opcode_jiz ({} == 0, {}) = {}",
		mem_get_noprint(memory, pc, 0),
		mem_get_noprint(memory, pc, 0) == 0,
		mem_get_noprint(memory, pc + 1, 1),
	);
	if mem_get(memory, pc, 0) == 0 {
		return mem_get(memory, pc + 1, 1);
	} else {
		return pc + 2;
	}
}

pub fn opcode_lt(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!(
		"opcode_lt({}) ({} < {}, {})",
		mem_get_noprint(memory, pc + 2, 2),
		mem_get_noprint(memory, pc, 0),
		mem_get_noprint(memory, pc + 1, 1),
		mem_get_noprint(memory, pc, 0) < mem_get_noprint(memory, pc + 1, 1),
	);
	let addr = mem_get_addr(memory, pc + 2);
	if mem_get(memory, pc, 0) < mem_get(memory, pc + 1, 1) {
		mem_set_addr(memory, addr, 1);
	} else {
		mem_set_addr(memory, addr, 0);
	}
	return 4;
}

pub fn opcode_eq(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!(
		"opcode_eq({}) ({} == {}, {})",
		mem_get_noprint(memory, pc + 2, 2),
		mem_get_noprint(memory, pc, 0),
		mem_get_noprint(memory, pc + 1, 1),
		mem_get_noprint(memory, pc, 0) == mem_get_noprint(memory, pc + 1, 1),
	);
	let addr = mem_get_addr(memory, pc + 2);
	if mem_get(memory, pc, 0) == mem_get(memory, pc + 1, 1) {
		mem_set_addr(memory, addr, 1);
	} else {
		mem_set_addr(memory, addr, 0);
	}
	return 4;
}

pub fn opcode_to_str(input: i32) -> &'static str {
	match input {
		1 => "Add",
		2 => "Mul",
		3 => "In",
		4 => "Out",
		99 => "Halt",
		_ => "____",
	}
}

pub fn parse_opcode(instruction: i32) -> i32 {
	let mut opcode = instruction;
	let mut digits: Vec<_> = Digits::new(instruction as usize).collect();
	print!("parse_opcode: '{}' = ", instruction);
	if digits.len() > 1 {
		digits.reverse();
		opcode = (*digits.get(0).expect("Expected i32") as i32)
			+ ((*digits.get(1).expect("Expected i32") as i32) * 10);
		print!("{} ({}) ", opcode, opcode_to_str(opcode));
		for i in 2..digits.len() {
			reg_set(i - 2, digits[i].try_into().unwrap());
			print!("{}, ", digits[i]);
		}
		println!();
	} else {
		print!("{} ({}) ", opcode, opcode_to_str(opcode));
		println!();
	}
	return opcode;
}

pub fn run_intcode(memory: &mut Vec<i32>) -> i32 {
	let mut pc: i32 = 0;
	loop {
		reg_clear();
		let opcode = parse_opcode(memory[pc as usize]);
		match opcode {
			1 => {
				pc += opcode_add(memory, pc + 1);
			}
			2 => {
				pc += opcode_mul(memory, pc + 1);
			}
			3 => {
				pc += opcode_in(memory, pc + 1);
			}
			4 => {
				pc += opcode_out(memory, pc + 1);
			}
			5 => {
				pc = opcode_jnz(memory, pc + 1);
			}
			6 => {
				pc = opcode_jiz(memory, pc + 1);
			}
			7 => {
				pc += opcode_lt(memory, pc + 1);
			}
			8 => {
				pc += opcode_eq(memory, pc + 1);
			}
			99 => {
				break;
			}
			_ => {
				pc += 1;
				println!("Invalid opcode: {}", opcode);
			}
		}
		// print_memory(memory, pc);
		// println!("PC: {}", pc);
	}
	return memory[0];
}

pub fn print_memory(memory: &Vec<i32>, pc: i32) {
	let column = 10;
	let mut current_column = 0;
	let mut row = 0;
	println!("---- REG ----");
	for index in 0..NUM_REG {
		print!("{}\t", reg_get(index));
	}
	println!("");
	println!("---- REG ----");
	println!("---- ROM ----");
	for index in 0..memory.len() {
		if current_column == 0 {
			print!("{}\t| ", row * 10);
		}
		if pc as usize == index {
			print!("*{}*\t", memory[index]);
		} else {
			print!("{}\t", memory[index]);
		}
		current_column += 1;
		if current_column >= column {
			current_column = 0;
			row += 1;
			println!("");
		}
	}
	println!("");
	println!("---- ROM ----");
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i32 {
	let mut memory: Vec<i32> = input
		.split_terminator(",")
		.map(|x| x.trim().parse::<i32>().expect("Expected integer"))
		.collect();
	// print_memory(&memory, 0);
	return run_intcode(&mut memory);
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
	// let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
	// let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
	// let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
	let mut memory: Vec<i32> = input
		.split_terminator(",")
		.map(|x| x.trim().parse::<i32>().expect("Expected integer"))
		.collect();
	// print_memory(&memory, 0);
	return run_intcode(&mut memory);
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn part_1() {
		assert_eq!(solve_part1("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
	}
}