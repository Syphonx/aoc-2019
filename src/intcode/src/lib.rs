use digiter::Digits;
use std::convert::TryInto;
#[allow(unused_imports)]
use std::io::{self, BufRead};

const ENABLE_USER_INPUT: bool = false;
const DEBUG_LOG: bool = false;
const NUM_REG: usize = 4;
const NUM_FLAGS: usize = 5;

pub struct VM {
	ram: Vec<i32>,
	registers: [u32; NUM_REG],
	flags: [u32; NUM_FLAGS],
	input: i32,
}

impl VM {
	pub fn new() -> Self {
		VM {
			ram: Vec::new(),
			registers: [0; NUM_REG],
			flags: [0; NUM_FLAGS],
			input: 0,
		}
	}

	pub fn from_memory(memory: &Vec<i32>) -> VM {
		VM {
			ram: memory.clone(),
			registers: [0; NUM_REG],
			flags: [0; NUM_FLAGS],
			input: 0,
		}
	}

	pub fn set_input(&mut self, input: i32) {
		self.input = input;
	}

	pub fn mem_set(&mut self, index: i32, value: i32) {
		let address = self.ram[index as usize];
		if DEBUG_LOG {
			println!("mem_set({}) = {}", index, value);
		}
		self.ram[address as usize] = value;
	}

	pub fn mem_set_addr(&mut self, address: i32, value: i32) {
		if DEBUG_LOG {
			println!("mem_set_addr({}) = {}", address, value);
		}
		self.ram[address as usize] = value;
	}

	pub fn mem_get(&mut self, index: i32, param: usize) -> i32 {
		let mode = self.reg_get(param);
		match mode {
			0 => {
				return self.mem_get_pos(index);
			}
			1 => {
				return self.mem_get_addr(index);
			}
			_ => {
				return 0;
			}
		}
	}

	pub fn mem_get_pos(&mut self, address: i32) -> i32 {
		let value = self.ram[address as usize];
		if DEBUG_LOG {
			println!(
				"mem_get_pos({}) |0| = {}",
				address, self.ram[value as usize]
			);
		}
		return self.ram[value as usize];
	}

	pub fn mem_get_addr(&mut self, address: i32) -> i32 {
		let value = self.ram[address as usize];
		if DEBUG_LOG {
			println!("mem_get_addr({}) |1| = {}", address, value);
		}
		return value;
	}

	pub fn mem_get_noprint(&mut self, index: i32, param: usize) -> i32 {
		let mode = self.reg_get(param);
		let address = self.ram[index as usize];
		match mode {
			0 => {
				return self.ram[address as usize];
			}
			1 => {
				return address;
			}
			_ => {
				return 0;
			}
		}
	}

	pub fn reg_set(&mut self, index: usize, value: u32) {
		self.registers[index] = value;
	}

	pub fn reg_get(&mut self, index: usize) -> u32 {
		return self.registers[index];
	}

	pub fn reg_clear(&mut self) {
		for i in 0..NUM_REG {
			self.reg_set(i, 0);
		}
	}

	pub fn opcode_add(&mut self, pc: i32) -> i32 {
		if DEBUG_LOG {
			println!(
				"opcode_add {}+{}={}",
				self.mem_get_noprint(pc, 0),
				self.mem_get_noprint(pc + 1, 1),
				self.mem_get_noprint(pc, 0) + self.mem_get_noprint(pc + 1, 1)
			);
		}
		let value = self.mem_get(pc, 0) + self.mem_get(pc + 1, 1);
		self.mem_set(pc + 2, value);
		return 4;
	}

	pub fn opcode_mul(&mut self, pc: i32) -> i32 {
		if DEBUG_LOG {
			println!(
				"opcode_mul {}*{}={}",
				self.mem_get_noprint(pc, 0),
				self.mem_get_noprint(pc + 1, 1),
				self.mem_get_noprint(pc, 0) * self.mem_get_noprint(pc + 1, 1)
			);
		}
		let value = self.mem_get(pc, 0) * self.mem_get(pc + 1, 1);
		self.mem_set(pc + 2, value);
		return 4;
	}

	pub fn opcode_in(&mut self, pc: i32) -> i32 {
		if DEBUG_LOG {
			println!("opcode_in... waiting for input");
		}
		if ENABLE_USER_INPUT {
			let stdin = io::stdin();
			self.input = stdin
				.lock()
				.lines()
				.next()
				.unwrap()
				.unwrap()
				.parse::<i32>()
				.expect("Expected integer");
		}
		let value = self.mem_get_addr(pc);
		self.mem_set_addr(value, self.input);
		if DEBUG_LOG {
			println!("opcode_in {} = {}", value, self.input);
		}
		return 2;
	}

	pub fn opcode_out(&mut self, pc: i32) -> i32 {
		println!("opcode_out: {} ", self.mem_get(pc, 0));
		return 2;
	}

	pub fn opcode_jnz(&mut self, pc: i32) -> i32 {
		if DEBUG_LOG {
			println!(
				"opcode_jnz ({} != 0, {}) = {}",
				self.mem_get_noprint(pc, 0),
				self.mem_get_noprint(pc, 0) != 0,
				self.mem_get_noprint(pc + 1, 1),
			);
		}
		if self.mem_get(pc, 0) != 0 {
			return self.mem_get(pc + 1, 1);
		} else {
			return pc + 2;
		}
	}

	pub fn opcode_jiz(&mut self, pc: i32) -> i32 {
		if DEBUG_LOG {
			println!(
				"opcode_jiz ({} == 0, {}) = {}",
				self.mem_get_noprint(pc, 0),
				self.mem_get_noprint(pc, 0) == 0,
				self.mem_get_noprint(pc + 1, 1),
			);
		}
		if self.mem_get(pc, 0) == 0 {
			return self.mem_get(pc + 1, 1);
		} else {
			return pc + 2;
		}
	}

	pub fn opcode_lt(&mut self, pc: i32) -> i32 {
		if DEBUG_LOG {
			println!(
				"opcode_lt({}) ({} < {}, {})",
				self.mem_get_noprint(pc + 2, 2),
				self.mem_get_noprint(pc, 0),
				self.mem_get_noprint(pc + 1, 1),
				self.mem_get_noprint(pc, 0) < self.mem_get_noprint(pc + 1, 1),
			);
		}
		let addr = self.mem_get_addr(pc + 2);
		if self.mem_get(pc, 0) < self.mem_get(pc + 1, 1) {
			self.mem_set_addr(addr, 1);
		} else {
			self.mem_set_addr(addr, 0);
		}
		return 4;
	}

	pub fn opcode_eq(&mut self, pc: i32) -> i32 {
		if DEBUG_LOG {
			println!(
				"opcode_eq({}) ({} == {}, {})",
				self.mem_get_noprint(pc + 2, 2),
				self.mem_get_noprint(pc, 0),
				self.mem_get_noprint(pc + 1, 1),
				self.mem_get_noprint(pc, 0) == self.mem_get_noprint(pc + 1, 1),
			);
		}
		let addr = self.mem_get_addr(pc + 2);
		if self.mem_get(pc, 0) == self.mem_get(pc + 1, 1) {
			self.mem_set_addr(addr, 1);
		} else {
			self.mem_set_addr(addr, 0);
		}
		return 4;
	}

	pub fn opcode_to_str(&mut self, input: i32) -> &'static str {
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

	pub fn parse_opcode(&mut self, instruction: i32) -> i32 {
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
				print!("{} ({}) ", opcode, self.opcode_to_str(opcode));
			}
			for i in 2..digits.len() {
				self.reg_set(i - 2, digits[i].try_into().unwrap());
				if DEBUG_LOG {
					print!("{}, ", digits[i]);
				}
			}
			if DEBUG_LOG {
				println!();
			}
		} else {
			if DEBUG_LOG {
				print!("{} ({}) ", opcode, self.opcode_to_str(opcode));
				println!();
			}
		}
		return opcode;
	}

	pub fn run_intcode(&mut self) -> i32 {
		let mut pc: i32 = 0;
		loop {
			self.reg_clear();
			let opcode = self.parse_opcode(self.ram[pc as usize]);
			match opcode {
				1 => {
					pc += self.opcode_add(pc + 1);
				}
				2 => {
					pc += self.opcode_mul(pc + 1);
				}
				3 => {
					pc += self.opcode_in(pc + 1);
				}
				4 => {
					pc += self.opcode_out(pc + 1);
				}
				5 => {
					pc = self.opcode_jnz(pc + 1);
				}
				6 => {
					pc = self.opcode_jiz(pc + 1);
				}
				7 => {
					pc += self.opcode_lt(pc + 1);
				}
				8 => {
					pc += self.opcode_eq(pc + 1);
				}
				99 => {
					break;
				}
				_ => {
					pc += 1;
					println!("Invalid opcode: {}", opcode);
				}
			}
			// print_memory(pc);
			// println!("PC: {}", pc);
		}
		return self.ram[0];
	}

	pub fn print_memory(&mut self, memory: &Vec<i32>, pc: i32) {
		let column = 10;
		let mut current_column = 0;
		let mut row = 0;
		println!("---- REG ----");
		for index in 0..NUM_REG {
			print!("{}\t", self.reg_get(index));
		}
		println!("");
		println!("---- REG ----");
		println!("---- ROM ----");
		for index in 0..memory.len() {
			if current_column == 0 {
				print!("{}\t| ", row * 10);
			}
			if pc as usize == index {
				print!("*{}*\t", self.ram[index]);
			} else {
				print!("{}\t", self.ram[index]);
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
}

#[cfg(test)]
mod tests {

	/*
		The example program below uses an input instruction to ask for a single number.
		The program will then output 999 if the input value is below 8, output 1000 if
		the input value is equal to 8, or output 1001 if the input value is greater than 8.
	*/
	#[test]
	fn it_works() {
		let mut vm = VM::new();
		vm = VM::from_memory(
			[
				3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
				98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
				1, 20, 4, 20, 1105, 1, 46, 98, 99,
			]
			.to_vec(),
		);
		vm.set_input(5);
		vm.run_intcode();
	}
}
