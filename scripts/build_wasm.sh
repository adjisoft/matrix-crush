#!/usr/bin/env bash
set -euo pipefail
OUT_DIR="${1:-docs}"

rustup target add wasm32-unknown-unknown

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  cargo install wasm-bindgen-cli --version 0.2.92
fi

cargo build --release --target wasm32-unknown-unknown

mkdir -p "$OUT_DIR"
wasm-bindgen --target web --no-typescript --out-dir "$OUT_DIR" target/wasm32-unknown-unknown/release/matrix_crushed.wasm

cp web/index.html "$OUT_DIR/index.html"
if [ -d assets ]; then
  cp -r assets "$OUT_DIR/assets"
fi

: > "$OUT_DIR/.nojekyll"