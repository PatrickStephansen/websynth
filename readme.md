# Websynth

## Rust to WASM toolchain

* Install `rustup` with: `curl https://sh.rustup.rs -sSf | sh`. Use the default settings.
* Follow the instructions at https://rust-lang-nursery.github.io/rust-wasm/setup.html. Skip `wasm-opt` for now.

## Build

* Compile with `cargo +nightly build --target wasm32-unknown-unknown --release`
* Minimise build size with `wasm-gc target/wasm32-unknown-unknown/release/websynth.wasm -o websynth.gc.wasm`

## Hosting

A web host that supports the 'application/wasm' MIME type is needed for proper wasm streaming. Install `https` using `cargo install https`. OpenSSL was missing on my machine, so I had to run `sudo apt-get install libssl-dev` before that would work. Once installed, run the site with the command `http` from the root of this repo.