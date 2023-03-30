mod cpu;
use cpu::*;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
	let data = read_mem("a.out");
	let mut mem = Memory::new(data);
	let mut cpu = Cpu::new();
	cpu.reset(&mem);
	println!("Reset the CPU: {cpu:?}");
	loop {
		mem.data[0xfe] = rand::gen_range(0, u8::MAX);
		cpu.execute(&mut mem);
		println!("{:02x?}", &mem.data[0x200..0x300]);
		println!("{cpu:?}");
	}
}

fn read_mem(file_path: &'static str) -> [u8; MEMORY_SIZE] {
	let rom = std::fs::read(file_path).unwrap();
	let mut data = [0; MEMORY_SIZE];
	assert_eq!(rom.len(), MEMORY_SIZE, "Invalid ROM size");
	for (index, val) in rom.iter().enumerate() {
		data[index] = *val;
	}
	data
}
