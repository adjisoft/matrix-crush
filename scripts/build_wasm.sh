#!/usr/bin/env bash
set -euo pipefail
OUT_DIR="${1:-docs}"

rustup target add wasm32-unknown-unknown

cargo build --release --target wasm32-unknown-unknown

mkdir -p "$OUT_DIR"
cp target/wasm32-unknown-unknown/release/matrix_crushed.wasm "$OUT_DIR/matrix_crushed.wasm"

CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
MQ_JS_BUNDLE="$(find "$CARGO_HOME/registry/src" -name mq_js_bundle.js | head -n 1)"
if [ -n "$MQ_JS_BUNDLE" ]; then
  cp "$MQ_JS_BUNDLE" "$OUT_DIR/mq_js_bundle.js"
fi

if [ -f web/index.html ]; then
  cp web/index.html "$OUT_DIR/index.html"
fi
if [ -d assets ]; then
  cp -r assets "$OUT_DIR/assets"
fi

: > "$OUT_DIR/.nojekyll"
