extern crate diesel;

use eframe::egui;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

//// Diesel setup
//mod schema {
//    table! {
//        // I need to be able to put a 0 into any _id field and have a corresponding lookup that
//        // allows me to generalize any identifier as a uuid/uri or human-readable.  Basically
//        // the same as a BLOB storage but shorter.  Whenever I run into ID=0, I know I could
//        // incorporate external identifiers, or declare them as externally represented
//        //
//        //textual_id {
//        //  table -> String,
//        //  field -> String, (or "table.field")
//        //
//        //  value -> String, // presume ("human readable value")
//        //  uuid -> String, // for server-side network identifiers
//        //  uri -> String, // human-lite, ala limited but semi-standardized "sms://123-123-1234" or
//        //                 // "http://asdf.com/asdf" could host data that we could fetch
//        //  these three values kind of scale where 'value' is most generic/useless, uuid is
//        //  most specific but also less useful, or a uri which kind of in the middle
//        //}
//        //
//        // this enables me to reference things that may not be resolved or resolvable immediately
//        // after a sync such as a new provider that doesn't have a corresponding relationship.
//        // Rather than a user_id like "123", I might have "user" like "mail://myname@asdf.com" that
//        // I could possibly fetch from the internet in order to locally construct such a user, in
//        // which case I could then generate an ID and use that instead.
//        //
//        // It's kind of irrelevant as if it doesn't contain a URI it will be obvious...
//
//        openings (id) {
//            id -> Integer,
//            closing_id -> Integer,
//            time -> Timestamp, // (start time)
//            duration -> Duration, // use MAX(duration) to optimize lookups by time
//            provider_id -> Integer,
//            reason -> String, // is this scheduled time on?
//        }
//        closings (id) {
//            id -> Integer,
//            opening_id -> Integer,
//            time -> Timestamp, // (start time)
//            duration -> Duration,
//            provider_id -> Integer,
//            consumer_id -> Integer, // (only closings)
//        }
//        //// rules end up being client side validation concerns (ala irrelevant for now)
//        //rules (id) {
//        //    id -> Integer,
//        //    //scope -> (indicates the severity of the inconsistency)
//        //    //         (defunct by) derived from the depth of the provider(*1) in it's heirarchy
//        //    provider_id -> Integer,
//        //    application -> Enum, // { openings/closings }
//        //    subject -> String, // SQL fragment like "time + duration"
//        //    validation -> Enum { },
//        //}
//
//        interactions(id) {
//            uuid -> String,
//            parent_id -> Option<Integer>,
//            regards_id -> Option<Integer>,
//            provider_id -> Integer,
//            consumer_id -> Integer,
//            service_id -> Option<Integer>,
//            rule_id -> Option<Integer>,
//            ref_uri -> Option<String>
//        }
//
//        interaction_states {
//            interaction_id -> Integer,
//            property -> String,
//            value -> String,
//        }
//        
//        // it's possible there is another inversion here happening between providers and services.
//        // in this inversion the services make inferences into the providers to establish which
//        // providers are able to perform them.  This aligns to the generative behavior of
//        // 'openings'.  These probabilities also must account for inventory and such rules that
//        // apply to them.
//        // inversely, providers declare which services they are willing to do and under what
//        // conditions.
//        providers { // heirarchy
//            parent_id -> Integer,
//            name -> String
//            // (null) (scope = 0) [imposed by the application] *1
//            // Scott J (scope = 1)
//            // \- Nichole (scope = 2)
//            //   \- Keyiana (scope = 3)
//            //   \- Rafi (scope = 3)
//        }
//        services {
//            provider_id -> Integer,
//            shortcode -> String, // like "HH" or "CSP" for search lookup
//            name -> String
//        }
//    }
//}

// consider that different services may allow for different rules
// providers may have their own set of rules
// companies may have rules
// the universe may have rules (what is physically possible, like traveling faster than light)

// Rule examples
// - No Double Booking = No overlapping 'start' times for the duration for a provider
//     Rule { subject (time+duration) }
// - No color services in the evenings for senior stylists
// - Everybody must (attempt) at least 2 blowdries a day

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
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::vec2(1920.0, 1080.0));
    eframe::run_native("Hairbraint", options, Box::new(|ctx| Box::new(App::new())))
}

impl App {
    fn new() -> App {
        App {
            state: AppState::Scheduling,
            query: String::new(),
            contacts: (0..100).map(|i| { Contact::new(i) }).collect(),
            contact: None
        }
    }
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
                    let painter = ui.painter_at(scroll_area.rect);
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
        egui::SidePanel::right("out").show(ctx, |ui| {
            ui.heading("Check out");
            // Add widgets here for the right panel
        });
    }
}
