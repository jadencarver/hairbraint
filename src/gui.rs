use actix::prelude::*;
use gtk::prelude::*;
use gtk::{Application, Inhibit};
use gtk::gdk::RGBA;

pub fn main() {
    gtk::init();
    let builder = gtk::Builder::from_string(include_str!("gui.ui"));
    let app: gtk::Application = builder.application()
        .unwrap_or(Application::builder()
                   .application_id("net.hairbraint.Hairbraint")
                   .build());
    let main: gtk::ApplicationWindow = builder.object("main").unwrap();
    let calendar: gtk::Expander = builder.object("calendar").unwrap();
    let scheduler: gtk::Stack = builder.object("scheduler").unwrap();
    let check: gtk::Frame = builder.object("check").unwrap();
    let book: gtk::Frame = builder.object("book").unwrap();
    let preview: gtk::DrawingArea = builder.object("preview").unwrap();
    let day: gtk::DrawingArea = builder.object("day_area").unwrap();

    let incoming: gtk::ListBox = builder.object("incoming").unwrap();
    //let appointments: gtk::ListStore = builder.object("appointments").expect("gui.ui to define appointments");
    //incoming.bind_model(Some(&appointments.upcast::<gtk::gio::ListStore>()), |appt| { gtk::Label::new(Some("asdf")).upcast::<gtk::Widget>() });

    preview.connect_draw(move |preview, cairo| {
        let width = preview.allocated_width() as f64 - 20.0;
        let color = preview.style_context().color(gtk::StateFlags::NORMAL);
        let color_em = preview.style_context().color(gtk::StateFlags::LINK);
        let color_bg = RGBA::new(1.0,1.0,1.0,1.0);
        let scale = 1.5;

        cairo.set_source_rgb(color_bg.red(), color_bg.green(), color_bg.blue());
        cairo.rectangle(5.0, 10.0 * scale, width, 30.0 * scale);
        cairo.rectangle(5.0, 85.0 * scale, width, 45.0 * scale);
        cairo.fill().unwrap();

        cairo.set_source_rgb(color_em.red(), color_em.green(), color_em.blue());
        cairo.rectangle(5.0, 10.0 * scale, 12.0, 30.0 * scale);
        cairo.rectangle(5.0, 85.0 * scale, 12.0, 45.0 * scale);
        cairo.fill().unwrap();

        cairo.set_line_width(1.5);
        cairo.set_source_rgb(color.red(), color.green(), color.blue());
        cairo.rectangle(5.0, 10.0 * scale, width, 30.0 * scale);
        cairo.rectangle(5.0, 85.0 * scale, width, 45.0 * scale);
        cairo.stroke().unwrap();

        cairo.set_font_size(12.0);
        cairo.move_to(20.0, 20.0 * scale);
        cairo.show_text("Single Process Color");
        cairo.move_to(20.0, 32.0 * scale);
        cairo.show_text("30 min");
        cairo.move_to(20.0, 95.0 * scale);
        cairo.show_text("Haircut");
        cairo.move_to(20.0, 108.0 * scale);
        cairo.show_text("45 min");

        Inhibit(false)
    });

    day.connect_draw(move |day, cairo| {
        let width = (day.allocated_width() as f64 - 20.0) / 5.0;
        let color = day.style_context().color(gtk::StateFlags::NORMAL);
        let color_em = day.style_context().color(gtk::StateFlags::LINK);
        let color_bg = RGBA::new(1.0,1.0,1.0,1.0);
        let scale = 1.5;

        for i in 0..5 {
            let i = i as f64;
            cairo.set_source_rgb(color_bg.red(), color_bg.green(), color_bg.blue());
            cairo.rectangle(5.0 + width * i, 10.0 * scale, width - 5.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * i, 85.0 * scale, width - 5.0, 45.0 * scale);
            cairo.fill().unwrap();

            cairo.set_source_rgb(color_em.red(), color_em.green(), color_em.blue());
            cairo.rectangle(5.0 + width * i, 10.0 * scale, 12.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * i, 85.0 * scale, 12.0, 45.0 * scale);
            cairo.fill().unwrap();

            cairo.set_line_width(1.5);
            cairo.set_source_rgb(color.red(), color.green(), color.blue());
            cairo.rectangle(5.0 + width * i, 10.0 * scale, width - 5.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * i, 85.0 * scale, width - 5.0, 45.0 * scale);
            cairo.stroke().unwrap();

            cairo.set_font_size(12.0);
            cairo.move_to(20.0 + width * i, 20.0 * scale);
            cairo.show_text("Single Process Color");
            cairo.move_to(20.0 + width * i, 32.0 * scale);
            cairo.show_text("30 min");
            cairo.move_to(20.0 + width * i, 95.0 * scale);
            cairo.show_text("Haircut");
            cairo.move_to(20.0 + width * i, 108.0 * scale);
            cairo.show_text("45 min");
        }

        Inhibit(false)
    });

    app.connect_activate(move |app| {
        let scheduler = scheduler.clone();
        let book = book.clone();
        let check = check.clone();

        calendar.connect_activate(move |cal| {
            if cal.is_expanded() {
                scheduler.set_visible_child(&check);
            } else {
                scheduler.set_visible_child(&book);
            }
        });
        //main.connect_close(|main| {
        //    app.quit();
        //});
        main.show_all();
    });

    app.run();
    gtk::main();
}
