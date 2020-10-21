# NEruSt
![Test](https://github.com/MarkMcCaskey/NEruSt/workflows/Test/badge.svg?branch=master)

NES emulator.  Back from the dead!  (for real this time)

High level TODO list:
- [ ] Come up with a better name!
- [x] CPU emulation!
- [x] Emulate 1 type of cartridge!
- [ ] Draw something!
- [ ] Play some sound!
- [x] Handle some input!
- [x] Wasm support
- [ ] Make the emulator nice to use once it's working (i.e. configuration, Wasm support, GUI).

## Development

For development, consider using [cargo-web](https://github.com/koute/cargo-web).

You can install it with

```sh
cargo install cargo-web
```

Then set it up to watch the project and automatically rebuild it

```sh
cargo web start
```

then go to http://localhost:8000 to interact with it.

## Running

To run with Wasm, run:

```sh
# Build the Rust Wasm
cargo build --release --target=wasm32-unknown-unknown

# Run a server to serve the html, js, and Wasm
python -m SimpleHTTPServer 8000
```

then go to http://localhost:8000/static/index.html to use it!
