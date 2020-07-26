extern crate gtk;
use crate::gtk::prelude::*;
use gtk::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("app.glade");

    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").unwrap();
    let button: gtk::Button = builder.get_object("button1").unwrap();

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


    window.set_decorated(false);
    window.show_all();
    window.maximize();

    gtk::main();
}
