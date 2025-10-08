#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir web --target web target/wasm32-unknown-unknown/release/rustwasm_web_canavs_game.wasm