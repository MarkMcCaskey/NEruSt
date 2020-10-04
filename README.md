# NEruSt
![Test](https://github.com/MarkMcCaskey/NEruSt/workflows/Test/badge.svg?branch=master)

NES emulator.  Back from the dead!  (for real this time)

High level TODO list:
- [ ] Come up with a better name!
- [x] CPU emulation!
- [ ] Emulate 1 type of cartridge!
- [ ] Draw something!
- [ ] Play some sound!
- [ ] Handle some input!
- [x] Wasm support
- [ ] Make the emulator nice to use once it's working (i.e. configuration, Wasm support, GUI).

## Running

To run with Wasm, run:

```sh
# Build the Rust Wasm
cargo build --release --target=wasm32-unknown-unknown

# Run a server to serve the html, js, and Wasm
python -m SimpleHTTPServer 8000
```

then go to http://localhost:8000/frontend/index.html to use it!
