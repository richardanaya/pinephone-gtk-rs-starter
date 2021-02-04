use std::process::Command;

fn main() {
    Command::new("glib-compile-resources")
        .arg("--target=src/app.gresource")
        .args(&["--sourcedir=src", "src/app.xml"])
        .status()
        .unwrap();

    // List all files that go into the resources file for rerun check.
    println!("cargo:rerun-if-changed=src/app.xml");
    println!("cargo:rerun-if-changed=src/app.glade");
    println!("cargo:rerun-if-changed=src/pine.svg");
    println!("cargo:rerun-if-changed=src/style.css");
}
