@echo off
echo Building Rust WebAssembly...

REM Check if wasm-pack is installed
where wasm-pack >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo wasm-pack is not installed. Installing...
    cargo install wasm-pack
    if %ERRORLEVEL% NEQ 0 (
        echo Failed to install wasm-pack. Please install it manually.
        pause
        exit /b 1
    )
)

REM Build the WebAssembly file
wasm-pack build --target web --out-dir pkg

REM Copy the generated files to the web directory
copy pkg\go_game_bg.wasm web\go_game_bg.wasm
copy pkg\go_game.js web\wasm_exec.js

echo Build complete! Open web/index.html in a web browser.
pause
