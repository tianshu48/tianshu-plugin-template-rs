#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
rustup target add wasm32-wasip2 >/dev/null
cargo build --features guest --target wasm32-wasip2 --release
mkdir -p dist
cp plugin.json ui.json USER.md dist/
if [ -f USER.zh.md ]; then cp USER.zh.md dist/; fi
if [ -f icon.svg ]; then cp icon.svg dist/; fi
wasm=$(find target/wasm32-wasip2/release -maxdepth 1 -name '*.wasm' | head -n 1)
test -n "$wasm"
cp "$wasm" dist/plugin.wasm
echo "packed dist/"
