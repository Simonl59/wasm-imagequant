#!/bin/sh
# Builds the npm package of this fork into pkg/.
set -eu
cd "$(dirname "$0")/.."
cargo build --release --target wasm32-unknown-unknown --lib
rm -rf pkg
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/wasm_imagequant.wasm
cp npm/package.json npm/README.md LICENSE-MIT LICENSE-APACHE pkg/
