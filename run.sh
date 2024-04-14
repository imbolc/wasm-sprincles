#!/usr/bin/env bash
set -eux

rustup target add wasm32-unknown-unknown
cargo install miniserve

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/basic_bindgen.wasm --no-typescript --target web --out-dir pkg
wasm-bindgen target/wasm32-unknown-unknown/release/greet.wasm --no-typescript --target web --out-dir pkg
wasm-bindgen target/wasm32-unknown-unknown/release/events.wasm --no-typescript --target web --out-dir pkg

miniserve . --index "index.html" -p 8000
