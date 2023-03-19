use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
enum Operation {
	/// Add with Carry
	ADC,
	/// Logical AND
	AND,
	/// Arithemetic Shift Left
	ASL,
	/// Branch if Carry Clear
	BCC,
	/// Branch if Carry Set
	BCS,
	/// Branch if Equal
	BEQ,
	/// Bit Test
	BIT,
	/// Branch if Minus
	BMI,
	/// Branch if Not Equal
	BNE,
	/// Branch if Positive
	BPL,
	/// Force Interrupt
	BRK,
	/// Branch if Overflow Clear
	BVC,
	/// Branch if Overflow Set
	BVS,
	/// Clear Carry Flag
	CLC,
	/// Clear Decimal Mode
	CLD,
	/// Clear Interrupt Disable
	CLI,
	/// Clear Overflow Flag
	CLV,
	/// Compare
	CMP,
	/// Compare X Register
	CPX,
	/// Compare Y Register
	CPY,
	/// Decrement Memory
	DEC,
	/// Decrement X Register
	DEX,
	/// Decrement Y Register
	DEY,
	/// Exclusive OR
	EOR,
	/// Increment Memory
	INC,
	/// Increment X Register
	INX,
	/// Increment Y Register
	INY,
	/// Jump
	JMP,
	/// Jump to Subroutine
	JSR,
	/// Load Accumulator
	LDA,
	/// Load X Register
	LDX,
	/// Load Y Register
	LDY,
	/// Logical Shift Right
	LSR,
	/// No Operation
	NOP,
	/// Logical Inclusive OR
	ORA,
	/// Push Accumulator
	PHA,
	/// Push Processor Status
	PHP,
	/// Pull Accumulator
	PLA,
	/// Pull Processor Status
	PLP,
	/// Rotate Left
	ROL,
	/// Rotate Right
	ROR,
	/// Return from Interrupt
	RTI,
	/// Return from Subroutine
	RTS,
	/// Subtract with Carry
	SBC,
	/// Set Carry Flag
	SEC,
	/// Set Decimal Flag
	SED,
	/// Set Interrupt Disable
	SEI,
	/// Store Accumulator
	STA,
	/// Store X
	STX,
	/// Store Y
	STY,
	/// Transfer Accumulator to X
	TAX,
	/// Transfer Accumulator to Y
	TAY,
	/// Transfer Stack Pointer to X
	TSX,
	/// Transfer X to Accumulator
	TXA,
	/// Transfer X to Stack Pointer
	TXS,
	/// Transfer Y to Accumulator
	TYA,
}

#[derive(Clone, Copy, Debug)]
enum Operand {
	Immediate(u8),
	Address(u16),
}

#[derive(Clone, Copy, Debug)]
struct Instruction(Operation, Operand);

pub struct Cpu {
	program_counter: u16,
	x: u8,
	y: u8,
	a: u8,
	status: u8,
	stack_pointer: u8,
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
		}
	}
	pub fn execute(&mut self, mem: &mut [u8; 65536]) {
		let instruction = self.decode(mem);
		use Operand::*;
		use Operation::*;
		match instruction {
			Instruction(LDA, Immediate(value)) => self.load_accumulator(value),
			_ => panic!("Invalid instruction: {:x?}", instruction),
		}
	}
	fn decode(&mut self, mem: &[u8; 65536]) -> Instruction {
		let operation = self.fetch_memory(mem);
		match operation {
			0xA9 => Instruction(Operation::LDA, Operand::Immediate(self.fetch_memory(mem))),
			_ => panic!(
				"Invalid instruction found at location 0x{:04x} => 0x{operation:02x}",
				self.program_counter - 1
			),
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

	fn fetch_memory(&mut self, mem: &[u8; 65536]) -> u8 {
		let address = self.program_counter;
		eprintln!(
			"Fetched value: {:02x} from mem addr: {address:04x}",
			mem[address as usize]
		);
		self.program_counter += 1;
		mem[address as usize]
	}

	fn load_accumulator(&mut self, value: u8) {
		self.a = value;
		self.set_flag(StatusFlags::Zero, self.a == 0);
		self.set_flag(StatusFlags::Negative, self.a & 0x80 > 0);
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
