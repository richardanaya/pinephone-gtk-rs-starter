# pinephone-gtk-rs-starter

This repo is a starter for creating a custom styled Gtk UI app in Rust for PinePhone

![pinephone rust app](screenshots/pinephone_app.jpg)
![pinephone rust app dev](screenshots/screenshot.png)

## Compilation

Currently I compile this on my desktop for quick iteration and my phone to test out.

* in debug mode, the app will be about the size of a pinephone
* in release mode, the app will maximize to take up available space (assumed to be running on phone)

When i compile on the phone, I have to increase my amount of ram by uzing zram

```
sudo swapoff /dev/zram0 
sudo zramctl --reset /dev/zram0 
sudo zramctl --find --size 2048M
sudo mkswap /dev/zram0 
sudo swapon /dev/zram0
```

## Setup Rust for cross compilation

I haven't figured this stuff out yet, I could use some help on these details.

1. get gcc setup for aarch64

```
sudo apt-get install gcc-aarch64-linux-gnu
```
```
sudo dnf install gcc-aarch64-linux-gnu
```

2. get rust setup for aarch64

```
rustup install stable-aarch64-unknown-linux-gnu
```

add to `~/.cargo/config`

```
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

3. Make sure you have aarch64 dependencies for Gtk

???

4. Start the compile!

```
PKG_CONFIG_ALLOW_CROSS=1 cargo build --target aarch64-unknown-linux-gnu --release
```
