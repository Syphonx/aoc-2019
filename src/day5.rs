/*
	--- Day 5: Sunny with a Chance of Asteroids ---
*/

use std::convert::TryInto;

#[allow(unused_imports)]
use std::io::{self, BufRead};

const ENABLE_USER_INPUT: bool = false;
const DEBUG_LOG: bool = false;
const NUM_REG: usize = 4;

static mut DEBUG_INPUT: i32 = 1;
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
	if DEBUG_LOG {
		println!("mem_set({}) = {}", index, value);
	}
	memory[address as usize] = value;
}

pub fn mem_set_addr(memory: &mut Vec<i32>, address: i32, value: i32) {
	if DEBUG_LOG {
		println!("mem_set_addr({}) = {}", address, value);
	}
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
	if DEBUG_LOG {
		println!("mem_get_pos({}) |0| = {}", address, memory[value as usize]);
	}
	return memory[value as usize];
}

pub fn mem_get_addr(memory: &mut Vec<i32>, address: i32) -> i32 {
	let value = memory[address as usize];
	if DEBUG_LOG {
		println!("mem_get_addr({}) |1| = {}", address, value);
	}
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
	if DEBUG_LOG {
		println!(
			"opcode_add {}+{}={}",
			mem_get_noprint(memory, pc, 0),
			mem_get_noprint(memory, pc + 1, 1),
			mem_get_noprint(memory, pc, 0) + mem_get_noprint(memory, pc + 1, 1)
		);
	}
	let value = mem_get(memory, pc, 0) + mem_get(memory, pc + 1, 1);
	mem_set(memory, pc + 2, value);
	return 4;
}

pub fn opcode_mul(memory: &mut Vec<i32>, pc: i32) -> i32 {
	if DEBUG_LOG {
		println!(
			"opcode_mul {}*{}={}",
			mem_get_noprint(memory, pc, 0),
			mem_get_noprint(memory, pc + 1, 1),
			mem_get_noprint(memory, pc, 0) * mem_get_noprint(memory, pc + 1, 1)
		);
	}
	let value = mem_get(memory, pc, 0) * mem_get(memory, pc + 1, 1);
	mem_set(memory, pc + 2, value);
	return 4;
}

pub fn opcode_in(memory: &mut Vec<i32>, pc: i32) -> i32 {
	if DEBUG_LOG {
		println!("opcode_in... waiting for input");
	}
	let mut input = 0;
	if ENABLE_USER_INPUT {
		let stdin = io::stdin();
		input = stdin
			.lock()
			.lines()
			.next()
			.unwrap()
			.unwrap()
			.parse::<i32>()
			.expect("Expected integer");
	} else {
		unsafe {
			input = DEBUG_INPUT;
		}
	}
	let value = mem_get_addr(memory, pc);
	mem_set_addr(memory, value, input);
	if DEBUG_LOG {
		println!("opcode_in {} = {}", value, input);
	}
	return 2;
}

pub fn opcode_out(memory: &mut Vec<i32>, pc: i32) -> i32 {
	println!("opcode_out: {} ", mem_get(memory, pc, 0));
	return 2;
}

pub fn opcode_jnz(memory: &mut Vec<i32>, pc: i32) -> i32 {
	if DEBUG_LOG {
		println!(
			"opcode_jnz ({} != 0, {}) = {}",
			mem_get_noprint(memory, pc, 0),
			mem_get_noprint(memory, pc, 0) != 0,
			mem_get_noprint(memory, pc + 1, 1),
		);
	}
	if mem_get(memory, pc, 0) != 0 {
		return mem_get(memory, pc + 1, 1);
	} else {
		return pc + 2;
	}
}

pub fn opcode_jiz(memory: &mut Vec<i32>, pc: i32) -> i32 {
	if DEBUG_LOG {
		println!(
			"opcode_jiz ({} == 0, {}) = {}",
			mem_get_noprint(memory, pc, 0),
			mem_get_noprint(memory, pc, 0) == 0,
			mem_get_noprint(memory, pc + 1, 1),
		);
	}
	if mem_get(memory, pc, 0) == 0 {
		return mem_get(memory, pc + 1, 1);
	} else {
		return pc + 2;
	}
}

pub fn opcode_lt(memory: &mut Vec<i32>, pc: i32) -> i32 {
	if DEBUG_LOG {
		println!(
			"opcode_lt({}) ({} < {}, {})",
			mem_get_noprint(memory, pc + 2, 2),
			mem_get_noprint(memory, pc, 0),
			mem_get_noprint(memory, pc + 1, 1),
			mem_get_noprint(memory, pc, 0) < mem_get_noprint(memory, pc + 1, 1),
		);
	}
	let addr = mem_get_addr(memory, pc + 2);
	if mem_get(memory, pc, 0) < mem_get(memory, pc + 1, 1) {
		mem_set_addr(memory, addr, 1);
	} else {
		mem_set_addr(memory, addr, 0);
	}
	return 4;
}

pub fn opcode_eq(memory: &mut Vec<i32>, pc: i32) -> i32 {
	if DEBUG_LOG {
		println!(
			"opcode_eq({}) ({} == {}, {})",
			mem_get_noprint(memory, pc + 2, 2),
			mem_get_noprint(memory, pc, 0),
			mem_get_noprint(memory, pc + 1, 1),
			mem_get_noprint(memory, pc, 0) == mem_get_noprint(memory, pc + 1, 1),
		);
	}
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
		2 => "Multiply",
		3 => "Input",
		4 => "Output",
		5 => "Jump != zero",
		6 => "Jump == zero",
		7 => "Less than",
		8 => "Equals",
		99 => "Halt",
		_ => "____",
	}
}

pub fn parse_opcode(instruction: i32) -> i32 {
	let mut opcode = instruction;
	let mut digits: Vec<_> = Digits::new(instruction as usize).collect();
	if DEBUG_LOG {
		print!("parse_opcode: '{}' = ", instruction);
	}
	if digits.len() > 1 {
		digits.reverse();
		opcode = (*digits.get(0).expect("Expected i32") as i32)
			+ ((*digits.get(1).expect("Expected i32") as i32) * 10);
		if DEBUG_LOG {
			print!("{} ({}) ", opcode, opcode_to_str(opcode));
		}
		for i in 2..digits.len() {
			reg_set(i - 2, digits[i].try_into().unwrap());
			if DEBUG_LOG {
				print!("{}, ", digits[i]);
			}
		}
		if DEBUG_LOG {
			println!();
		}
	} else {
		if DEBUG_LOG {
			print!("{} ({}) ", opcode, opcode_to_str(opcode));
			println!();
		}
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
	unsafe {
		DEBUG_INPUT = 1;
	}
	return run_intcode(&mut memory);
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i32 {
	let mut memory: Vec<i32> = input
		.split_terminator(",")
		.map(|x| x.trim().parse::<i32>().expect("Expected integer"))
		.collect();
	// print_memory(&memory, 0);
	unsafe {
		DEBUG_INPUT = 5;
	}
	return run_intcode(&mut memory);
}
