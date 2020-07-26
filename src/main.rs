extern crate gtk;
use crate::gtk::prelude::BuilderExtManual;
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

    button.connect_clicked(move |_| println!("foo"));

    window.show_all();

    gtk::main();
}
