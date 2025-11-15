#!/bin/bash

echo "Building Rust WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing..."
    cargo install wasm-pack
    if [ $? -ne 0 ]; then
        echo "Failed to install wasm-pack. Please install it manually."
        exit 1
    fi
fi

# Build the WebAssembly file
wasm-pack build --target web --out-dir pkg

# Copy the generated files to the web directory
cp pkg/go_game_bg.wasm web/go_game_bg.wasm
cp pkg/go_game.js web/wasm_exec.js

echo "Build complete! Open web/index.html in a web browser."
