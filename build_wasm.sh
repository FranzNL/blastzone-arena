#!/usr/bin/env bash
set -e

echo "=== BlastZone Arena WASM Build ==="

# Ensure the wasm target is installed
rustup target add wasm32-unknown-unknown

# Build
echo "Building for wasm32..."
cargo build --target wasm32-unknown-unknown --profile wasm-release

# Install wasm-bindgen-cli at the version matching Cargo.lock
WASM_BINDGEN_VERSION=$(grep -A1 'name = "wasm-bindgen"' Cargo.lock | grep version | head -1 | sed 's/.*version = "\([^"]*\)".*/\1/')
echo "Installing wasm-bindgen-cli $WASM_BINDGEN_VERSION..."
cargo install wasm-bindgen-cli --version "$WASM_BINDGEN_VERSION" --locked

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
