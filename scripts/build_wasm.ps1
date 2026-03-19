param(
    [string]$OutDir = "docs"
)

$ErrorActionPreference = "Stop"

rustup target add wasm32-unknown-unknown

cargo build --release --target wasm32-unknown-unknown

if (-not (Test-Path $OutDir)) {
    New-Item -ItemType Directory -Path $OutDir | Out-Null
}

Copy-Item target/wasm32-unknown-unknown/release/matrix_crushed.wasm (Join-Path $OutDir "matrix_crushed.wasm") -Force

$cargoHome = $env:CARGO_HOME
if (-not $cargoHome) {
    $cargoHome = Join-Path $env:USERPROFILE ".cargo"
}
$bundle = Get-ChildItem (Join-Path $cargoHome "registry\\src") -Recurse -Filter mq_js_bundle.js | Select-Object -First 1
if ($bundle) {
    Copy-Item $bundle.FullName (Join-Path $OutDir "mq_js_bundle.js") -Force
}

if (Test-Path web/index.html) {
    Copy-Item web/index.html (Join-Path $OutDir "index.html") -Force
}
if (Test-Path assets) {
    Copy-Item assets (Join-Path $OutDir "assets") -Recurse -Force
}
"" | Set-Content -Path (Join-Path $OutDir ".nojekyll") -NoNewline
