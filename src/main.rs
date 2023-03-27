mod cpu;
use cpu::*;

fn main() {
	let mut mem = [0u8; 1 << 16];
	let mut cpu = Cpu::new();
	// Inline machine code for testing
	// Set pointer to first instruction
	mem[0xfffc] = 0x34;
	mem[0xfffd] = 0x12;
	// LDA ($fc),Y
	mem[0x1234] = 0xb1;
	mem[0x1235] = 0xfc;
	mem[0x00fc] = 0x35;
	mem[0x00fd] = 0x12;
	//STA $02,X
	mem[0x1236] = 0x95;
	mem[0x1237] = 0x02;
	//ADC #24
	mem[0x1238] = 0x69;
	mem[0x1239] = 0x24;
	//ADC $1243
	mem[0x123a] = 0x6d;
	mem[0x123b] = 0x43;
	mem[0x123c] = 0x12;
	mem[0x1243] = 0xab;
	cpu.reset(&mem);
	println!("{cpu:?}");
	cpu.execute(&mut mem);
	println!("{cpu:?}");
	cpu.execute(&mut mem);
	println!("{cpu:?}");
	cpu.execute(&mut mem);
	println!("{cpu:?}");
	cpu.execute(&mut mem);
	println!("{cpu:?}");
	assert_eq!(mem[0x02], 0xfc);
}
