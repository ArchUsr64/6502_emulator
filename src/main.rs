mod cpu;
use std::collections::HashMap;

use cpu::*;
use egui_macroquad::*;

mod app;
use app::App;
use log::{info, LevelFilter};

use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use macroquad::prelude::{
	clear_background, draw_rectangle, draw_rectangle_lines, draw_text, is_key_down, next_frame,
	rand, screen_height, screen_width, Color, KeyCode, BLACK, WHITE,
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
	// TODO: Change default value depending on screen's refresh rate
	#[arg(short, long, default_value_t = 100)]
	instructions_per_framce: u32,
	/// Debug symbols generated by the provided assembler
	#[arg(short, long, default_value_t = String::from("symbols.dbg"))]
	debug_symbols: String,
	/// Assembly source code
	#[arg(short, long, default_value_t = String::from("examples/snake.asm"))]
	assembly_source: String,
}

#[macroquad::main("6502 Emulator")]
async fn main() {
	let args = Args::parse();
	#[cfg(target_family = "unix")]
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

	let mut debug_symbols_map: HashMap<usize, u16> = HashMap::new();
	#[cfg(target_family = "wasm")]
	let file = include_str!("../symbols.dbg");
	#[cfg(target_family = "unix")]
	let file = std::fs::read_to_string(args.debug_symbols).expect("Failed to read debug symbols");
	file.lines().for_each(|line| {
		let line_number: usize = line.split_whitespace().nth(0).unwrap().parse().unwrap();
		let pc_address: u16 =
			u16::from_str_radix(line.split_whitespace().last().unwrap(), 16).unwrap();
		debug_symbols_map.insert(line_number, pc_address);
	});

	#[cfg(target_family = "wasm")]
	let file = include_str!("../examples/snake.asm");
	#[cfg(target_family = "unix")]
	let file = std::fs::read_to_string(args.assembly_source).expect("Failed to read debug symbols");
	let source_file: Vec<String> = file.lines().map(|i| String::from(i)).collect();

	let mut last_pc_value = 0;
	let mut debug_symbols: Vec<u16> = Vec::with_capacity(source_file.len());
	for (line_number, _) in source_file.iter().enumerate().rev() {
		if let Some(&pc_addr) = debug_symbols_map.get(&line_number) {
			debug_symbols.push(pc_addr);
			last_pc_value = pc_addr;
		} else {
			debug_symbols.push(last_pc_value);
		}
	}
	debug_symbols.reverse();

	let data = read_mem(&args.executable);
	let mut mem = Memory::new(data);
	let mut cpu = Cpu::new();
	let mut app = App::new(debug_symbols, source_file);

	app.instructions_per_frame = args.instructions_per_framce;
	loop {
		if app.reset {
			cpu = Cpu::new();
			mem = Memory::new(data);
			app.reset = false;
		}
		let mut execute_one_cycle = || {
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
		};
		if !app.paused {
			(0..app.instructions_per_frame).for_each(|_| execute_one_cycle());
		} else if app.step {
			execute_one_cycle();
		}
		// Window Decorations
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
		draw_text(
			format!("FPS: {:.0}", macroquad::time::get_fps(),).as_str(),
			0.,
			0.5 * min_screen_dimension * 0.1,
			min_screen_dimension * 0.1,
			WHITE,
		);
		if app.paused {
			draw_text(
				format!("PAUSED",).as_str(),
				0.,
				1.5 * min_screen_dimension * 0.1,
				min_screen_dimension * 0.1,
				WHITE,
			);
		}

		egui_macroquad::ui(|egui_ctx| {
			app.render_ui(egui_ctx, &cpu);
		});

		egui_macroquad::draw();

		next_frame().await
	}
}

fn read_mem(file_path: &str) -> [u8; MEMORY_SIZE] {
	#[cfg(target_family = "unix")]
	let rom = std::fs::read(file_path).unwrap();
	#[cfg(target_family = "wasm")]
	let rom = include_bytes!("../a.out");
	let mut data = [0; MEMORY_SIZE];
	for (index, val) in rom.iter().enumerate() {
		data[index] = *val;
	}
	data
}
