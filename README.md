# atomCAD

A CAD environment for designing atomically-precise molecular nanotechnology.

## To Run

1. [Install Rust](https://rustup.rs/)
2. Install build dependencies: `brew install cmake` (macOS), `apt install build-essential cmake libx11-dev` (debian/ubuntu)
3. `git clone` this repository and navigate to it
4. run `cargo run`

## Web

1. Install wasm32 target: `rustup target add wasm32-unknown-unknown`
2. Install trunk: `cargo install --locked trunk`
3. `git clone` this repository and navigate to it
4. run `trunk serve --open`
