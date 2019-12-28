use env_logger;
use log::LevelFilter;

use num_derive::FromPrimitive; // For converting intcode into enums
use num_traits::FromPrimitive;
use std::cell::Cell; // For multiple mutable references // For converting intcode into enumss

#[derive(PartialEq)]
pub enum Status {
	WaitForInput,
	NewOutput,
	Halt,
}

#[derive(FromPrimitive)]
enum Opcode {
	ADD = 1,   // Addition
	MUL = 2,   // Multiplication
	IN = 3,    // Read input
	OUT = 4,   // Write output
	JIT = 5,   // Jump If True
	JIF = 6,   // Jump If False
	LT = 7,    // Less Than check
	EQ = 8,    // Equal check
	HALT = 99, // End of program
}

// Memory access modes
#[derive(FromPrimitive)]
enum MemMode {
	Address = 0,
	Immediate = 1,
}

pub struct VM {
	pub input: Vec<i64>,  // Queue of input values
	pub output: Vec<i64>, // Queue of output values
	pc: Cell<usize>,      // Program counter, keeps track of execution
	ram: Vec<i64>,        // Internal memory of the machine
}

pub fn enable_logging() -> bool {
	return env_logger::builder()
		.format_timestamp(None)
		.filter_level(LevelFilter::Info)
		.format_module_path(false)
		.try_init()
		.is_ok();
}

impl VM {
	pub fn new() -> Self {
		VM {
			input: Vec::new(),
			output: Vec::new(),
			pc: Cell::new(0),
			ram: Vec::new(),
		}
	}

	pub fn from_memory(memory: &Vec<i64>) -> VM {
		let vm: VM = VM::new();
		VM {
			ram: memory.clone(),
			..vm // Update syntax: only update 'ram'
		}
	}

	pub fn reset(&mut self, memory: &Vec<i64>) {
		self.input.clear();
		self.output.clear();
		self.pc = Cell::new(0);
		self.ram = memory.clone();
	}

	pub fn process_input(&mut self) {
		self.opcode_in();
    }
    
	fn get_param_modes(intcode: i64, count: u32) -> Vec<MemMode> {
		(0..count)
			.map(|i| {
				let m = (intcode / ((10 as i64).pow(i + 2))) % 10;
				MemMode::from_i64(m).expect("Bad MemMode")
			})
			.collect()
	}

	fn mem_read(&self, index: usize, mode: &MemMode) -> i64 {
		match mode {
			MemMode::Address => {
				let addr = self.ram[index] as usize;
				self.ram[addr]
			}
			MemMode::Immediate => self.ram[index],
		}
	}

	fn mem_write(&mut self, index: usize, value: i64) {
		// Writes are always in MemMode::Address mode
		let addr = self.ram[index] as usize;
		self.ram[addr] = value;
	}

	fn opcode_add(&mut self, intcode: i64) {
		let pmodes = VM::get_param_modes(intcode, 2);
		let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
		let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
		self.mem_write(self.next_ip(), p0 + p1);
	}

	fn opcode_mul(&mut self, intcode: i64) {
		let pmodes = VM::get_param_modes(intcode, 2);
		let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
		let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
		self.mem_write(self.next_ip(), p0 * p1);
	}

	fn opcode_in(&mut self) -> Option<Status> {
		if self.input.is_empty() {
			return Some(Status::WaitForInput);
		} else {
			let input = self.input.remove(0);
			self.mem_write(self.next_ip(), input);
		}
		return None;
	}

	fn opcode_out(&mut self, intcode: i64) -> Status {
		let pmodes = VM::get_param_modes(intcode, 1);
		let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
		self.output.push(p0);
		return Status::NewOutput;
	}

	fn opcode_jit(&mut self, intcode: i64) {
		let pmodes = VM::get_param_modes(intcode, 2);
		let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
		let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
		if p0 != 0 {
			self.pc.set(p1 as usize);
		}
	}

	fn opcode_jif(&mut self, intcode: i64) {
		let pmodes = VM::get_param_modes(intcode, 2);
		let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
		let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
		if p0 == 0 {
			self.pc.set(p1 as usize);
		}
	}

	fn opcode_lt(&mut self, intcode: i64) {
		let pmodes = VM::get_param_modes(intcode, 2);
		let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
		let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
		if p0 < p1 {
			self.mem_write(self.next_ip(), 1);
		} else {
			self.mem_write(self.next_ip(), 0);
		}
	}

	fn opcode_eq(&mut self, intcode: i64) {
		let pmodes = VM::get_param_modes(intcode, 2);
		let p0 = self.mem_read(self.next_ip(), &pmodes[0]);
		let p1 = self.mem_read(self.next_ip(), &pmodes[1]);
		if p0 == p1 {
			self.mem_write(self.next_ip(), 1);
		} else {
			self.mem_write(self.next_ip(), 0);
		}
	}

	fn next_ip(&self) -> usize {
		let aux = self.pc.get();
		self.pc.set(self.pc.get() + 1);
		return aux;
	}

	pub fn run_intcode(&mut self) -> Status {
		loop {
			let intcode: i64 = *self.ram.get(self.next_ip()).expect("Bad address");
			let opcode = Opcode::from_i64(intcode % 100).expect("Bad opcode");
			match opcode {
				Opcode::ADD => {
					self.opcode_add(intcode);
				}
				Opcode::MUL => {
					self.opcode_mul(intcode);
				}
				Opcode::IN => match self.opcode_in() {
					Some(result) => return result,
					None => (),
				},
				Opcode::OUT => {
					return self.opcode_out(intcode);
				}
				Opcode::JIT => {
					self.opcode_jit(intcode);
				}
				Opcode::JIF => {
					self.opcode_jif(intcode);
				}
				Opcode::LT => {
					self.opcode_lt(intcode);
				}
				Opcode::EQ => {
					self.opcode_eq(intcode);
				}
				Opcode::HALT => {
					return Status::Halt;
				}
			}
		}
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
		let mut vm = VM::from_memory(
			[
				3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
				98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
				1, 20, 4, 20, 1105, 1, 46, 98, 99,
			]
			.to_vec(),
		);
		vm.queue_input(5);
		vm.run_intcode();
	}
}
