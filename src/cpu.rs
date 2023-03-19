use std::fmt;

#[allow(clippy::upper_case_acronyms)]
mod instructions {
	// Load Accumulator
	pub const LDA_IMMEDIATE: u8 = 0xA9;
}

pub struct Cpu {
	program_counter: u16,
	x: u8,
	y: u8,
	a: u8,
	status: u8,
	stack_pointer: u8,
	cycles: u32,
}

impl Cpu {
	pub fn new() -> Self {
		Self {
			program_counter: 0xfffc,
			x: 0,
			y: 0,
			a: 0,
			status: 0,
			stack_pointer: 0xff,
			cycles: 0,
		}
	}
	pub fn reset(&mut self, mem: &[u8; 65536]) {
		let low_byte = self.fetch_memory(mem) as u16;
		let high_byte = self.fetch_memory(mem) as u16;
		self.program_counter = (high_byte << 8) | low_byte;
	}
	#[inline]
	fn get_flag(&self, flag: StatusFlags) -> bool {
		self.status & flag.get_bit_mask() != 0
	}
	#[inline]
	fn set_flag(&mut self, flag: StatusFlags, value: bool) {
		self.status = (flag.get_bit_mask() | self.status) * value as u8
			+ (!flag.get_bit_mask() & self.status) * !value as u8
	}

	#[inline]
	fn fetch_memory(&mut self, mem: &[u8; 65536]) -> u8 {
		let address = self.program_counter as usize;
		self.cycles += 1;
		self.program_counter += 1;
		mem[address]
	}

	pub fn execute(&mut self, mem: &mut [u8; 65536]) {
		let instruction = self.fetch_memory(mem);
		use instructions::*;
		match instruction {
			LDA_IMMEDIATE => {
				let operand = self.fetch_memory(mem);
				self.a = operand;
				self.set_flag(StatusFlags::Zero, self.a == 0);
				self.set_flag(StatusFlags::Negative, self.a & 0x80 > 0);
			}
			_ => panic!(
				"Invalid instruction found at location 0x{:04x} => 0x{instruction:02x}",
				self.program_counter - 1
			),
		}
	}
}

#[derive(Clone, Copy)]
enum StatusFlags {
	Carry,
	Zero,
	InterruptDisable,
	DecimalMode,
	Break,
	Overflow,
	Negative,
}

impl StatusFlags {
	fn get_bit_mask(self) -> u8 {
		1 << match self {
			Self::Carry => 0,
			Self::Zero => 1,
			Self::InterruptDisable => 2,
			Self::DecimalMode => 3,
			Self::Break => 4,
			Self::Overflow => 6,
			Self::Negative => 7,
		}
	}
}

impl fmt::Debug for Cpu {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = String::from("\n│CPU:\n");
		output.push_str("│A │X │Y │NV B DIZC│SP  │PC  │\n");
		output.push_str(&format!(
			"│{:02x}│{:02x}│{:02x}│{:04b} {:04b}│{:04x}│{:04x}│\n",
			self.a,
			self.x,
			self.y,
			self.status >> 4,
			(self.status << 4) >> 4,
			self.stack_pointer,
			self.program_counter,
		));
		write!(f, "{output}")
	}
}
