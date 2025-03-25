# mpvr
An mpv wrapper written in Rust to play music in a single mpv instance.

Tested with Linux, compatibility with other systems is untested and not guaranteed.

## Installation
- Install [Rust](https://www.rust-lang.org/).
- Clone the repository:
  `git clone https://github.com/enderprism/mpvr`
- Compile mpvr with cargo:
  `cargo build --release`
- Grab the binary in `target/release/`.
- Put it somewhere on your `$PATH`.

> [!TIP]
> You can use the `mpvr.desktop.sample` file to add a desktop entry for mpvr.
