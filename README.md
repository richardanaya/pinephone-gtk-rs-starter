# pinephone-gtk-rs-starter


## Setup Rust for cross compilation

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
