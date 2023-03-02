use actix::prelude::*;
use gtk::prelude::*;
use gtk::{Application, Inhibit};
use gtk::gdk::RGBA;
use glib::{Date, DateTime};
use core::time::Duration;
use gtk::cairo::{FontSlant, FontWeight, Rectangle};
use librsvg;

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
    let viewport: gtk::Viewport = builder.object("day_viewport").unwrap();

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

    day.connect_draw(move |day, cairo| {
        let width = (day.allocated_width() as f64 - 20.0) / 5.0;
        let start = 35.0;
        let color = day.style_context().color(gtk::StateFlags::NORMAL);
        let color_em = day.style_context().color(gtk::StateFlags::LINK);
        let color_bg = RGBA::new(1.0,1.0,1.0,1.0);
        let color_title = RGBA::new(0.95,0.95,0.95,1.0);
        let scale = 1.25;
        cairo.set_line_width(1.0);
        let status_handle = librsvg::Loader::new().read_path("share/status.svg").unwrap();
        let status_renderer = librsvg::CairoRenderer::new(&status_handle);

        let stylists = vec![
            ("Susan", "#available", 75.0),
            ("Richard", "#alert", 135.0),
            ("Charlie", "#unavailable", 45.0),
            ("Juliet", "#late", 0.0),
            ("Romeo", "#missing", 90.0)
        ];

        for i in 0..5 {
            let f = i as f64;

            cairo.rectangle(2.5 + width * f, 2.5, width - 2.5, 28.0);
            cairo.clip();
            cairo.set_source_rgb(color_title.red(), color_title.green(), color_title.blue());
            cairo.paint().unwrap();

            cairo.set_source_rgb(color.red(), color.green(), color.blue());
            cairo.rectangle(2.5 + width * f, 2.5, width - 2.5, 28.0);
            cairo.stroke().unwrap();

            cairo.select_font_face("DIN Alternate", FontSlant::Normal, FontWeight::Bold);
            cairo.set_font_size(14.0);
            cairo.move_to(10.0 + width * f, 21.0);
            let (name, status, offset) = stylists[i];
            let start = start + (offset * scale);
            cairo.show_text(name);
            status_renderer.render_layer(&cairo, Some(status), &Rectangle::new(2.5 + width * f + width - 30.0, 5.0, 25.0, 25.0)).expect(status);
            cairo.reset_clip();

            //----

            cairo.rectangle(5.0 + width * f, start, width - 5.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * f, start + 75.0 * scale, width - 5.0, 45.0 * scale);
            cairo.clip();
            cairo.set_source_rgb(color_bg.red(), color_bg.green(), color_bg.blue());
            cairo.paint().unwrap();

            cairo.set_source_rgb(color_em.red(), color_em.green(), color_em.blue());
            cairo.rectangle(5.0 + width * f, start, 12.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * f, start + 75.0 * scale, 12.0, 45.0 * scale);
            cairo.fill().unwrap();

            cairo.set_source_rgb(color.red(), color.green(), color.blue());
            cairo.select_font_face("Lucida Grande", FontSlant::Normal, FontWeight::Normal);
            cairo.set_font_size(12.0);
            cairo.move_to(20.0 + width * f, start + 15.0);
            cairo.show_text("Single Process Color");
            cairo.move_to(20.0 + width * f, start + 30.0);
            cairo.show_text("30 min");
            cairo.move_to(20.0 + width * f, start + 15.0 + (75.0 * scale));
            cairo.show_text("Haircut");
            cairo.move_to(20.0 + width * f, start + 30.0 + (75.0 * scale));
            cairo.show_text("45 min");
            cairo.reset_clip();

            cairo.rectangle(5.0 + width * f, start, width - 5.0, 30.0 * scale);
            cairo.rectangle(5.0 + width * f, start + 75.0 * scale, width - 5.0, 45.0 * scale);
            cairo.stroke().unwrap();
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
