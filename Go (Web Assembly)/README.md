## Prerequisites

Before you can run the game, make sure you have the following installed:

- **Rust** - [Install Rust](https://www.rust-lang.org/tools/install)
- **wasm-pack** - Will be installed automatically by the build script if not present
- **A web server** - Required to serve the game (see options below)

## Building the Project

### Windows
Run the build script:
```bash
scripts\build.bat
```

### Linux/Mac
Run the build script:
```bash
./scripts/build.sh
```

The build script will:
- Check if `wasm-pack` is installed (installs it automatically if missing)
- Compile the Rust code to WebAssembly
- Generate the necessary JavaScript bindings
- Copy files to the `web/` directory

**Note:** The first build may take several minutes as it downloads and compiles dependencies.

## Running the Game

The game uses ES6 modules and WebAssembly, which require it to be served over HTTP (you cannot simply open `index.html` directly in a browser due to CORS restrictions).

**Important:** Make sure to serve from the project root directory, not the `web/` folder, so that the paths resolve correctly.

### Option 1: Python HTTP Server

If you have Python installed:

```bash
python -m http.server 8000
```

Then open your browser and navigate to:
```
http://localhost:8000/web/index.html
```

### Option 2: Node.js HTTP Server

If you have Node.js installed:

```bash
npx http-server
```

Then open your browser and navigate to:
```
http://localhost:8080/web/index.html
```

## Troubleshooting

### "Error loading WebAssembly"
- Make sure you've run the build script (`scripts/build.bat` or `scripts/build.sh`)
- Check that `web/go_game_bg.wasm` exists in the web directory
- Ensure you're accessing the game through a web server, not by opening the file directly
- Make sure you're serving from the project root and accessing `/web/index.html`

### Build fails
- Ensure Rust is properly installed: `rustc --version`
- Try installing `wasm-pack` manually: `cargo install wasm-pack`
- Make sure you have the `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Game doesn't respond to clicks
- Check the browser console for errors (F12)

- Ensure WebAssembly is supported in your browser (all modern browsers support it)

##  Directory structure

Go 19 x 19/

├── src/ # Rust source code

│ ├── lib.rs # WebAssembly bindings

│ ├── game.rs # Game logic

│ └── board.rs # Board state management

├── web/ # Web files

│ ├── index.html # Main HTML file

│ ├── app.js # JavaScript frontend

│ ├── style.css # Styling

│ ├── go_game_bg.wasm # Compiled WebAssembly binary

│ └── wasm_exec.js # WebAssembly JavaScript bindings

├── scripts/ # Build scripts

│ ├── build.bat # Build script (Windows)

│ ├── build.ps1 # Build script (PowerShell)

│ └── build.sh # Build script (Linux/Mac)

├── Cargo.toml # Rust project configuration

├── Cargo.lock # Rust dependency lock file

└── README.md # Project documentation
