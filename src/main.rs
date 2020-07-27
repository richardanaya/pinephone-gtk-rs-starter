use gtk::prelude::*;
use gtk::*;
use rand::Rng;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // we embed our image,glade file, and css  in a glib gresource file generated
    //  from app.xml, let's load it in from bytes embedded in our app
    let bytes = glib::Bytes::from_static(include_bytes!("app.gresource"));
    let res = gio::Resource::from_data(&bytes).unwrap();
    gio::resources_register(&res);

    // lets generate all our controls from glade file
    let builder = gtk::Builder::from_resource("/app/app.glade");

    // grab the controls we'll be using
    let window: gtk::Window = builder.get_object("window1").unwrap();
    let button: gtk::Button = builder.get_object("button1").unwrap();
    let canvas: Rc<RefCell<gtk::DrawingArea>> =
        Rc::new(RefCell::new(builder.get_object("drawable1").unwrap()));

    // set global style to something cool
    let screen = window.get_screen().unwrap();
    let provider = CssProvider::new();
    provider.load_from_resource("/app/style.css");
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // redraw on click
    let canvas_handle = canvas.clone();
    button.connect_clicked(move |_| {
        canvas_handle.borrow_mut().queue_draw();
    });

    // handle draw and use cairo context
    canvas.borrow_mut().connect_draw(|_, ctx| {
        let mut rng = rand::thread_rng();
        ctx.scale(100f64, 100f64);
        let rgb = (rng.gen(), rng.gen(), rng.gen());
        ctx.set_source_rgba(rgb.0, rgb.1, rgb.2, 0.6);
        ctx.arc(0.40, 0.53, 0.2, 0.0, PI * 2.);
        ctx.fill();
        let rgb = (rng.gen(), rng.gen(), rng.gen());
        ctx.set_source_rgba(rgb.0, rgb.1, rgb.2, 0.6);
        ctx.arc(0.5, 0.65, 0.2, 0.0, PI * 2.);
        ctx.fill();
        let rgb = (rng.gen(), rng.gen(), rng.gen());
        ctx.set_source_rgba(rgb.0, rgb.1, rgb.2, 0.6);
        ctx.arc(0.6, 0.53, 0.2, 0.0, PI * 2.);
        ctx.fill();

        Inhibit(false)
    });

    // show the window
    window.show_all();

    // don't maximize in debug (we assume debug is desktop)
    // on phone we should maximize
    #[cfg(not(debug_assertions))]
    window.maximize();

    gtk::main();
}
