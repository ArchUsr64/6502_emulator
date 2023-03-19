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
	Implicit,
	Value(u8),
	Address(u16),
}

#[derive(Clone, Copy, Debug)]
struct Instruction(Operation, Operand);

#[derive(Clone, Copy, Debug)]
enum AddressingMode {
	Implicit,
	Accumulator,
	Immediate,
	ZeroPage,
	ZeroPageX,
	ZeroPageY,
	Relative,
	Absolute,
	AbsoluteX,
	AbsoluteY,
	Indirect,
	IndexedIndirect,
	IndirectIndexed,
}

impl AddressingMode {
	fn get_operand(&self, cpu: &mut Cpu, mem: &[u8; 65536]) -> Operand {
		use AddressingMode as AM;
		use Operand::*;
		match self {
			AM::Immediate => Value(cpu.fetch_byte(mem)),
			AM::ZeroPage => Address(cpu.fetch_byte(mem) as u16),
			AM::ZeroPageX => Address(cpu.fetch_byte(mem).wrapping_add(cpu.x) as u16),
			AM::ZeroPageY => Address(cpu.fetch_byte(mem).wrapping_add(cpu.y) as u16),
			AM::Absolute => Address(cpu.fetch_word(mem)),
			AM::AbsoluteX => Address(cpu.fetch_word(mem).wrapping_add(cpu.x as u16)),
			AM::AbsoluteY => Address(cpu.fetch_word(mem).wrapping_add(cpu.y as u16)),
			AM::Indirect => Address(read_word(mem, cpu.fetch_word(mem))),
			//(Indirect, X)
			//Zero page address specified at the next byte + X as indexing register
			AM::IndexedIndirect => Address(read_word_from_zero_page(
				mem,
				cpu.fetch_byte(mem).wrapping_add(cpu.x),
			)),
			//(Indirect), Y
			//16-bit address specified at the zero page at next byte address + Y as indexing register
			AM::IndirectIndexed => Address({
				let address_from_zero_page =
					dbg!(read_word_from_zero_page(mem, cpu.fetch_byte(mem)));
				(address_from_zero_page >> 8) << 8
					| (address_from_zero_page as u8).wrapping_add(cpu.y) as u16
			}),
			AM::Implicit | AM::Accumulator => Implicit,
			AM::Relative => Value(cpu.fetch_byte(mem)),
		}
	}
}

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
	pub fn reset(&mut self, mem: &[u8; 65536]) {
		self.program_counter = self.fetch_word(mem);
	}

	pub fn execute(&mut self, mem: &mut [u8; 65536]) {
		let instruction = self.decode(mem);
		use Operand::*;
		use Operation::*;
		match instruction {
			Instruction(LDA, Value(value)) => self.load_accumulator(value),
			Instruction(LDA, Address(addr)) => self.load_accumulator(mem[dbg!(addr) as usize]),
			Instruction(STA, Address(addr)) => mem[addr as usize] = self.a,
			_ => panic!("Invalid instruction: {:x?}", instruction),
		}
	}

	fn decode(&mut self, mem: &[u8; 65536]) -> Instruction {
		let operation = self.fetch_byte(mem);
		let mut operand = |addressing_mode: AddressingMode| addressing_mode.get_operand(self, mem);
		use AddressingMode::*;
		use Operation::*;
		match operation {
			//Load Accumulator
			0xa9 => Instruction(LDA, operand(Immediate)),
			0xa5 => Instruction(LDA, operand(ZeroPage)),
			0xb5 => Instruction(LDA, operand(ZeroPageX)),
			0xad => Instruction(LDA, operand(Absolute)),
			0xbd => Instruction(LDA, operand(AbsoluteX)),
			0xb9 => Instruction(LDA, operand(AbsoluteY)),
			0xa1 => Instruction(LDA, operand(IndexedIndirect)),
			0xb1 => Instruction(LDA, operand(IndirectIndexed)),
			//Store Accumulator
			0x85 => Instruction(STA, operand(ZeroPage)),
			0x95 => Instruction(STA, operand(ZeroPageX)),
			0x8d => Instruction(STA, operand(Absolute)),
			0x9d => Instruction(STA, operand(AbsoluteX)),
			0x99 => Instruction(STA, operand(AbsoluteY)),
			0x81 => Instruction(STA, operand(IndexedIndirect)),
			0x91 => Instruction(STA, operand(IndirectIndexed)),
			_ => panic!(
				"Invalid instruction found at location 0x{:04x} => 0x{operation:02x}",
				self.program_counter - 1
			),
		}
	}
	fn get_flag(&self, flag: StatusFlags) -> bool {
		self.status & flag.get_bit_mask() != 0
	}
	fn set_flag(&mut self, flag: StatusFlags, value: bool) {
		self.status = (flag.get_bit_mask() | self.status) * value as u8
			+ (!flag.get_bit_mask() & self.status) * !value as u8
	}
	fn fetch_word(&mut self, mem: &[u8; 65536]) -> u16 {
		let address = self.program_counter;
		self.program_counter += 2;
		let word = read_word(mem, address);
		eprintln!("Fetched word: {:04x} from: {address:04x}", word);
		word
	}
	fn fetch_byte(&mut self, mem: &[u8; 65536]) -> u8 {
		let address = self.program_counter;
		eprintln!(
			"Fetched byte: {:02x} from: {address:04x}",
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

fn read_word(mem: &[u8; 65536], address: u16) -> u16 {
	let lower_byte = mem[address as usize] as u16;
	let higher_byte = mem[(address + 1) as usize] as u16;
	higher_byte << 8 | lower_byte
}

fn read_word_from_zero_page(mem: &[u8; 65536], address: u8) -> u16 {
	let lower_byte = mem[address as usize] as u16;
	let higher_byte = mem[address.wrapping_add(1) as usize] as u16;
	higher_byte << 8 | lower_byte
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
