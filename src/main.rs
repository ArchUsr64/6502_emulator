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
		//Window Decorations
		clear_background(BLACK);
		let screen_size = (screen_width(), screen_height());
		let min_screen_dimension = screen_size.0.min(screen_size.1);
		let pixel_size = 0.95 * min_screen_dimension / 32.;
		let gap = (
			screen_size.0 - 32. * pixel_size,
			screen_size.1 - 32. * pixel_size,
		);
		draw_rectangle_lines(
			(gap.0 / 2.) - 2.,
			(gap.1 / 2.) - 2.,
			(pixel_size * 32.) + 4.,
			(pixel_size * 32.) + 4.,
			5.,
			WHITE,
		);
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
					gap.0 / 2. + j as f32 * pixel_size,
					gap.1 / 2. + i as f32 * pixel_size,
					pixel_size,
					pixel_size,
					color(mem.data[SCREEN_MEMORY_START + (i << 5) + j]),
				);
			})
		});
		let text_scalar = 0.1;
		draw_text(
			format!("FPS: {}", get_fps()).as_str(),
			0.,
			1.5 * min_screen_dimension * text_scalar,
			min_screen_dimension * text_scalar,
			WHITE,
		);
		draw_text(
			format!("{:.1} ms", get_frame_time() * 1000.).as_str(),
			0.,
			0.5 * min_screen_dimension * text_scalar,
			min_screen_dimension * text_scalar,
			WHITE,
		);
		mem.data[0xfe] = rand::gen_range(0, u8::MAX);
		cpu.execute(&mut mem);
		println!("{cpu:?}");
		next_frame().await;
	}
}

fn read_mem(file_path: &'static str) -> [u8; MEMORY_SIZE] {
	let rom = std::fs::read(file_path).unwrap();
	let mut data = [0; MEMORY_SIZE];
	for (index, val) in rom.iter().enumerate() {
		data[index] = *val;
	}
	data
}
