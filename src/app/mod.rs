use eframe::egui;

enum State {
	Started,
	Normal,
	Modal(String, bool)
}

pub struct App {
    db: sled::Db,
	last_state: State,
	state: State
}

impl App {
    pub fn new() -> App {
        let db = sled::open("target/data").expect("database failure");
        App { db: db, last_state: State::Started, state: State::Normal }
    }

	pub fn title(&self) -> String {
		self.db.get("title").ok().and_then(|result| result)
			.map(|title| String::from_utf8_lossy(&title.to_vec()).to_string())
			.unwrap_or_else(|| String::from("Hairbraint"))
	}

	pub fn customer(&self, id: u8) -> Option<u8> {
		let db = self.db.open_tree("customer").expect("Customer tree error");
		if let Ok(result) = db.get(vec![id]) {
			Some(result.unwrap()[0])
		} else { None }
	}

}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if let State::Modal(message_, cancel) = &self.state {
			let message = message_.clone();
			egui::Window::new("Modal Window").collapsible(false).resizable(false).show(ctx, |ui| {
				ui.label(message);
				if ui.button("OK").clicked() {
					self.state = State::Normal;
				}
			});
		}
        egui::SidePanel::left("in").show(ctx, |ui| {});
        egui::SidePanel::right("out").show(ctx, |ui| {});
        egui::TopBottomPanel::top("planner").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let date = chrono::Local::now();
                ui.heading(format!("{}", date.format("%x %r")));
            });
        });
		egui::CentralPanel::default().show(ctx, |ui| {
			let scroll_area = egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
				ui.horizontal(|ui| {
					ui.add_space(75.0);
					for i in 0..20 {
						ui.vertical(|ui| {
							ui.add_space(25.0);
							for j in 0..16 {
								if (i + j) % 4 < 3 { ui.add_space(75.0); }
								let service = egui::Button::new(["Blowdry", "Haircut", "Highlight", "Root Color", "Gloss"][(i + j) % 5])
									.min_size(egui::vec2(150.0, [75.0, 75.0, 100.0, 50.0, 25.0][(i + j) % 5]))
									.fill([egui::Color32::LIGHT_BLUE, egui::Color32::LIGHT_GRAY, egui::Color32::LIGHT_RED][(i + j) % 3]);
								let button = ui.add(service);
								if button.clicked() {
									self.state = State::Modal(String::from("Hello"), false);
									println!("{:?}", button);
								}
							}
						});

					}
				});
			});
			let painter = ui.painter_at(scroll_area.inner_rect.expand(3.0));
			let _offset_x = scroll_area.state.offset.x;
			let _offset_y = scroll_area.state.offset.y;
			let bg_color = ui.visuals().widgets.noninteractive.bg_fill;
			//let bg_color = egui::Color32::DEBUG_COLOR;
			//let bg_color = egui::Color32::from_rgba_premultiplied(64, 64, 64, 128);
			painter.rect_filled(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(ui.min_rect().max.x, 25.0)), egui::Rounding::ZERO, bg_color);
			painter.rect_filled(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(ui.min_rect().min.x + 65.0, ui.min_rect().max.y)), egui::Rounding::ZERO, bg_color);
			for i in 0..20 {
				let pos = egui::pos2((i * 158) as f32 + ui.min_rect().min.x + 75.0 - scroll_area.state.offset.x, ui.min_rect().min.y);
				painter.text(pos, egui::Align2::LEFT_TOP, ["Jabez Carver", "Jaden Carver", "Charlie Chappy", "Boardy Board"][i % 4], egui::FontId::proportional(14.0), ui.visuals().text_color());
			}
			for j in (0..24*4).step_by(2) {
				let pos_text = egui::pos2(ui.min_rect().min.x + 60.0, (j * 25) as f32 + ui.min_rect().min.y + 25.0 - scroll_area.state.offset.y);
				painter.text(pos_text, egui::Align2::RIGHT_TOP, format!("{}:{:02} PM", j / 4, j % 4 * 15), egui::FontId::proportional(14.0), ui.visuals().text_color());
				let pos_bar = egui::pos2(ui.min_rect().min.x + 30.0, (j * 25) as f32 + ui.min_rect().min.y + 55.0 - scroll_area.state.offset.y);
				painter.rect_filled(egui::Rect::from_min_size(pos_bar, egui::vec2(30.0, 1.0)), egui::Rounding::ZERO, ui.visuals().text_color());
			}
		});
    }
}
