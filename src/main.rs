mod cpu;
use cpu::*;

use macroquad::prelude::*;
const SCREEN_MEMORY_START: usize = 0xf000;

#[macroquad::main("BasicShapes")]
async fn main() {
	let data = read_mem("a.out");
	let mut mem = Memory::new(data);
	let mut cpu = Cpu::new();
	loop {
		clear_background(BLACK);
		let screen_size = (screen_width(), screen_height());
		let pixel_size = (screen_size.0 / 32.).min(screen_size.1 / 32.);
		(0..32).for_each(|i| {
			(0..32).for_each(|j| {
				let color = |byte| {
					Color::new(
						(byte >> 5) as f32 / 8.,
						((byte >> 2) % 8u8) as f32 / 8.,
						(byte % 4) as f32 / 4.,
						1.,
					)
				};
				draw_rectangle(
					j as f32 * pixel_size,
					i as f32 * pixel_size,
					pixel_size,
					pixel_size,
					color(mem.data[SCREEN_MEMORY_START + (i << 5) + j]),
				);
			})
		});
		next_frame().await;
		// mem.data[0xfe] = rand::gen_range(0, u8::MAX);
		// cpu.execute(&mut mem);
		// println!("{cpu:?}");
	}
}

fn read_mem(file_path: &'static str) -> [u8; MEMORY_SIZE] {
	let rom = std::fs::read(file_path).unwrap();
	let mut data = [0; MEMORY_SIZE];
	for (index, val) in rom.iter().enumerate() {
		data[index] = *val;
	}
	data.iter_mut()
		.enumerate()
		.for_each(|(index, val)| *val = index as u8);
	data
}
