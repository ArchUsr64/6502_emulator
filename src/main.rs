mod cpu;
use cpu::*;

use log::{info, LevelFilter};

use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use macroquad::prelude::{
	clear_background, draw_rectangle, draw_rectangle_lines, draw_text, is_key_down, is_key_pressed,
	is_mouse_button_pressed, next_frame, rand, screen_height, screen_width, Color, KeyCode,
	MouseButton, BLACK, WHITE,
};
const SCREEN_MEMORY_START: usize = 0xfb00;
const INPUT_MEMORY_LOCATION: usize = 0xfb;
const RNG_MEMORY_LOCATION: usize = 0xff;

use clap::Parser;

/// A simple 6502 emulator
#[derive(Parser, Debug)]
struct Args {
	/// Path to the 6502 binary
	#[arg(default_value = "a.out")]
	executable: String,
	/// Debug Verbosity level [0-2]
	#[arg(short, long, default_value_t = 0)]
	verbosity: u8,
	/// Start in debug mode
	#[arg(short, long, default_value_t = false)]
	start_debug: bool,
	/// Number of CPU instructions to execute per frame
	#[arg(short, long, default_value_t = 100)]
	executions_per_frame: u16,
}

#[macroquad::main("6502 Emulator")]
async fn main() {
	let args = Args::parse();
	TermLogger::init(
		match args.verbosity {
			1 => LevelFilter::Info,
			2 => LevelFilter::Debug,
			_ => LevelFilter::Error,
		},
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto,
	)
	.unwrap();
	let data = read_mem(&args.executable);
	let mut mem = Memory::new(data);
	let mut cpu = Cpu::new();
	let mut paused = args.start_debug;
	use std::time;
	let mut frame_time = time::Duration::from_millis(0);
	loop {
		let start = time::Instant::now();
		if is_mouse_button_pressed(MouseButton::Left) {
			paused = !paused;
		}
		if !paused || is_key_pressed(KeyCode::Space) {
			(0..args.executions_per_frame).for_each(|_| {
				info!("{cpu:?}");
				cpu.execute(&mut mem);
				mem.data[RNG_MEMORY_LOCATION] = rand::gen_range(u8::MIN, u8::MAX);
				// Left, Down, Up, Right
				mem.data[INPUT_MEMORY_LOCATION] =
					(is_key_down(KeyCode::Left) | is_key_down(KeyCode::A)) as u8;
				mem.data[INPUT_MEMORY_LOCATION + 1] =
					(is_key_down(KeyCode::Down) | is_key_down(KeyCode::S)) as u8;
				mem.data[INPUT_MEMORY_LOCATION + 2] =
					(is_key_down(KeyCode::Up) | is_key_down(KeyCode::W)) as u8;
				mem.data[INPUT_MEMORY_LOCATION + 3] =
					(is_key_down(KeyCode::Right) | is_key_down(KeyCode::D)) as u8;
			});
		}
		// Window Decorations
		clear_background(BLACK);
		let screen_size = (screen_width(), screen_height());
		let min_screen_dimension = screen_size.0.min(screen_size.1);
		draw_text(
			format!(
				"FPS: {:.0}",
				(frame_time.as_micros() as f32 / 1_000_000.).recip()
			)
			.as_str(),
			0.,
			0.5 * min_screen_dimension * 0.1,
			min_screen_dimension * 0.1,
			WHITE,
		);
		if paused {
			draw_text(
				format!("PAUSED",).as_str(),
				0.,
				1.5 * min_screen_dimension * 0.1,
				min_screen_dimension * 0.1,
				WHITE,
			);
		}
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
		next_frame().await;
		frame_time = start.elapsed();
	}
}

fn read_mem(file_path: &str) -> [u8; MEMORY_SIZE] {
	let rom = std::fs::read(file_path).unwrap();
	let mut data = [0; MEMORY_SIZE];
	for (index, val) in rom.iter().enumerate() {
		data[index] = *val;
	}
	data
}
