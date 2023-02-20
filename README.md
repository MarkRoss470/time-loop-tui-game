# Wibbly-Wobbly Timey Wimey Stuff (in space)

A text-based adventure game in rust. Includes many rooms, items, lines of dialogue, and actions.

# Getting started

## Unix systems (only linux tested)

Make sure you have cargo installed. Clone the repo and run `cargo run`. Use `--release` for full-terminal menus. 

## Other - from stock

Go to [this online linux virtualiser](https://copy.sh/v86/?profile=archlinux) and upload the binary from the latest release. Run the following commands to run the binary (the emulator doesn't support paste, so type each line until the #):

```sh
chmod +x /rust-text-game # This makes the file executable, so you can run it as a program
/rust-text-game # This runs the program
# If you get a 'no such file or directory' error, run `ls /` and look at the file names
# Then run the above commands with 'rust-text-game' replaced with the file name you saw
```

## Other - with changes

* Make sure you have cargo installed
* Install the `i686-unknown-linux-gnu` target (run `rustup target add i686-unknown-linux-gnu` if you installed your toolchain through rustup)
* Make your changes to the codebase
* Build the binary using `cargo build --release --target i686-unknown-linux-gnu --features no-flicker`
* The binary will be written to `target/i686-unknown-linux-gnu/release/rust-text-game`
* Upload this binary to the website listed above and run the commands


# About

See [here](features.md) for a list of features - spoiler warning