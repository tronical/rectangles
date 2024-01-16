# Prerequisites

 - Install Rust: https://rustup.rs
 - Install Cross: `cargo install cross --git https://github.com/cross-rs/cross`
 - Install Docker

# Run locally

`cargo run`

# Build for armv7

`cross build --target=armv7-unknown-linux-gnueabihf`

# Build for armv8

`cross build --target=aarch64-unknown-linux-gnu`
