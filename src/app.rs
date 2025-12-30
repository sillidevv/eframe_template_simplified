/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
	label: String,

	#[serde(skip)] // opt-out of serialization
	value: f32,
}

impl Default for TemplateApp {
	fn default() -> Self {
		Self {
			label: "Hello World!".to_owned(),
			value: 2.7,
		}
	}
}

impl TemplateApp {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		if let Some(storage) = cc.storage {
			eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
		} else {
			Self::default()
		}
	}
}

impl eframe::App for TemplateApp {
	/// Save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	/// Called each frame.
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::MenuBar::new().ui(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						ctx.send_viewport_cmd(egui::ViewportCommand::Close);
					}
				});

				ui.add_space(16.0);
				egui::widgets::global_theme_preference_buttons(ui);
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("eframe template");

			ui.horizontal(|ui| {
				ui.label("Write something:");
				ui.text_edit_singleline(&mut self.label);
			});

			ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

			if ui.button("Increment").clicked() {
				self.value += 1.0;
			}

			ui.separator();

			ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
				powered_by_egui_and_eframe(ui);
				egui::warn_if_debug_build(ui);
			});
		});
	}
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
	ui.horizontal(|ui| {
		ui.spacing_mut().item_spacing.x = 0.0;
		ui.label("Powered by ");
		ui.hyperlink_to("egui", "https://github.com/emilk/egui");
		ui.label(" and ");
		ui.hyperlink_to(
			"eframe",
			"https://github.com/emilk/egui/tree/master/crates/eframe",
		);
		ui.label(".");
	});
}
