<img width="973" height="1175" alt="Go" src="https://github.com/user-attachments/assets/2eb81b64-8816-4a0d-8cea-27348d40b87b" />

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
