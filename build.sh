#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_canvas.wasm --target web --out-dir ./public/pkg
deno bundle public/ts/canvas.ts public/js/canvas.js
