use egui_macroquad::egui::Align2;
use egui_macroquad::egui::Color32;
use egui_macroquad::egui::Widget;

use crate::{cpu, egui, Memory};

pub struct App {
	pub paused: bool,
	pub step: bool,
	pub instructions_per_frame: u32,
	/// A debug symbols relates a line from the source code to the corresponding
	/// u16 program counter address
	pub debug_symbols: Vec<u16>,
	pub source_file: Vec<String>,
	pub reset: bool,
	/// Vector of line numbers
	breakpoints: Vec<usize>,
	breakpoints_user_entry: String,
	break_address: Vec<u16>,
	watchpoints: Vec<u16>,
	watchpoints_user_entry: String,
	ui_scale: f32,
	pub window_scale: f32,
}

impl App {
	pub fn new(debug_symbols: Vec<u16>, source_file: Vec<String>) -> Self {
		Self {
			step: false,
			paused: false,
			instructions_per_frame: 100,
			debug_symbols,
			source_file,
			reset: false,
			breakpoints: vec![],
			breakpoints_user_entry: String::new(),
			break_address: vec![],
			watchpoints: vec![],
			watchpoints_user_entry: String::new(),
			ui_scale: 1.,
			window_scale: 0.95,
		}
	}
	pub fn breakpoints_addresses(&self) -> &[u16] {
		&self.break_address
	}
	pub fn render_ui(&mut self, ctx: &egui::Context, cpu: &cpu::Cpu, mem: &mut Memory) {
		let current_line_number = self
			.debug_symbols
			.iter()
			.position(|&i| i == cpu.state().program_counter)
			.unwrap();
		egui::Window::new("Debug Controls").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("UI Scale: ");
				if ui
					.add(egui::Slider::new(&mut self.ui_scale, 0.5f32..=3.))
					.drag_released()
				{
					ctx.set_pixels_per_point(self.ui_scale);
				};
			});
			ui.horizontal(|ui| {
				ui.label("Emulator Window: ");
				ui.add(egui::Slider::new(&mut self.window_scale, 0.1f32..=1.))
			});
			ui.horizontal(|ui| {
				ui.label("Simulation Speed: ");
				ui.add(egui::Slider::new(
					&mut self.instructions_per_frame,
					1u32..=500,
				))
			});
			ui.horizontal(|ui| {
				if ui
					.add(egui::Button::new(if !self.paused {
						"Pause"
					} else {
						"Resume"
					}))
					.clicked()
				{
					self.paused = !self.paused;
				};
				if self.paused {
					if ui.add(egui::Button::new("Step")).clicked() {
						self.step = true
					};
				}
				if ui.add(egui::Button::new("Reset")).clicked() {
					self.reset = true
				};
			});
			let cpu_state = cpu.state();
			ui.label("Program Counter:");
			ui.label(
				egui::RichText::new(format!("0x{:04x}", cpu_state.program_counter))
					.monospace()
					.color(Color32::GOLD),
			);
			ui.label("Stack Pointer:");
			ui.label(
				egui::RichText::new(format!("0x{:02x}", cpu_state.stack_pointer))
					.monospace()
					.color(Color32::BROWN),
			);
			ui.label("Line Number:");
			ui.label(
				egui::RichText::new((current_line_number + 1).to_string())
					.monospace()
					.color(Color32::LIGHT_RED),
			);
			ui.add(egui::Label::new("Instruction:"));
			let line = self.source_file[current_line_number].trim_start();
			ui.horizontal(|ui| {
				for (i, words) in line.split_whitespace().enumerate() {
					ui.label(egui::RichText::new(words).monospace().color(if i == 0 {
						Color32::KHAKI
					} else {
						Color32::GOLD
					}));
				}
			});
			ui.add(egui::Label::new("Registers:"));
			ui.label(
				egui::RichText::new(format!(
					"A: 0x{:02x}, X: 0x{:02x}, Y: 0x{:02x}",
					cpu_state.a, cpu_state.x, cpu_state.y
				))
				.monospace()
				.color(Color32::LIGHT_GREEN),
			);
		});
		egui::Window::new("Source Code").show(ctx, |ui| {
			egui::ScrollArea::vertical().hscroll(true).show(ui, |ui| {
				self.source_file
					.iter()
					.enumerate()
					.for_each(|(line_number, line)| {
						ui.horizontal(|ui| {
							if egui::Label::new(
								egui::RichText::new(format!(
									"{}{} ",
									" ".repeat(3 - (line_number + 1).to_string().len()),
									line_number + 1
								))
								.monospace()
								.background_color(if line_number == current_line_number {
									Color32::RED
								} else if self.breakpoints.contains(&(line_number + 1)) {
									Color32::BLUE
								} else {
									Color32::default()
								}),
							)
							.sense(egui::Sense::click())
							.ui(ui)
							.clicked()
							{
								if let Some(index) =
									self.breakpoints.iter().position(|&i| i == line_number + 1)
								{
									self.breakpoints.remove(index);
								} else {
									self.breakpoints.push(line_number + 1);
								}
							};
							if line.contains(";") {
								ui.label(
									egui::RichText::new(line)
										.color(Color32::DARK_GREEN)
										.monospace(),
								);
							} else if line.contains(":") {
								ui.label(
									egui::RichText::new(line)
										.color(Color32::LIGHT_BLUE)
										.monospace(),
								);
							} else {
								let leading_whitespace = line.len() - line.trim_start().len();
								ui.label(
									egui::RichText::new(&line[..leading_whitespace]).monospace(),
								);
								for (i, words) in line.split_whitespace().enumerate() {
									ui.label(
										egui::RichText::new(words)
											.monospace()
											.color(if i == 0 {
												Color32::KHAKI
											} else {
												Color32::GOLD
											})
											.background_color(
												if current_line_number == line_number {
													Color32::DARK_RED
												} else {
													Color32::default()
												},
											),
									);
								}
							}
						});
					})
			});
		});
		egui::Window::new("Watchpoints").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Address:");
				if ui
					.add(
						egui::TextEdit::singleline(&mut self.watchpoints_user_entry)
							.font(egui::TextStyle::Monospace)
							.desired_width(40.)
							.hint_text("in hex"),
					)
					.lost_focus() || ui.button("Add").clicked()
				{
					// TODO: handle the case with 0x as prefix
					if let Ok(line_number) = u16::from_str_radix(&self.watchpoints_user_entry, 16) {
						if !self.watchpoints.contains(&line_number) {
							self.watchpoints.push(line_number);
						}
					}
					self.watchpoints_user_entry.clear();
				}
			});
			let mut to_remove = Vec::new();
			for (i, &watchpoint) in self.watchpoints.iter().enumerate() {
				ui.horizontal(|ui| {
					let mut user_entry = format!("{:x}", mem.read_byte(watchpoint));
					ui.label(
						egui::RichText::new(format!("0x{watchpoint:04x}"))
							.monospace()
							.color(Color32::LIGHT_YELLOW),
					);
					if egui::TextEdit::singleline(&mut user_entry)
						.code_editor()
						.desired_width(30.)
						.interactive(self.paused)
						.ui(ui)
						.changed()
					{
						if let Ok(new_value) = u8::from_str_radix(&user_entry, 16) {
							mem.write_byte(watchpoint, new_value)
						}
					}
					if ui.button("X").clicked() {
						to_remove.push(i);
					}
				});
			}
			to_remove.iter().for_each(|i| {
				self.watchpoints.remove(*i);
			});
		});
		egui::Window::new("Breakpoints")
			.anchor(Align2::LEFT_BOTTOM, [10., -10.])
			.show(ctx, |ui| {
				ui.horizontal(|ui| {
					ui.label("Line number:");
					if ui
						.add(
							egui::TextEdit::singleline(&mut self.breakpoints_user_entry)
								.desired_width(40.),
						)
						.lost_focus() || ui.button("Add").clicked()
					{
						if let Ok(line_number) = self.breakpoints_user_entry.parse() {
							if !self.breakpoints.contains(&line_number) {
								self.breakpoints.push(line_number);
							}
						}
						self.breakpoints_user_entry.clear();
					}
				});
				let mut to_remove = Vec::new();
				for (i, &breakpoint) in self.breakpoints.iter().enumerate() {
					ui.horizontal(|ui| {
						ui.label(
							egui::RichText::new(format!("{breakpoint:>3}"))
								.monospace()
								.color(if breakpoint == current_line_number + 1 {
									Color32::LIGHT_RED
								} else {
									Color32::LIGHT_BLUE
								}),
						);
						if ui.button("X").clicked() {
							to_remove.push(i);
						}
					});
				}
				to_remove.iter().for_each(|i| {
					self.breakpoints.remove(*i);
				});
			});
		#[cfg(target_family = "wasm")]
		egui::Window::new("Help").show(ctx, |ui| {
			ui.label(egui::RichText::new(
				"1. Use the arrow or WASD keys to move the snake.",
			).color(Color32::LIGHT_GREEN));
			ui.label(egui::RichText::new(
				"2. Click 'Pause' button in 'Debug Controls' to pause execution.",
			).color(Color32::LIGHT_GREEN));
			ui.label(egui::RichText::new(
				"3. Click on the line numbers on the 'Source Code' window to toggle breakpoints.",
			).color(Color32::LIGHT_GREEN));
			ui.label("Source Code:");
			ui.hyperlink("https://github.com/ArchUsr64/6502_emulator/");
		});
		if self.breakpoints.contains(&(current_line_number + 1)) {
			self.paused = true;
		}
		self.breakpoints.sort_unstable();
		self.watchpoints.sort_unstable();
		self.break_address = self
			.breakpoints
			.iter()
			.map(|&i| self.debug_symbols[i - 1])
			.collect();
	}
}
