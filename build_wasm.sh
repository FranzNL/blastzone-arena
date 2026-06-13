#!/usr/bin/env bash
set -e

echo "=== BlastZone Arena WASM Build ==="

# Ensure the wasm target is installed
rustup target add wasm32-unknown-unknown

# Build
echo "Building for wasm32..."
cargo build --target wasm32-unknown-unknown --profile wasm-release

# Check wasm-bindgen is installed
if ! command -v wasm-bindgen &>/dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

WASM_FILE="target/wasm32-unknown-unknown/wasm-release/blastzone-arena.wasm"

# Generate JS bindings
echo "Running wasm-bindgen..."
mkdir -p dist
wasm-bindgen \
    --out-dir ./dist \
    --target web \
    --no-typescript \
    "$WASM_FILE"

# Copy assets and HTML
echo "Copying assets..."
cp -r assets dist/
cp index.html dist/

echo ""
echo "=== Build complete! ==="
echo "To serve: cd dist && python3 -m http.server 8080"
echo "Then open: http://localhost:8080"
echo ""
echo "Upload the contents of dist/ to your web server."
echo "Make sure your server sets: Cross-Origin-Opener-Policy: same-origin"
echo "                            Cross-Origin-Embedder-Policy: require-corp"
echo "(or use a simple HTTP server — the headers above are only needed for SharedArrayBuffer)"
