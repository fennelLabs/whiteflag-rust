#!/usr/bin/env bash

rustup update stable
rustup override set stable

cargo update
cargo clean
cargo build
cargo test
cargo doc

cargo build --release
cargo doc --release
