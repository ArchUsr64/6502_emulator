use super::egui;

pub struct App {
	pub paused: bool,
}

impl App {
	pub fn new() -> Self {
		Self { paused: false }
	}
	pub fn render_ui(&mut self, ctx: &egui::Context) {
		egui::Window::new("Debug Controls").show(ctx, |ui| {
			if ui.add(egui::Button::new("Pause Execution")).clicked() {
				self.paused = !self.paused;
			};
		});
	}
}
