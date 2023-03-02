use actix::prelude::*;
use gtk::prelude::*;
use gtk::{Application, Inhibit};
use gtk::gdk::RGBA;
use glib::{Date, DateTime};
use core::time::Duration;
use gtk::cairo::{FontSlant, FontWeight, Rectangle};

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
    let timeline: gtk::DrawingArea = builder.object("timeline_area").unwrap();
    let timeline_viewport: gtk::Viewport = builder.object("timeline_viewport").unwrap();
    let providers: gtk::Box = builder.object("providers_box").unwrap();
    let providers_viewport: gtk::Viewport = builder.object("providers_viewport").unwrap();

    let scale = 1.25;

    let stylists = vec![
        ("Julissa", "#alert", 135.0),
        ("Susan", "#available", 75.0),
        ("Richard", "#alert", 135.0),
        ("Charlie", "#unavailable", 45.0),
        ("Chloe", "#unavailable", 45.0),
        ("Juliet", "#late", 0.0),
        ("Laurell", "#unavailable", 45.0),
        ("Oaxana", "#unavailable", 45.0),
        ("Mikaela", "#unavailable", 45.0),
        ("Romeo", "#missing", 90.0)
    ];

    let incoming: gtk::ListBox = builder.object("incoming").unwrap();
    //let appointments: gtk::ListStore = builder.object("appointments").expect("gui.ui to define appointments");
    //incoming.bind_model(Some(&appointments.upcast::<gtk::gio::ListStore>()), |appt| { gtk::Label::new(Some("asdf")).upcast::<gtk::Widget>() });

    main.connect_destroy(move |main| {
        gtk::main_quit();
    });

    preview.connect_draw(move |preview, cairo| {
        let width = preview.allocated_width() as f64 - 20.0;
        let color = preview.style_context().color(gtk::StateFlags::NORMAL);
        let color_em = preview.style_context().color(gtk::StateFlags::LINK);
        let color_bg = RGBA::new(1.0,1.0,1.0,1.0);
        let scale = 1.25;
        cairo.set_line_width(1.0);

        cairo.set_source_rgb(color_bg.red(), color_bg.green(), color_bg.blue());
        cairo.rectangle(5.0, 10.0 * scale, width, 30.0 * scale);
        cairo.rectangle(5.0, 85.0 * scale, width, 45.0 * scale);
        cairo.clip();
        cairo.paint().unwrap();

        cairo.set_source_rgb(color_em.red(), color_em.green(), color_em.blue());
        cairo.rectangle(5.0, 10.0 * scale, 12.0, 30.0 * scale);
        cairo.rectangle(5.0, 85.0 * scale, 12.0, 45.0 * scale);
        cairo.fill().unwrap();

        cairo.set_source_rgb(color.red(), color.green(), color.blue());
        cairo.select_font_face("Lucida Grande", FontSlant::Normal, FontWeight::Normal);
        cairo.set_font_size(12.0);
        cairo.move_to(20.0, 10.0 * scale + 15.0);
        cairo.show_text("Single Process Color");
        cairo.move_to(20.0, 10.0 * scale + 30.0);
        cairo.show_text("30 min");
        cairo.move_to(20.0, 85.0 * scale + 15.0);
        cairo.show_text("Haircut");
        cairo.move_to(20.0, 85.0 * scale + 30.0);
        cairo.show_text("45 min");
        cairo.reset_clip();

        cairo.set_source_rgb(color.red(), color.green(), color.blue());
        cairo.rectangle(5.0, 10.0 * scale, width, 30.0 * scale);
        cairo.rectangle(5.0, 85.0 * scale, width, 45.0 * scale);
        cairo.stroke().unwrap();

        Inhibit(false)
    });

    let stylists_ = stylists.clone();
    day.connect_draw(move |day, cairo| {
        let width = (day.allocated_width() as f64 - 20.0) / 5.0;
        let color = day.style_context().color(gtk::StateFlags::NORMAL);
        let color_em = day.style_context().color(gtk::StateFlags::LINK);
        let color_bg = RGBA::new(1.0,1.0,1.0,1.0);
        let color_title = RGBA::new(0.95,0.95,0.95,1.0);
        cairo.set_line_width(1.0);

        for i in 0..5 {
            let f = i as f64;
            let (name, status, offset) = stylists_[i];
            let offset = offset * scale;

            cairo.rectangle(5.0 + width * f, offset, width - 5.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * f, offset + 75.0 * scale, width - 5.0, 45.0 * scale);
            cairo.clip();
            cairo.set_source_rgb(color_bg.red(), color_bg.green(), color_bg.blue());
            cairo.paint().unwrap();

            cairo.set_source_rgb(color_em.red(), color_em.green(), color_em.blue());
            cairo.rectangle(5.0 + width * f, offset, 12.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * f, offset + 75.0 * scale, 12.0, 45.0 * scale);
            cairo.fill().unwrap();

            cairo.set_source_rgb(color.red(), color.green(), color.blue());
            cairo.select_font_face("Lucida Grande", FontSlant::Normal, FontWeight::Normal);
            cairo.set_font_size(12.0);
            cairo.move_to(20.0 + width * f, offset + 15.0);
            cairo.show_text("Single Process Color");
            cairo.move_to(20.0 + width * f, offset + 30.0);
            cairo.show_text("30 min");
            cairo.move_to(20.0 + width * f, offset + 15.0 + (75.0 * scale));
            cairo.show_text("Haircut");
            cairo.move_to(20.0 + width * f, offset + 30.0 + (75.0 * scale));
            cairo.show_text("45 min");
            cairo.reset_clip();

            cairo.rectangle(5.0 + width * f, offset, width - 5.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * f, offset + 75.0 * scale, width - 5.0, 45.0 * scale);
            cairo.stroke().unwrap();
        }

        Inhibit(false)
    });

    app.connect_activate(move |app| {
        let scheduler = scheduler.clone();
        let book = book.clone();
        let check = check.clone();

        for (name, status, offset) in stylists.iter() {
            let button: gtk::Button = gtk::Button::with_label(&name);
            providers.pack_start(&button, true, true, 0);
        }

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

    timeline.connect_draw(move |timeline, cairo| {
        for i in 0..96 {
            let mut h = i / 4;
            if h == 0 { h = 12; }
            if h > 12 { h -= 12; }
            let m = i % 4;
            cairo.move_to(5.0, 12.0 + i as f64 * 15.0 * scale);
            cairo.set_font_size(12.0);
            cairo.select_font_face("Menlo", FontSlant::Normal, FontWeight::Normal);
            cairo.set_line_width(0.5);
            cairo.show_text(&format!("{:2}:{:02}", h, &["00","15","30","45"][m]));
            cairo.move_to(0.0, i as f64 * 15.0 * scale);
            cairo.line_to(timeline.allocated_width() as f64, i as f64 * 15.0 * scale);
            cairo.set_source_rgb(0.0, 0.0, 0.0);
            cairo.stroke().unwrap();
        };
        Inhibit(false)
    });

    app.run();
    gtk::main();
}
