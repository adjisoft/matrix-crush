param(
    [string]$OutDir = "docs"
)

$ErrorActionPreference = "Stop"

rustup target add wasm32-unknown-unknown

if (-not (Get-Command wasm-bindgen -ErrorAction SilentlyContinue)) {
    cargo install wasm-bindgen-cli --version 0.2.92
}

cargo build --release --target wasm32-unknown-unknown

if (-not (Test-Path $OutDir)) {
    New-Item -ItemType Directory -Path $OutDir | Out-Null
}

wasm-bindgen --target web --no-typescript --out-dir $OutDir target/wasm32-unknown-unknown/release/matrix_crushed.wasm

Copy-Item web/index.html (Join-Path $OutDir "index.html") -Force
if (Test-Path assets) {
    Copy-Item assets (Join-Path $OutDir "assets") -Recurse -Force
}
"" | Set-Content -Path (Join-Path $OutDir ".nojekyll") -NoNewline