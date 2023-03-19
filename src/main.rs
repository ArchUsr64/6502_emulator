mod cpu;
use cpu::*;

fn main() {
	let mut mem = [0u8; 1 << 16];
	let mut cpu = Cpu::new();
	// Inline machine code for testing
	mem[0xfffc] = 0x34;
	mem[0xfffd] = 0x12;
	mem[0x1234] = 0xa9;
	mem[0x1235] = 0xab;
	cpu.reset(&mem);
	println!("{cpu:?}");
	cpu.execute(&mut mem);
	println!("{cpu:?}")
}
