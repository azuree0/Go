Write-Host "Building Rust WebAssembly..." -ForegroundColor Cyan

# Check if wasm-pack is installed
$wasmPackPath = Get-Command wasm-pack -ErrorAction SilentlyContinue
if (-not $wasmPackPath) {
    Write-Host "wasm-pack is not installed. Installing..." -ForegroundColor Yellow
    cargo install wasm-pack
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to install wasm-pack. Please install it manually." -ForegroundColor Red
        Read-Host "Press Enter to exit"
        exit 1
    }
}

# Build the WebAssembly file
wasm-pack build --target web --out-dir pkg

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    Read-Host "Press Enter to exit"
    exit 1
}

# Copy the generated files to the web directory
Copy-Item -Path "pkg\go_game_bg.wasm" -Destination "web\go_game_bg.wasm" -Force
Copy-Item -Path "pkg\go_game.js" -Destination "web\wasm_exec.js" -Force

Write-Host "Build complete! Open web/index.html in a web browser." -ForegroundColor Green
Read-Host "Press Enter to exit"

