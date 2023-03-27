mod cpu;
use cpu::*;

fn main() {
	let mut mem = read_mem("a.out").unwrap();
	let mut cpu = Cpu::new();
	cpu.reset(&mem);
	println!("Reset the CPU: {cpu:?}");
	loop {
		cpu.execute(&mut mem);
		println!("{cpu:?}");
	}
}

fn read_mem(file_path: &'static str)-> Option<[u8; 0x10000]>{
	let rom = std::fs::read(file_path).unwrap();
	assert_eq!(rom.len(), 0x10000);
	let mut mem = [0u8; 0x10000];
	for (index, val) in rom.iter().enumerate(){
		mem[index] = *val;
	}
	Some(mem)
}
