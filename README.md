# atomCAD

A CAD environment for designing atomically-precise molecular nanotechnology.

## To Run

1. [Install Rust](https://rustup.rs/)
2. Install build dependencies: `brew install cmake` (macOS), `apt install build-essential cmake libx11-dev` (debian/ubuntu)
3. `git clone` this repository and navigate to it
4. run `cargo run`

needed to install libiconv

## Needed to build dependencies w/ Nix on MacOS:
`export CFLAGS="-I/System/Volumes/Data/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include -Wno-undef-prefix"`
`export SDKROOT="$(xcrun --sdk macosx --show-sdk-path)"` 

There is an unresolved nix issue where the nix clang does not properly have some headers included, you must use apple clang via `PATH="/usr/bin:$PATH" cargo build` (yes, this defeats the purpose of nix in a large way, but it works for now)
