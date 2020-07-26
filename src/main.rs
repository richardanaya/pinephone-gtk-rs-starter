use gtk::prelude::*;
use gtk::*;
use std::f64::consts::PI;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("app.glade");

    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").unwrap();
    let button: gtk::Button = builder.get_object("button1").unwrap();
    let canvas: gtk::DrawingArea = builder.get_object("drawable1").unwrap();
    
    // set global style
    let screen = window.get_screen().unwrap();
    let style = include_str!("style.css");
    let provider = CssProvider::new();
    provider.load_from_data(style.as_bytes()).unwrap();
    gtk::StyleContext::add_provider_for_screen(&screen, &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

    // exit app on click
    button.connect_clicked(move |_| {
        println!("exiting app!");
        std::process::exit(0);
    });

    // handle draw and use cairo context
    canvas.connect_draw(|_, ctx| {
        ctx.scale(100f64, 100f64);
        ctx.set_source_rgba(1.0, 0.2, 0.2, 0.6);
        ctx.arc(0.40, 0.53, 0.2, 0.0, PI * 2.);
        ctx.fill();
        ctx.set_source_rgba(1.0, 0.2, 0.92, 0.6);
        ctx.arc(0.5, 0.65, 0.2, 0.0, PI * 2.);
        ctx.fill();
        ctx.set_source_rgba(0.0, 0.2, 0.92, 0.6);
        ctx.arc(0.6, 0.53, 0.2, 0.0, PI * 2.);
        ctx.fill();

        Inhibit(false)
    });


    window.show_all();

    // don't maximize in debug (we assume debug is desktop)
    // on phone we should maximize
    #[cfg(not(debug_assertions))]
    window.maximize();

    gtk::main();
}
