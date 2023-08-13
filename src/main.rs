extern crate diesel;

use eframe::egui;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

//// Diesel setup
//mod schema {
//    table! {
//        users (id) {
//            id -> Integer,
//            name -> Text,
//            age -> Integer,
//        }
//    }
//}
//
//#[derive(Debug, Queryable)]
//struct User {
//    id: i32,
//    name: String,
//    age: i32,
//}

fn establish_connection() -> SqliteConnection {
    let database_url = "database.sqlite";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Debug, Queryable)]
struct Contact {
    name: String,
    uri: String,
    icon_id: Option<egui::TextureId>
}

enum AppState {
    Relating,
    Scheduling,
    Transaction
}

struct App {
    state: AppState,
    query: String,
    contacts: Vec<Contact>,
    contact: Option<usize>,
}

impl Contact {
    pub fn new() -> Contact {
        Contact {
            name: String::from("Jabez Carver"),
            uri: String::from("Dyslexic version of me"),
            icon_id: None
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
                ui.label(&self.uri);
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Hairbraint", native_options, Box::new(|ctx| Box::new(App::new())))
}

impl App {
    fn new() -> App {
        App {
            state: AppState::Scheduling,
            query: String::new(),
            contacts: vec![
                Contact::new(), Contact::new(), Contact::new(), Contact::new(), Contact::new(),
                Contact::new(), Contact::new(), Contact::new(), Contact::new(), Contact::new(),
                Contact::new(), Contact::new(), Contact::new(), Contact::new(), Contact::new(),
                Contact::new(), Contact::new(), Contact::new(), Contact::new(), Contact::new(),
                Contact::new(), Contact::new(), Contact::new(), Contact::new(), Contact::new(),
            ],
            contact: None
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left Panel
        egui::SidePanel::left("in").show(ctx, |ui| {
            // Search
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.query);
                if ui.button("🔍").clicked() {
                    // Handle search logic here
                }
            });

            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
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
                            for i in 0..10 {
                                ui.vertical(|ui| {
                                    ui.add_space(25.0);
                                    for j in 0..10 {
                                        ui.add(egui::Button::new("Blowdry").min_size(egui::vec2(150.0,75.0)));
                                    }
                                });

                            }
                        });
                    });
                    let painter = ui.painter();
                    let offset_x = scroll_area.state.offset.x;
                    let offset_y = scroll_area.state.offset.y;
                    for i in 0..10 {
                        let pos = egui::pos2((i * 158) as f32 + ui.min_rect().min.x + 75.0 - scroll_area.state.offset.x, ui.min_rect().min.y);
                        painter.rect_filled(egui::Rect::from_min_size(pos, egui::vec2(150.0, 20.0)), egui::Rounding::none(), ui.visuals().window_fill);
                        painter.text(pos, egui::Align2::LEFT_TOP, "Jay Bez Ness", egui::FontId::proportional(14.0), ui.visuals().text_color());
                    }
                    for j in 0..24*4 {
                        let pos = egui::pos2(ui.min_rect().min.x, (j * 25) as f32 + ui.min_rect().min.y + 25.0 - scroll_area.state.offset.y);
                        painter.rect_filled(egui::Rect::from_min_size(pos, egui::vec2(75.0, 25.0)), egui::Rounding::none(), ui.visuals().window_fill);
                        painter.text(pos, egui::Align2::LEFT_TOP, format!("{}:{:02} PM", j / 4, j % 4 * 15), egui::FontId::proportional(14.0), ui.visuals().text_color());
                    }

                },
                AppState::Transaction => { ui.heading("Transaction"); }
            };
        });

        // Right Panel
        //egui::SidePanel::right("out").show(ctx, |ui| {
        //    ui.label("Check out");
        //    // Add widgets here for the right panel
        //});
    }
}
