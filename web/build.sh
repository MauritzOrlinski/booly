#!/usr/bin/env sh
# Builds the WebAssembly package for doesitsat into web/pkg.
# Requires: rustup target add wasm32-unknown-unknown && cargo install wasm-pack
set -eu
cd "$(dirname "$0")/.."
wasm-pack build --release --target web --out-dir web/pkg --out-name booly --no-default-features --features wasm
echo "Serve the web/ folder with any static file server, e.g.: python3 -m http.server -d web 8000"
