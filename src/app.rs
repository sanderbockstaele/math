use egui::*;
use epi::*;

// ------------ App ---------------------------------------------

#[cfg_attr(feature = "persistence", dervive(serde::Desirialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct App {
	window: AppWindow,
}

impl Default for App {
	fn default() -> Self {
		Self {
			window: AppWindow::default()
		}
	}
}

impl epi::App for App {
	fn name(&self) -> &str{
		"Math"
	}

	fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>){
		self.window.ui(ctx);
	}
}

// ---------------- AppWindow --------------------
struct AppWindow {
	history: Vec<String>,
	equation_input: String,
}

impl Default for AppWindow {
	fn default() -> Self {
		Self {
			history: vec![],
			equation_input: "".to_string(),
		}
	}
}

impl AppWindow {
	fn ui(&mut self, ctx: &CtxRef) {
		egui::SidePanel::left("side_panel", 500.0).show(ctx, |ui| {
			ui.heading("Equations");

			ui.separator();

			egui::ScrollArea::auto_sized().show( ui, |ui| {
				let mut current_equation : usize = 1;
				// iterate over the history and for each equation create a radian button
				for equation in self.history.iter() {
					ui.add(egui::RadioButton::new(false, equation));
					current_equation = current_equation + 1;
				}
			});

			ui.horizontal(|ui| {
				ui.add(egui::TextEdit::singleline(&mut self.equation_input).hint_text("write your equation"));


				if ui.button("Add").clicked() {
					// this is verbose: unborrow equation and cast it to a String
					self.history.push((*self.equation_input).to_string());
				}
			});
		});
	}
}
