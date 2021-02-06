# pinephone-gtk-rs-starter

This repo is a starter for creating a custom styled Gtk UI app in Rust for PinePhone

![pinephone rust app](screenshots/pinephone_app.jpg)
![pinephone rust app dev](screenshots/screenshot.png)

## Compilation

Run `cargo build` - checkout the `build.rs` file to see how everything is done.

Notice how I compile glade, images, and css into a single `.gresource` file, this lets me embed 
the bytes of all the things my app needs into my Rust app.

Currently I compile this on my desktop for quick iteration and my phone to test out.

* In debug mode, the app will be about the size of a pinephone
* In release mode, the app will maximize to take up available space (assumed to be running on phone)

When I compile on the phone, I have to increase my amount of ram by using zram

```
sudo swapoff /dev/zram0 
sudo zramctl --reset /dev/zram0 
sudo zramctl --find --size 2048M
sudo mkswap /dev/zram0 
sudo swapon /dev/zram0
```

## Setup Rust for cross compilation

1. Make sure you have docker installed and running
2. Get [cross](https://github.com/rust-embedded/cross) by running `cargo install cross`
3. Build the Dockerfile in this folder with `docker build -t pinephone-build docker`
4. Run `cross build --target aarch64-unknown-linux-gnu` with or without `--release` to build your project
5. Copy `target/aarch64-unknown-linux-gnu/gld-test` to your phone
6. Run the application on your phone and marvel.

