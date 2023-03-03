use std::rc::Rc;
use actix::prelude::*;
use gtk::prelude::*;
use gtk::{render_background, Application, Button, Inhibit};
use gtk::gdk::RGBA;
use glib::{timeout_add_local, Continue, Date, DateTime};
use core::time::Duration;
use gtk::cairo::{FontSlant, FontWeight};

pub fn main() {
    gtk::init();
    let scale: f64 = 1.25;

    let builder: Rc<gtk::Builder> = Rc::new(gtk::Builder::from_string(include_str!("gui.ui")));
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
    let day_viewport: gtk::Viewport = builder.object("day_viewport").unwrap();
    let day_window: gtk::ScrolledWindow = builder.object("day_window").unwrap();
    let timeline: gtk::DrawingArea = builder.object("timeline_area").unwrap();
    let timeline_window: gtk::ScrolledWindow = builder.object("timeline_window").unwrap();
    let timeline_viewport: gtk::Viewport = builder.object("timeline_viewport").unwrap();
    let providers: gtk::Box = builder.object("providers_box").unwrap();
    let providers_window: gtk::ScrolledWindow = builder.object("providers_window").unwrap();
    let providers_viewport: gtk::Viewport = builder.object("providers_viewport").unwrap();
    let timeline_adjustment: gtk::Adjustment = builder.object("timeline_adjustment").unwrap();
    timeline.set_height_request((1440.0 * scale) as i32);
    timeline_adjustment.set_upper(1440.0 * scale);
    timeline_adjustment.set_value(480.0 * scale);

    let providers_adjustment: gtk::Adjustment = builder.object("providers_adjustment").unwrap();
    let time: gtk::Label = builder.object("time").unwrap();


    let stylists: Rc<Vec<(&str, Button)>> = Rc::new(vec![
        ("Romeo"),
        ("Scott"),
        ("Hauns"),
        ("Susan"),
        ("Julissa"),
        ("Rosie"),
        ("Carpio"),
        //("Dawn"),
        //("Steve"),
        //("Romeo"),
        //("Natalia")
    ].iter().map(|name| (*name, Button::with_label(&name))).collect());

    let incoming: gtk::ListBox = builder.object("incoming").unwrap();
    //let appointments: gtk::ListStore = builder.object("appointments").expect("gui.ui to define appointments");
    //incoming.bind_model(Some(&appointments.upcast::<gtk::gio::ListStore>()), |appt| { gtk::Label::new(Some("asdf")).upcast::<gtk::Widget>() });

    main.connect_destroy(move |main| {
        gtk::main_quit();
    });

    let stylists_: Rc<Vec<(&str, Button)>> = stylists.clone();
    let day_ = day.clone();
    let providers_ = providers.clone();
    app.connect_activate(move |app| {
        let scheduler = scheduler.clone();
        let book = book.clone();
        let check = check.clone();
        let time = time.clone();

        for (name, button) in stylists_.iter() {
            button.set_width_request(150);
            providers_.pack_start(button, true, true, 0);
        }

        calendar.connect_activate(move |cal| {
            if cal.is_expanded() {
                scheduler.set_visible_child(&check);
            } else {
                scheduler.set_visible_child(&book);
            }
        });
        main.show_all();

        timeout_add_local(Duration::new(5, 0), move || {
            let now = DateTime::now_local().unwrap();
            time.set_text(&now.format("%l:%M %p").unwrap());
            Continue(true)
        });
    });

    preview.connect_draw(move |preview, cairo| {
        let width = preview.allocated_width() as f64 - 20.0;
        let color = preview.style_context().color(gtk::StateFlags::NORMAL);
        let color_em = preview.style_context().color(gtk::StateFlags::LINK);
        let color_bg = RGBA::new(1.0,1.0,1.0,1.0);
        let scale = 1.25;
        cairo.set_line_width(0.5);

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
        cairo.select_font_face("Andale Mono", FontSlant::Normal, FontWeight::Normal);
        cairo.set_font_size(10.0);
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

    let providers_ = providers.clone();
    day.connect_draw(move |day, cairo| {
        let color = day.style_context().color(gtk::StateFlags::NORMAL);
        let color_em = day.style_context().color(gtk::StateFlags::LINK);
        let color_bg = RGBA::new(1.0,1.0,1.0,1.0);
        let color_title = RGBA::new(0.95,0.95,0.95,1.0);
        cairo.set_line_width(1.0);
        let context = day.style_context();
        render_background(&context, cairo, 0.0, 0.0, day.allocated_width().into(), day.allocated_height().into());

        for (name, button) in stylists.iter() {
            let width = button.allocated_width() as f64;
            let x = button.translate_coordinates(&day_window, 0, 0).unwrap().0 as f64;

            cairo.rectangle(x, 0.0, width, 30.0 * scale);
            cairo.rectangle(x, 0.0 + 75.0 * scale, width, 45.0 * scale);
            cairo.clip();
            cairo.set_source_rgb(color_bg.red(), color_bg.green(), color_bg.blue());
            cairo.paint().unwrap();

            cairo.set_source_rgb(color_em.red(), color_em.green(), color_em.blue());
            cairo.rectangle(x, 0.0, 12.0, 30.0 * scale);
            cairo.rectangle(x, 0.0 + 75.0 * scale, 12.0, 45.0 * scale);
            cairo.fill().unwrap();

            cairo.set_source_rgb(color.red(), color.green(), color.blue());
            cairo.select_font_face("Baskerville", FontSlant::Normal, FontWeight::Normal);
            cairo.set_font_size(13.0);
            cairo.move_to(x + 20.0, 0.0 + 15.0);
            cairo.show_text("Single Process Color");
            cairo.move_to(x + 20.0, 0.0 + 15.0 + (75.0 * scale));
            cairo.show_text("Haircut");
            cairo.select_font_face("San Francisco", FontSlant::Normal, FontWeight::Normal);
            cairo.set_font_size(11.0);
            cairo.move_to(x + 20.0, 0.0 + 30.0);
            cairo.show_text("30 min");
            cairo.move_to(x + 20.0, 0.0 + 30.0 + (75.0 * scale));
            cairo.show_text("Schedule");
            cairo.reset_clip();

            cairo.rectangle(x, 0.0, width, 30.0 * scale);
            cairo.rectangle(x, 0.0 + 75.0 * scale, width, 45.0 * scale);
            cairo.stroke().unwrap();
        }

        Inhibit(false)
    });

    timeline.connect_draw(move |timeline, cairo| {
        cairo.set_line_width(0.5);
        let color = timeline.style_context().color(gtk::StateFlags::NORMAL);
        for i in 0..96 {
            let mut pm = false;
            let mut h = i / 4;
            if h == 0 { h = 12; }
            if h > 12 { h -= 12; pm = true; }
            let m = i % 4;
            cairo.move_to(5.0, 12.0 + i as f64 * 15.0 * scale);
            cairo.set_font_size(12.0);
            cairo.select_font_face("Courier New", FontSlant::Normal, FontWeight::Normal);
            cairo.set_line_width(0.5);
            cairo.show_text(&format!("{:2}:{:02} {}", h, &["00","15","30","45"][m], if pm { "PM" } else { "AM" }));
            let line_to = 0.5 + (i as f64 * 15.0 * scale);
            cairo.move_to(0.0, line_to);
            cairo.line_to(timeline.allocated_width() as f64, line_to);
            cairo.set_source_rgb(color.red(), color.green(), color.blue());
            cairo.stroke().unwrap();
        };
        Inhibit(false)
    });

    app.run();
    gtk::main();
}
