use crate::{cpu, egui};

pub struct App {
	pub paused: bool,
	pub step: bool,
}

impl Default for App {
	fn default() -> Self {
		Self {
			step: false,
			paused: false,
		}
	}
}

impl App {
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
			if self.paused {
				if ui.add(egui::Button::new("Step")).clicked() {
					self.step = true
				};
			}
		});
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
				ui.add(egui::Label::new("Registers:"));
				ui.add(egui::Label::new(format!(
					"A: 0x{:2x}, X: 0x{:2x}, Y: 0x{:2x}",
					cpu_state.a, cpu_state.x, cpu_state.y
				)));
			});
		}
	}
}
