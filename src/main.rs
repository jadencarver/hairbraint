extern crate diesel;

use eframe::egui;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use current_locale::current_locale;
use chrono::Local;

pub mod schema;
pub mod models;

use self::models::{Ash, AsChange};

#[derive(PartialEq)]
enum AppState {
    Relating,
    Scheduling,
    Transaction,
    Advanced
}

struct App {
    ante: Ash,
    focus: Ash,
    state: AppState,
    resource: bool,
    query: String,
    db: SqliteConnection
}

fn main() -> eframe::Result<()> {
    let mut app = App::new();
    let title = app.title();

    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::vec2(1920.0, 1080.0));

    eframe::run_native(&title, options, Box::new(|_ctx| Box::new(app)))
}

impl App {
    fn new() -> App {
        use schema::ashes::dsl::{ashes, ash};

        let database_url = "database.sqlite";
        let mut db = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

        //let lang = current_locale().unwrap_or(String::from("en"));
        let lang = String::from("en");
        let ante = ashes.filter(ash.eq(format!("lang.{}", lang))).first::<Ash>(&mut db).unwrap_or(
            ashes.filter(ash.eq("lang.en")).first::<Ash>(&mut db).expect("missing lang.en antecedent")
        );

        App {
            ante: ante.clone(), focus: ante,
            resource: false,
            state: AppState::Scheduling,
            query: String::new(),
            db: db
        }
    }

    fn title(&mut self) -> String {
        "Hairbraint".into()
    }

    //fn lookup(mut self, ash: &Ash, to: &Ash) -> AsChange {
    //    use schema::aschanges::dsl::{aschanges, ash_id, ante_id, product_id};
    //    aschanges.filter(ante_id.eq(self.ante.id)).filter(ash_id.eq(ash.id)).filter(product_id.eq(to.id)).first::<AsChange>(&mut self.db).unwrap()
    //}
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Left Panel
        egui::SidePanel::left("in").show(ctx, |ui| {

            ui.label(&self.ante.ash);
            ui.label(&self.focus.ash);

            // Search
            if ui.text_edit_singleline(&mut self.query).gained_focus() {
                use schema::ashes::dsl::{ashes, ash};
                let lang = String::from("en");
                self.ante = ashes.filter(ash.eq(format!("lang.{}", lang))).first::<Ash>(&mut self.db).unwrap();
            };
            egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                use schema::aschanges::dsl::{aschanges, ante_id};

                let results = aschanges.filter(ante_id.eq_any([self.ante.id, self.focus.id])).load::<AsChange>(&mut self.db).unwrap();
                for result in results {
                    let target = result.ash(&mut self.db);
                    let label = target.ash.clone();
                    ui.selectable_value(&mut self.focus, target, label);
                }

            });

        });

        egui::TopBottomPanel::top("planner").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let date = Local::now();
                ui.heading(format!("{}",date.format("%c")));
                ui.selectable_value(&mut self.state, AppState::Relating, "Relating");
                ui.selectable_value(&mut self.state, AppState::Scheduling, "Scheduling");
                ui.selectable_value(&mut self.state, AppState::Transaction, "Transaction");
                ui.selectable_value(&mut self.state, AppState::Advanced, "Advanced");
            });
        });

        // Central Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Add widgets here for the central panel
            match self.state {
                AppState::Relating => {

                    egui::Window::new("New Relationship").show(ctx, |ui| {
                        egui::Grid::new("relationship").num_columns(2).show(ui, |ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut self.query);
                            ui.end_row();
                            ui.label("Phone");
                            ui.text_edit_singleline(&mut self.query);
                            ui.end_row();
                            ui.label("E-Mail");
                            ui.text_edit_singleline(&mut self.query);
                            ui.end_row();
                            ui.label("Notes");
                            ui.text_edit_multiline(&mut self.query);
                            ui.end_row();
                        });
                    });

                    egui::Window::new("Another Relationship").show(ctx, |ui| {
                        egui::Grid::new("relationship").num_columns(2).show(ui, |ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut self.query);
                            ui.end_row();
                            ui.label("Phone");
                            ui.text_edit_singleline(&mut self.query);
                            ui.end_row();
                            ui.label("E-Mail");
                            ui.text_edit_singleline(&mut self.query);
                            ui.end_row();
                            ui.label("Notes");
                            ui.text_edit_multiline(&mut self.query);
                            ui.end_row();
                        });
                    });

                }
                AppState::Scheduling => {
                    let scroll_area = egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(75.0);
                            for i in 0..20 {
                                ui.vertical(|ui| {
                                    ui.add_space(25.0);
                                    for j in 0..16 {
                                        if (i + j) % 4 < 3 { ui.add_space(75.0); }
                                        let service = egui::Button::new(["Blowdry", "Haircut", "Highlight", "Root Color", "Gloss"][(i + j) % 5])
                                            .min_size(egui::vec2(150.0, [75.0, 75.0, 100.0, 50.0, 25.0][(i + j) % 5]));
                                            //.fill([egui::Color32::LIGHT_BLUE, egui::Color32::LIGHT_GRAY, egui::Color32::LIGHT_RED][(i + j) % 3]);
                                        let button = ui.add(service);
                                        if button.clicked() {
                                            self.resource = true;
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
                    painter.rect_filled(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(ui.min_rect().max.x, 25.0)), egui::Rounding::none(), bg_color);
                    painter.rect_filled(egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(ui.min_rect().min.x + 65.0, ui.min_rect().max.y)), egui::Rounding::none(), bg_color);
                    for i in 0..20 {
                        let pos = egui::pos2((i * 158) as f32 + ui.min_rect().min.x + 75.0 - scroll_area.state.offset.x, ui.min_rect().min.y);
                        painter.text(pos, egui::Align2::LEFT_TOP, ["Jabez Carver", "Jaden Carver", "Charlie Chappy", "Boardy Board"][i % 4], egui::FontId::proportional(14.0), ui.visuals().text_color());
                    }
                    for j in (0..24*4).step_by(2) {
                        let pos_text = egui::pos2(ui.min_rect().min.x + 60.0, (j * 25) as f32 + ui.min_rect().min.y + 25.0 - scroll_area.state.offset.y);
                        painter.text(pos_text, egui::Align2::RIGHT_TOP, format!("{}:{:02} PM", j / 4, j % 4 * 15), egui::FontId::proportional(14.0), ui.visuals().text_color());
                        let pos_bar = egui::pos2(ui.min_rect().min.x + 30.0, (j * 25) as f32 + ui.min_rect().min.y + 55.0 - scroll_area.state.offset.y);
                        painter.rect_filled(egui::Rect::from_min_size(pos_bar, egui::vec2(30.0, 3.0)), egui::Rounding::none(), ui.visuals().text_color());
                    }

                },
                AppState::Transaction => {}
                AppState::Advanced => {
                    ui.heading("Advanced");

                    use schema::ashes::dsl::{ashes};
                    use schema::aschanges::dsl::{aschanges, ash_id};
                    let results = ashes.load::<Ash>(&mut self.db).unwrap();

                    egui::Grid::new("ashes2ashes").min_col_width(10.0).num_columns(5).show(ui, |ui| {
                        ui.label("ante");
                        ui.add_sized([150.0, 15.0], egui::Label::new("ash"));
                        ui.label("Σ");
                        ui.label("product");
                        //ui.label("rate");
                        ui.label("alias");
                        ui.end_row();

                        for result in results.iter() {
                            let changes = aschanges.filter(ash_id.eq(result.id)).load::<AsChange>(&mut self.db);
                            for change in changes.unwrap() {
                                let ante = ashes.find(change.ante_id).first::<Ash>(&mut self.db).unwrap();
                                let product = ashes.find(change.product_id).first::<Ash>(&mut self.db).unwrap();
                                let mut alias = change.alias.unwrap_or("None".into());
                                let mut ash = result.ash.clone();
                                let mut selected: i32;
                                selected = 1;

                                ui.label(ante.ash);
                                ui.text_edit_singleline(&mut ash);
                                ui.label(format!("{}",change.sigma));

                                egui::ComboBox::from_id_source(change.id)
                                    .selected_text(product.ash)
                                    .show_ui(ui, |ui| {
                                        for product in results.iter() {
                                            ui.selectable_value(&mut selected, product.id, product.ash.clone());
                                        }
                                    });

                                //ui.label(format!("{:?}", change.rate));
                                ui.text_edit_singleline(&mut alias);
                                ui.end_row();
                            }
                        }
                    });

                }
            };
        });

        if self.resource {

            // Right Panel
            egui::SidePanel::right("out").min_width(300.0).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Resources");
                    ui.add_space(ui.available_width() - 25.0);
                    let close = ui.add_sized([20.0, 20.0], egui::Button::new("❌").rounding(10.0));
                    if close.clicked() {
                        self.resource = false;
                    };
                });
                ui.separator();
            });

        }

    }
}
