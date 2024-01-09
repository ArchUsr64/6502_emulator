use egui_macroquad::egui::{Align2, Color32};

use crate::{cpu, egui};

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
		}
	}
	pub fn render_ui(&mut self, ctx: &egui::Context, cpu: &cpu::Cpu) {
		self.step = false;
		egui::Window::new("Debug Controls").show(ctx, |ui| {
			if ui
				.add(egui::Button::new(if !self.paused {
					"Pause Execution"
				} else {
					"Resume Execution"
				}))
				.clicked()
			{
				self.paused = !self.paused;
			};
			if ui.add(egui::Button::new("Reset")).clicked() {
				self.reset = true
			};
			if self.paused {
				if ui.add(egui::Button::new("Step")).clicked() {
					self.step = true
				};
			}
		});
		if !self.paused {
			egui::Window::new("Simulation Speed").show(ctx, |ui| {
				ui.add(egui::Slider::new(
					&mut self.instructions_per_frame,
					1u32..=500,
				))
			});
		}
		if self.paused {
			let cpu_state = cpu.state();
			egui::Window::new("CPU State").show(ctx, |ui| {
				ui.add(egui::Label::new(format!(
					"Prgram Counter: 0x{:0x}",
					cpu_state.program_counter
				)));
				ui.add(egui::Label::new(format!(
					"Stack Pointer: 0x{:0x}",
					cpu_state.stack_pointer
				)));
				ui.add(egui::Label::new(format!(
					"Line Number: {}",
					if let Some(line_number) = self
						.debug_symbols
						.iter()
						.position(|&i| i == cpu_state.program_counter)
					{
						(line_number + 1).to_string()
					} else {
						"xxx".to_string()
					}
				)));
				ui.add(egui::Label::new("Registers:"));
				ui.label(
					egui::RichText::new(format!(
						"A: 0x{:02x}, X: 0x{:02x}, Y: 0x{:02x}",
						cpu_state.a, cpu_state.x, cpu_state.y
					))
					.color(Color32::LIGHT_RED),
				)
			});
		}
		egui::Window::new("Source Code")
			.anchor(Align2::RIGHT_TOP, [-10., 10.])
			.show(ctx, |ui| {
				egui::ScrollArea::vertical().show(ui, |ui| {
					ui.add(
						egui::TextEdit::multiline(
							&mut self
								.source_file
								.iter()
								.enumerate()
								.map(|(line_number, line)| format!("{}:\t{line}", line_number + 1))
								.collect::<Vec<_>>()
								.join("\n"),
						)
						.text_color(Color32::YELLOW)
						.desired_width(f32::INFINITY)
						.desired_rows(40)
						.clip_text(true)
						.interactive(false)
						.font(egui::TextStyle::Monospace),
					);
				})
			});
		egui::Window::new("Breakpoints").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Line number:");
				ui.add(
					egui::TextEdit::singleline(&mut self.breakpoints_user_entry).desired_width(40.),
				);

				if ui.button("Add").clicked() {
					if let Ok(line_number) = self.breakpoints_user_entry.parse() {
						if !self.breakpoints.contains(&line_number) {
							self.breakpoints.push(line_number);
						}
					}
					self.breakpoints_user_entry.clear();
				}
			});
			let mut to_remove = Vec::new();
			for (i, breakpoint) in self.breakpoints.iter().enumerate() {
				ui.horizontal(|ui| {
					ui.label(format!("{breakpoint}"));
					if ui.button("X").clicked() {
						to_remove.push(i);
					}
				});
			}
			to_remove.iter().for_each(|i| {
				self.breakpoints.remove(*i);
			});
		});
		if let Some(line_number) = self
			.debug_symbols
			.iter()
			.position(|&i| i == cpu.state().program_counter)
		{
			if self.breakpoints.contains(&(line_number + 1)) {
				self.paused = true;
			}
		}
	}
}
