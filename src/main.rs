mod cpu;
use cpu::*;

fn main() {
	let mut mem = read_mem("a.out");
	let mut cpu = Cpu::new();
	cpu.reset(&mem);
	println!("Reset the CPU: {cpu:?}");
	loop {
		mem.data[0xfe] = rand::random();
		cpu.execute(&mut mem);
		println!("{:02x?}", &mem.data[0x200..0x300]);
		println!("{cpu:?}");
		// println!("{:02x?}", mem[0xfe]);
		// println!("{:02x?}", mem[0x200]);
	}
}

fn read_mem(file_path: &'static str) -> Memory {
	let rom = std::fs::read(file_path).unwrap();
	let data = [0; MEMORY_SIZE];
	let mut mem = Memory::new(data);
	assert_eq!(rom.len(), MEMORY_SIZE, "Invalid ROM size");
	for (index, val) in rom.iter().enumerate() {
		mem.data[index] = *val;
	}
	mem
}
