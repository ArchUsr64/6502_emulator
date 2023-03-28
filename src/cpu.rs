use std::fmt;

pub const MEM_SIZE: usize = 0x10000;
pub type Mem = [u8; MEM_SIZE];
pub const fn new_mem() -> Mem {
	[0; MEM_SIZE]
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operand {
	Value(u8),
	Address(u16),
}

#[derive(Clone, Copy, Debug)]
struct Instruction(Operation, Option<Operand>);

#[derive(Clone, Copy, Debug)]
enum AddressingMode {
	Implicit,
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
	fn get_operand(&self, cpu: &mut Cpu, mem: &Mem) -> Option<Operand> {
		use AddressingMode as AM;
		use Operand::*;
		Some(match self {
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
				let address_from_zero_page = read_word_from_zero_page(mem, cpu.fetch_byte(mem));
				address_from_zero_page.wrapping_add(cpu.y as u16)
			}),
			AM::Relative => Value(cpu.fetch_byte(mem)),
			AM::Implicit => return None,
		})
	}
}

#[derive(Clone, Copy)]
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
			stack_pointer: 0xfd,
		}
	}
	pub fn reset(&mut self, mem: &Mem) {
		self.program_counter = self.fetch_word(mem);
	}

	pub fn execute(&mut self, mem: &mut Mem) {
		let instruction = self.decode(mem);
		eprintln!("[Execute] {instruction:x?}");
		use Operand::*;
		use Operation::*;
		let pass_by_value = |operand| match operand {
			Value(x) => x,
			Address(x) => mem[x as usize],
		};
		let mut branch = |flag: StatusFlags, condition: bool, offset: u8| {
			if self.get_flag(flag) == condition {
				self.program_counter = (self.program_counter as i16 + (offset as i8) as i16) as u16
			}
		};

		match instruction {
			//Logical Operations
			Instruction(ADC, Some(operand)) => self.add_with_carry(pass_by_value(operand)),
			Instruction(AND, Some(operand)) => self.set_a(self.a & pass_by_value(operand)),
			Instruction(ASL, operand) => self.arithmetic_shift_left(mem, operand),
			Instruction(BIT, Some(operand)) => self.bit(pass_by_value(operand)),
			Instruction(CMP, Some(operand)) => {
				self.compare_register(pass_by_value(operand), self.a)
			}
			Instruction(CPX, Some(operand)) => {
				self.compare_register(pass_by_value(operand), self.x)
			}
			Instruction(CPY, Some(operand)) => {
				self.compare_register(pass_by_value(operand), self.y)
			}
			Instruction(DEC, Some(Address(addr))) => {
				mem[addr as usize] -= 1;
				self.update_zero_and_negative_flag(mem[addr as usize])
			}
			Instruction(DEX, None) => self.set_x(self.x - 1),
			Instruction(DEY, None) => self.set_y(self.y - 1),
			Instruction(INX, None) => self.set_x(self.x + 1),
			Instruction(INY, None) => self.set_y(self.y + 1),
			Instruction(EOR, Some(operand)) => self.set_a(self.a ^ pass_by_value(operand)),
			Instruction(INC, Some(Address(addr))) => {
				mem[addr as usize] += 1;
				self.update_zero_and_negative_flag(mem[addr as usize])
			}
			Instruction(LSR, operand) => self.logical_shift_right(mem, operand),
			Instruction(ORA, Some(operand)) => self.set_a(self.a | pass_by_value(operand)),
			Instruction(ROL, operand) => self.rotate_right(mem, operand),
			Instruction(ROR, operand) => self.rotate_left(mem, operand),
			Instruction(SBC, Some(operand)) => self.sub_with_carry(pass_by_value(operand)),
			//Flags
			Instruction(CLC, None) => self.set_flag(StatusFlags::Carry, false),
			Instruction(CLD, None) => self.set_flag(StatusFlags::DecimalMode, false),
			Instruction(CLI, None) => self.set_flag(StatusFlags::InterruptDisable, false),
			Instruction(CLV, None) => self.set_flag(StatusFlags::Overflow, false),
			Instruction(SEC, None) => self.set_flag(StatusFlags::Carry, true),
			Instruction(SED, None) => self.set_flag(StatusFlags::DecimalMode, true),
			Instruction(SEI, None) => self.set_flag(StatusFlags::InterruptDisable, true),
			//Misc
			Instruction(NOP, None) => (),
			Instruction(LDA, Some(operand)) => self.set_a(pass_by_value(operand)),
			Instruction(LDX, Some(operand)) => self.set_x(pass_by_value(operand)),
			Instruction(LDY, Some(operand)) => self.set_y(pass_by_value(operand)),
			Instruction(STA, Some(Address(addr))) => mem[addr as usize] = self.a,
			Instruction(STX, Some(Address(addr))) => mem[addr as usize] = self.x,
			Instruction(STY, Some(Address(addr))) => mem[addr as usize] = self.y,
			//Transfer
			Instruction(TAX, None) => self.set_x(self.a),
			Instruction(TAY, None) => self.set_y(self.a),
			Instruction(TSX, None) => self.set_x(self.stack_pointer),
			Instruction(TXA, None) => self.set_a(self.x),
			Instruction(TXS, None) => self.stack_pointer = self.x,
			Instruction(TYA, None) => self.set_a(self.y),
			//Branching
			Instruction(BCC, Some(Value(offset))) => branch(StatusFlags::Carry, false, offset),
			Instruction(BCS, Some(Value(offset))) => branch(StatusFlags::Carry, true, offset),
			Instruction(BEQ, Some(Value(offset))) => branch(StatusFlags::Zero, true, offset),
			Instruction(BMI, Some(Value(offset))) => branch(StatusFlags::Negative, true, offset),
			Instruction(BNE, Some(Value(offset))) => branch(StatusFlags::Zero, true, offset),
			Instruction(BPL, Some(Value(offset))) => branch(StatusFlags::Negative, false, offset),
			Instruction(BVC, Some(Value(offset))) => branch(StatusFlags::Overflow, false, offset),
			Instruction(BVS, Some(Value(offset))) => branch(StatusFlags::Overflow, true, offset),
			//Stack operations
			Instruction(JSR, Some(Address(addr))) => {
				self.push_word(mem, self.program_counter);
				self.program_counter = addr;
			}
			Instruction(RTS, None) => self.program_counter = self.pop_word(mem),
			Instruction(PHA, None) => self.push_byte(mem, self.a),
			Instruction(PHP, None) => self.push_byte(mem, self.status),
			Instruction(PLA, None) => {
				let data = self.pop_byte(mem);
				self.set_a(data)
			}
			Instruction(PLP, None) => self.status = self.pop_byte(mem),
			//Jump
			Instruction(JMP, Some(Address(addr))) => self.program_counter = addr,
			_ => panic!("Invalid instruction: {:x?}", instruction),
		}
	}

	fn push_word(&mut self, mem: &mut Mem, value: u16) {
		self.push_byte(mem, (value >> 8) as u8);
		self.push_byte(mem, value as u8);
	}
	fn pop_word(&mut self, mem: &mut Mem) -> u16 {
		let low_byte = self.pop_byte(mem);
		let high_byte = self.pop_byte(mem);
		(high_byte as u16) << 8 | low_byte as u16
	}
	fn push_byte(&mut self, mem: &mut Mem, value: u8) {
		mem[self.stack_pointer as usize | 0x100] = value;
		self.stack_pointer = self.stack_pointer.wrapping_sub(1);
	}
	fn pop_byte(&mut self, mem: &mut Mem) -> u8 {
		let value = mem[self.stack_pointer as usize | 0x100];
		self.stack_pointer = self.stack_pointer.wrapping_add(1);
		value
	}

	fn decode(&mut self, mem: &Mem) -> Instruction {
		let operation = self.fetch_byte(mem);
		let mut instruction = |operation, addressing_mode: AddressingMode| {
			Instruction(operation, addressing_mode.get_operand(self, mem))
		};
		use AddressingMode::*;
		use Operation::*;
		match operation {
			//Add with Carry
			0x69 => instruction(ADC, Immediate),
			0x65 => instruction(ADC, ZeroPage),
			0x75 => instruction(ADC, ZeroPageX),
			0x6d => instruction(ADC, Absolute),
			0x7d => instruction(ADC, AbsoluteX),
			0x79 => instruction(ADC, AbsoluteY),
			0x61 => instruction(ADC, IndexedIndirect),
			0x71 => instruction(ADC, IndirectIndexed),
			//Logical AND
			0x29 => instruction(AND, Immediate),
			0x25 => instruction(AND, ZeroPage),
			0x35 => instruction(AND, ZeroPageX),
			0x2d => instruction(AND, Absolute),
			0x3d => instruction(AND, AbsoluteX),
			0x39 => instruction(AND, AbsoluteY),
			0x21 => instruction(AND, IndexedIndirect),
			0x31 => instruction(AND, IndirectIndexed),
			//Arithemetic Shift Left
			0x0a => instruction(ASL, Implicit),
			0x06 => instruction(ASL, ZeroPage),
			0x16 => instruction(ASL, ZeroPageX),
			0x0e => instruction(ASL, Absolute),
			0x1e => instruction(ASL, AbsoluteX),
			//BIT
			0x24 => instruction(BIT, ZeroPage),
			0x2c => instruction(BIT, Absolute),
			//Compare
			0xc9 => instruction(CMP, Immediate),
			0xc5 => instruction(CMP, ZeroPage),
			0xd5 => instruction(CMP, ZeroPageX),
			0xcd => instruction(CMP, Absolute),
			0xdd => instruction(CMP, AbsoluteX),
			0xd9 => instruction(CMP, AbsoluteY),
			0xc1 => instruction(CMP, IndexedIndirect),
			0xd1 => instruction(CMP, IndirectIndexed),
			//Compare X
			0xe0 => instruction(CPX, Immediate),
			0xe4 => instruction(CPX, ZeroPage),
			0xec => instruction(CPX, Absolute),
			//Compare Y
			0xc0 => instruction(CPY, Immediate),
			0xc4 => instruction(CPY, ZeroPage),
			0xcc => instruction(CPY, Absolute),
			//Decrement
			0xc6 => instruction(DEC, ZeroPage),
			0xd6 => instruction(DEC, ZeroPageX),
			0xce => instruction(DEC, Absolute),
			0xde => instruction(DEC, AbsoluteX),
			//Decrement X
			0xca => instruction(DEX, Implicit),
			//Decrement Y
			0x88 => instruction(DEY, Implicit),
			//Exclusive OR
			0x49 => instruction(EOR, Immediate),
			0x45 => instruction(EOR, ZeroPage),
			0x55 => instruction(EOR, ZeroPageX),
			0x4d => instruction(EOR, Absolute),
			0x5d => instruction(EOR, AbsoluteX),
			0x59 => instruction(EOR, AbsoluteY),
			0x41 => instruction(EOR, IndexedIndirect),
			0x51 => instruction(EOR, IndirectIndexed),
			//Increment Memory
			0xe6 => instruction(INC, ZeroPage),
			0xf6 => instruction(INC, ZeroPageX),
			0xee => instruction(INC, Absolute),
			0xfe => instruction(INC, AbsoluteX),
			//Decrement X
			0xe8 => instruction(INX, Implicit),
			//Decrement Y
			0xc8 => instruction(INY, Implicit),
			//Load Accumulator
			0xa9 => instruction(LDA, Immediate),
			0xa5 => instruction(LDA, ZeroPage),
			0xb5 => instruction(LDA, ZeroPageX),
			0xad => instruction(LDA, Absolute),
			0xbd => instruction(LDA, AbsoluteX),
			0xb9 => instruction(LDA, AbsoluteY),
			0xa1 => instruction(LDA, IndexedIndirect),
			0xb1 => instruction(LDA, IndirectIndexed),
			//Load X Register
			0xa2 => instruction(LDX, Immediate),
			0xa6 => instruction(LDX, ZeroPage),
			0xb6 => instruction(LDX, ZeroPageY),
			0xae => instruction(LDX, Absolute),
			0xbe => instruction(LDX, AbsoluteY),
			//Load Y Register
			0xa0 => instruction(LDY, Immediate),
			0xa4 => instruction(LDY, ZeroPage),
			0xb4 => instruction(LDY, ZeroPageY),
			0xac => instruction(LDY, Absolute),
			0xbc => instruction(LDY, AbsoluteY),
			//Logical Shift Right
			0x4a => instruction(LSR, Implicit),
			0x46 => instruction(LSR, ZeroPage),
			0x56 => instruction(LSR, ZeroPageX),
			0x4e => instruction(LSR, Absolute),
			0x5e => instruction(LSR, AbsoluteX),
			//No Operation
			0xea => instruction(NOP, Implicit),
			//Logical Inclusive OR
			0x09 => instruction(ORA, Immediate),
			0x05 => instruction(ORA, ZeroPage),
			0x15 => instruction(ORA, ZeroPageX),
			0x0d => instruction(ORA, Absolute),
			0x1d => instruction(ORA, AbsoluteX),
			0x19 => instruction(ORA, AbsoluteY),
			0x01 => instruction(ORA, IndexedIndirect),
			0x11 => instruction(ORA, IndirectIndexed),
			//Rotate Left
			0x2a => instruction(ROL, Implicit),
			0x26 => instruction(ROL, ZeroPage),
			0x36 => instruction(ROL, ZeroPageX),
			0x2e => instruction(ROL, Absolute),
			0x3e => instruction(ROL, AbsoluteX),
			//Rotate Right
			0x6a => instruction(ROR, Implicit),
			0x66 => instruction(ROR, ZeroPage),
			0x76 => instruction(ROR, ZeroPageX),
			0x6e => instruction(ROR, Absolute),
			0x7e => instruction(ROR, AbsoluteX),
			//Subtract with Carry
			0xe9 => instruction(SBC, Immediate),
			0xe5 => instruction(SBC, ZeroPage),
			0xf5 => instruction(SBC, ZeroPageX),
			0xed => instruction(SBC, Absolute),
			0xfd => instruction(SBC, AbsoluteX),
			0xf9 => instruction(SBC, AbsoluteY),
			0xe1 => instruction(SBC, IndexedIndirect),
			0xf1 => instruction(SBC, IndirectIndexed),
			//Store accumulator
			0x85 => instruction(STA, ZeroPage),
			0x95 => instruction(STA, ZeroPageX),
			0x8d => instruction(STA, Absolute),
			0x9d => instruction(STA, AbsoluteX),
			0x99 => instruction(STA, AbsoluteY),
			0x81 => instruction(STA, IndexedIndirect),
			0x91 => instruction(STA, IndirectIndexed),
			//Store X Register
			0x86 => instruction(STX, ZeroPage),
			0x96 => instruction(STX, ZeroPageY),
			0x8e => instruction(STX, Absolute),
			//Store Y Register
			0x84 => instruction(STY, ZeroPage),
			0x94 => instruction(STY, ZeroPageY),
			0x8c => instruction(STY, Absolute),
			//Transfer
			0xaa => instruction(TAX, Implicit),
			0xa8 => instruction(TAY, Implicit),
			0xba => instruction(TSX, Implicit),
			0x8a => instruction(TXA, Implicit),
			0x9a => instruction(TXS, Implicit),
			0x98 => instruction(TYA, Implicit),
			//Clear Flags
			0x18 => instruction(CLC, Implicit),
			0xd8 => instruction(CLD, Implicit),
			0x58 => instruction(CLI, Implicit),
			0xb8 => instruction(CLV, Implicit),
			//Set Flags
			0x38 => instruction(SEC, Implicit),
			0xf8 => instruction(SED, Implicit),
			0x78 => instruction(SEI, Implicit),
			//Branch Instructions
			0x90 => instruction(BCC, Relative),
			0xb0 => instruction(BCS, Relative),
			0xf0 => instruction(BEQ, Relative),
			0x30 => instruction(BMI, Relative),
			0xd0 => instruction(BNE, Relative),
			0x10 => instruction(BPL, Relative),
			0x50 => instruction(BVC, Relative),
			0x70 => instruction(BVS, Relative),
			//Jump
			0x4c => instruction(JMP, Absolute),
			0x6c => instruction(JMP, Indirect),
			//Stack operations
			0x20 => instruction(JSR, Absolute),
			0x60 => instruction(RTS, Implicit),
			0x48 => instruction(PHA, Implicit),
			0x08 => instruction(PHP, Implicit),
			0x68 => instruction(PLA, Implicit),
			0x28 => instruction(PLP, Implicit),
			_ => panic!(
				"Invalid instruction found at location {:04x} => {operation:02x}",
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
	fn fetch_word(&mut self, mem: &Mem) -> u16 {
		let address = self.program_counter;
		self.program_counter += 2;
		let word = read_word(mem, address);
		eprintln!("[Fetch] word: {:04x} from: {address:04x}", word);
		word
	}
	fn fetch_byte(&mut self, mem: &Mem) -> u8 {
		let address = self.program_counter;
		eprintln!(
			"[Fetch] byte: {:02x} from: {address:04x}",
			mem[address as usize]
		);
		self.program_counter += 1;
		mem[address as usize]
	}
	fn set_a(&mut self, value: u8) {
		self.a = value;
		self.update_zero_and_negative_flag(value);
	}
	fn set_x(&mut self, value: u8) {
		self.x = value;
		self.update_zero_and_negative_flag(value);
	}
	fn set_y(&mut self, value: u8) {
		self.y = value;
		self.update_zero_and_negative_flag(value);
	}
	fn update_zero_and_negative_flag(&mut self, value: u8) {
		self.set_flag(StatusFlags::Zero, value == 0);
		self.set_flag(StatusFlags::Negative, value & 0x80 > 0);
	}
	fn arithmetic_shift_left(&mut self, mem: &mut Mem, operand: Option<Operand>) {
		if let Some(operand) = operand {
			match operand {
				Operand::Address(addr) => {
					self.set_flag(StatusFlags::Carry, mem[addr as usize] & 0x80 > 0);
					mem[addr as usize] >>= 1;
					self.update_zero_and_negative_flag(mem[addr as usize]);
				}
				Operand::Value(_) => panic!("Value operand not supported for ASL: {operand:?}"),
			}
		} else {
			self.set_flag(StatusFlags::Carry, self.a & 0x80 > 0);
			self.set_a(self.a >> 1);
		}
	}
	fn rotate_left(&mut self, mem: &mut Mem, operand: Option<Operand>) {
		if let Some(operand) = operand {
			match operand {
				Operand::Address(addr) => {
					let new_carray_value = mem[addr as usize] & 0x1 > 0;
					mem[addr as usize] <<= 1 | self.get_flag(StatusFlags::Carry) as u8;
					self.set_flag(StatusFlags::Carry, new_carray_value);
					self.update_zero_and_negative_flag(mem[addr as usize]);
				}
				Operand::Value(_) => panic!("Value operand not supported for ROL: {operand:?}"),
			}
		} else {
			let new_carray_value = self.a & 0x1 > 0;
			self.set_a(self.a << 1 | self.get_flag(StatusFlags::Carry) as u8);
			self.set_flag(StatusFlags::Carry, new_carray_value);
		}
	}
	fn rotate_right(&mut self, mem: &mut Mem, operand: Option<Operand>) {
		if let Some(operand) = operand {
			match operand {
				Operand::Address(addr) => {
					let new_carray_value = mem[addr as usize] & 0x1 > 0;
					mem[addr as usize] >>= 1 | self.get_flag(StatusFlags::Carry) as u8;
					self.set_flag(StatusFlags::Carry, new_carray_value);
					self.update_zero_and_negative_flag(mem[addr as usize]);
				}
				Operand::Value(_) => panic!("Value operand not supported for ROR: {operand:?}"),
			}
		} else {
			let new_carray_value = self.a & 0x1 > 0;
			self.set_a(self.a >> 1 | self.get_flag(StatusFlags::Carry) as u8);
			self.set_flag(StatusFlags::Carry, new_carray_value);
		}
	}
	fn logical_shift_right(&mut self, mem: &mut Mem, operand: Option<Operand>) {
		if let Some(operand) = operand {
			match operand {
				Operand::Address(addr) => {
					self.set_flag(StatusFlags::Carry, mem[addr as usize] & 0x1 > 0);
					mem[addr as usize] >>= 1;
					self.update_zero_and_negative_flag(mem[addr as usize]);
				}
				Operand::Value(_) => panic!("Value operand not supported for LSR: {operand:?}"),
			}
		} else {
			self.set_flag(StatusFlags::Carry, self.a & 0x1 > 0);
			self.set_a(self.a >> 1);
		}
	}
	fn bit(&mut self, value: u8) {
		self.set_flag(StatusFlags::Negative, self.a & 0x80 > 0);
		self.set_flag(StatusFlags::Overflow, self.a & 0x40 > 0);
		self.set_flag(StatusFlags::Zero, self.a & value == 0);
	}
	fn compare_register(&mut self, value: u8, register_value: u8) {
		self.set_flag(StatusFlags::Negative, (register_value - value) & 0x80 > 0);
		self.set_flag(StatusFlags::Carry, register_value >= value);
		self.set_flag(StatusFlags::Zero, register_value == value);
	}
	fn add_with_carry(&mut self, value: u8) {
		let result = self.a as u16
			+ value as u16
			+ if self.get_flag(StatusFlags::Carry) {
				1 << 8
			} else {
				0
			};
		self.a = result as u8;
		self.set_flag(StatusFlags::Zero, self.a == 0);
		self.set_flag(StatusFlags::Negative, self.a & 0x80 > 0);
		self.set_flag(StatusFlags::Carry, result & 0x100 > 0);
		let result = if result & 0x100 > 0 {
			result | 0xff00
		} else {
			result
		} as i16;
		self.set_flag(StatusFlags::Overflow, !(-128..=127).contains(&result));
	}
	fn sub_with_carry(&mut self, value: u8) {
		let result = self.a as u16
			+ if self.get_flag(StatusFlags::Carry) {
				0
			} else {
				1 << 8
			} - value as u16;
		self.a = result as u8;
		self.set_flag(StatusFlags::Zero, self.a == 0);
		self.set_flag(StatusFlags::Negative, self.a & 0x80 > 0);
		self.set_flag(StatusFlags::Carry, result & 0x100 != 0);
		let result = if result & 0x100 > 0 {
			result | 0xff00
		} else {
			result
		} as i16;
		self.set_flag(StatusFlags::Overflow, !(-128..=127).contains(&result));
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

fn write_byte(mem: &mut Mem, address: u16, value: u8) {
	mem[address as usize] = value;
}

fn read_word(mem: &Mem, address: u16) -> u16 {
	let lower_byte = mem[address as usize] as u16;
	let higher_byte = mem[(address + 1) as usize] as u16;
	higher_byte << 8 | lower_byte
}

fn read_word_from_zero_page(mem: &Mem, address: u8) -> u16 {
	let lower_byte = mem[address as usize] as u16;
	let higher_byte = mem[address.wrapping_add(1) as usize] as u16;
	higher_byte << 8 | lower_byte
}

impl fmt::Debug for Cpu {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut output = String::from("\n");
		output.push_str("╭───╮\n");
		output.push_str("│CPU│\n");
		output.push_str("├───┼──┬──┬─────────┬────┬────╮\n");
		output.push_str("│A  │X │Y │NV B DIZC│SP  │PC  │\n");
		output.push_str(&format!(
			"│{:02x} │{:02x}│{:02x}│{:04b} {:04b}│{:04x}│{:04x}│\n",
			self.a,
			self.x,
			self.y,
			self.status >> 4,
			(self.status << 4) >> 4,
			self.stack_pointer,
			self.program_counter,
		));
		output.push_str("╰───┴──┴──┴─────────┴────┴────╯\n");
		write!(f, "{output}")
	}
}
#[allow(unused)]
mod test {
	use crate::cpu::*;
	use rand::random;
	fn test_cpu() -> Cpu {
		let mut cpu = Cpu::new();
		cpu.program_counter = random();
		cpu.a = random();
		cpu.x = random();
		cpu.y = random();
		cpu
	}
	fn test_mem() -> Mem {
		let mut mem = new_mem();
		mem.iter_mut().for_each(|x| *x = random());
		mem
	}
	mod addressing_mode {
		#[allow(unused_imports)]
		use crate::cpu::test::*;
		#[test]
		fn immediate() {
			let mem = test_mem();
			let mut cpu = test_cpu();
			assert_eq!(
				AddressingMode::Immediate
					.get_operand(&mut cpu, &mem)
					.unwrap(),
				Operand::Value(mem[cpu.program_counter as usize - 1])
			);
		}
		#[test]
		fn zero_page() {
			let mem = test_mem();
			let mut cpu = test_cpu();
			assert_eq!(
				AddressingMode::ZeroPage
					.get_operand(&mut cpu, &mem)
					.unwrap(),
				Operand::Address(mem[cpu.program_counter as usize - 1] as u16)
			);
		}
		#[test]
		fn zero_page_xy() {
			let mem = test_mem();
			let mut cpu = test_cpu();
			assert_eq!(
				AddressingMode::ZeroPageX
					.get_operand(&mut cpu, &mem)
					.unwrap(),
				Operand::Address(
					(mem[cpu.program_counter as usize - 1].wrapping_add(cpu.x)) as u16
				)
			);
			assert_eq!(
				AddressingMode::ZeroPageY
					.get_operand(&mut cpu, &mem)
					.unwrap(),
				Operand::Address(
					(mem[cpu.program_counter as usize - 1].wrapping_add(cpu.y)) as u16
				)
			);
		}
		#[test]
		fn absolute() {
			let mem = test_mem();
			let mut cpu = test_cpu();
			if let Operand::Address(addr) = AddressingMode::Absolute
				.get_operand(&mut cpu, &mem)
				.unwrap()
			{
				assert_eq!(addr % 256, (mem[cpu.program_counter as usize - 2]) as u16);
				assert_eq!(addr >> 8, (mem[cpu.program_counter as usize - 1]) as u16);
			} else {
				panic!()
			}
		}
		#[test]
		fn absolute_xy() {
			let mem = test_mem();
			let mut cpu = test_cpu();
			let value = (
				AddressingMode::AbsoluteX.get_operand(&mut cpu, &mem),
				AddressingMode::AbsoluteY.get_operand(&mut cpu, &mem),
			);
			let get_bytes = |register_value, pc_offset: usize| match mem
				[cpu.program_counter as usize - 2 - pc_offset]
				.checked_add(register_value)
			{
				Some(val) => (val, mem[cpu.program_counter as usize - 1 - pc_offset]),
				None => (
					mem[cpu.program_counter as usize - 2 - pc_offset].wrapping_add(register_value),
					mem[cpu.program_counter as usize - 1 - pc_offset].wrapping_add(1),
				),
			};
			if let (Some(Operand::Address(addr_x)), Some(Operand::Address(addr_y))) = value {
				assert_eq!(addr_x % 256, get_bytes(cpu.x, 2).0 as u16,);
				assert_eq!(addr_x >> 8, get_bytes(cpu.x, 2).1 as u16,);
				assert_eq!(addr_y % 256, get_bytes(cpu.y, 0).0 as u16,);
				assert_eq!(addr_y >> 8, get_bytes(cpu.y, 0).1 as u16,);
			} else {
				panic!()
			}
		}
		#[test]
		fn indexed_indirect() {
			let mem = test_mem();
			let mut cpu = test_cpu();
			if let Operand::Address(addr) = AddressingMode::IndexedIndirect
				.get_operand(&mut cpu, &mem)
				.unwrap()
			{
				let zero_page_addr = mem[cpu.program_counter as usize - 1].wrapping_add(cpu.x);
				assert_eq!(addr, read_word_from_zero_page(&mem, zero_page_addr));
			} else {
				panic!()
			}
		}
		#[test]
		fn indirect_indexed() {
			let mem = test_mem();
			let mut cpu = test_cpu();
			if let Operand::Address(addr) = AddressingMode::IndirectIndexed
				.get_operand(&mut cpu, &mem)
				.unwrap()
			{
				let zero_page_addr =
					read_word_from_zero_page(&mem, mem[cpu.program_counter as usize - 1]);
				assert_eq!(addr, zero_page_addr.wrapping_add(cpu.y as u16));
			} else {
				panic!()
			}
		}
	}
}
