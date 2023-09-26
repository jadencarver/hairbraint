extern crate diesel;

use eframe::egui;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use current_locale::current_locale;

pub mod schema;
pub mod models;

use self::models::{Ash, AsChange};

struct Contact {
    name: String,
    uri: String,
    service: String,
    time: String,
    icon_id: Option<egui::TextureId>
}

enum AppState {
    Relating,
    Scheduling,
    Transaction
}

struct App {
    ante: Ash,
    db: SqliteConnection,
    state: AppState,
    query: String,
    contacts: Vec<Contact>,
    contact: Option<usize>,
}

impl Contact {
    pub fn new(i: usize) -> Contact {
        Contact {
            name: String::from(["Jabez Carver", "Jaden Carver", "Charlie Chappy", "Boardy Board"][i % 4]),
            uri: String::from(["Biblical King", "Dyslexic King", "All Dogs Go To Heaven", "Monopoly Man"][i % 4]),
            icon_id: None,
            service: String::from(["Haircut", "Highlight", "Shine Coat", "Single Process Color"][i % 4]),
            time: String::from(format!("{}:{:02} PM", i / 4, i % 4 * 15))
        }
    }

    pub fn display(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.set_min_size(egui::vec2(22.0, 22.0));
                ui.label("IMG");
            });
            ui.vertical(|ui| {
                ui.strong(&self.name);
                ui.label(&self.service);
            });
            ui.strong(&self.time);
        });
    }
}

fn main() -> eframe::Result<()> {
    let mut app = App::new();
    let title = app.title();

    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::vec2(1920.0, 1080.0));

    eframe::run_native(&title, options, Box::new(|ctx| Box::new(app)))
}

impl App {
    fn new() -> App {
        use schema::ashes::dsl::{ashes, ash};

        let database_url = "database.sqlite";
        let mut db = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

        let lang = current_locale().unwrap_or(String::from("en"));
        let ante = ashes.filter(ash.eq(format!("lang.{}", lang))).first::<Ash>(&mut db).unwrap_or(
            ashes.filter(ash.eq("lang.en")).first::<Ash>(&mut db).expect("missing lang.en antecedent")
        );

        App {
            ante: ante,
            state: AppState::Scheduling,
            query: String::new(),
            contacts: (0..100).map(|i| { Contact::new(i) }).collect(),
            contact: None,
            db: db
        }
    }

    fn title(&mut self) -> String {
        use schema::ashes::dsl::{ashes, ash};
        use schema::aschanges::dsl::{aschanges, ash_id};
        let title = ashes.filter(ash.eq("my.title")).first::<Ash>(&mut self.db).unwrap();
        let localized = aschanges.filter(ash_id.eq(title.id)).first::<AsChange>(&mut self.db);
        localized.unwrap().alias.unwrap()
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
            // Search
            ui.text_edit_singleline(&mut self.query);
            egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                for (i, contact) in self.contacts.iter().enumerate() {
                    contact.display(ui);
                }
            });

        });

        // Central Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // Add widgets here for the central panel
            match self.state {
                AppState::Relating => { ui.heading("Relationship"); }
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
                                    }
                                });

                            }
                        });
                    });
                    let painter = ui.painter_at(scroll_area.inner_rect.expand(3.0));
                    let offset_x = scroll_area.state.offset.x;
                    let offset_y = scroll_area.state.offset.y;
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
                AppState::Transaction => { ui.heading("Transaction"); }
            };
        });

        // Right Panel
        egui::SidePanel::right("out").min_width(200.0).show(ctx, |ui| {
            ui.heading("Check out");
            // Add widgets here for the right panel
            
            use schema::ashes::dsl::{ashes, ash};
            use schema::aschanges::dsl::{aschanges, ash_id};
            let results = ashes.load::<Ash>(&mut self.db).unwrap();

            egui::Grid::new("some_unique_id").min_col_width(10.0).num_columns(3).show(ui, |ui| {
                //ui.label("ash");
                //ui.label("ante");
                ui.label("Σ");
                ui.label("product");
                ui.label("alias");
                //ui.label("rate");
                ui.end_row();

                for result in results {
                    let changes = aschanges.filter(ash_id.eq(result.id)).load::<AsChange>(&mut self.db);
                    for change in changes.unwrap() {
                        let ante = ashes.find(change.ante_id).first::<Ash>(&mut self.db).unwrap();
                        let product = ashes.find(change.product_id).first::<Ash>(&mut self.db).unwrap();
                        let mut alias = change.alias.unwrap_or("None".into());
                        //ui.label(&result.ash);
                        //ui.label(ante.ash);
                        ui.label(format!("{}",change.sigma));
                        ui.label(product.ash);
                        ui.text_edit_singleline(&mut alias);
                        //ui.label(format!("{:?}", change.rate));
                        ui.end_row();
                    }
                }
            });
        });
    }
}
