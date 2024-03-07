Welcome to Wisp!

This project uses Rust and the Leptos framework to build web applications.  While it's still under development ("Here there be dragons"), this guide will help you get started with building and running Wisp.

## Prerequisites:
**Rust**: Download and install Rust from the official [website](https://www.rust-lang.org/tools/install). This will also install cargo, the Rust package manager.
**Trunk**: Install the [Trunk](https://trunkrs.dev/) bundler using cargo:
```
cargo install trunk
```

## Building Wisp:
There are three ways to build Wisp, depending on your needs:

### 1. Build Once
To build the application for deployment, run this command in your terminal (assuming you're in the project's root directory):
```
trunk build --release
```

### 2. Build Continuously
This option automatically rebuilds the application whenever you make changes to the code. This is useful during development. To use it, run:
```
trunk watch --watch src/ --watch Cargo.toml --watch index.html
```

### 3. Build Continuously & Run Development Server
This option combines building and running a development server in one command. This is also helpful for development:
```
trunk serve --port 3000 --open --watch src/ --watch Cargo.toml --watch index.html
```
This will start the development server at http://localhost:3000 and open it in your browser. The --ignore tests/ flag tells Trunk to ignore changes in the tests folder to avoid unnecessary rebuilds.

## Making Wisp load faster:
Wisp builds a single JavaScript file that loads the actual computation happening within a separate .wasm binary. This JavaScript file can be minified to reduce its size, making your web page load a bit faster – especially important for release builds.

Here's how to minify the JavaScript file:
### 1. Install `esbuild` (Node package manager required):
In this example, we'll use `pnpm` as the Node package manager. If you don't have one installed, you'll need to set that up first. Once you have `pnpm` (or another package manager) ready, run this command in your project's root directory to install `esbuild`:
```
pnpm install
```
After installing `esbuild`, run the following command in your terminal (again, assuming you're in the project's root directory):
```
./node_modules/.bin/esbuild ./dist/**/*.js --bundle --minify --outdir=./dist --allow-overwrite
```
#### Explanation of the flags:
- `./dist/**/*.js`: This tells `esbuild` to look for all JavaScript files (represented by `*.js`) within the `dist` folder and its subfolders (represented by `**`).
- `--bundle`: Combines all the JavaScript files into a single minified file.
- `--minify`: Removes unnecessary characters and whitespace from the code, reducing its size.
- `--outdir=./dist`: Specifies the output directory for the minified file. It will be placed back in the `dist` folder.
- `--allow-overwrite`: Allows `esbuild` to overwrite any existing files in the output directory.

## Testing Wisp:
To check if the test cases pass in a browser environment, install [`wasm-pack`](https://github.com/rustwasm/wasm-pack), and run:
```
wasm-pack test --headless --firefox
```

## Wisp's Core:
- **Leptos**: This project is built with Leptos, a powerful web framework for Rust. You can learn more about Leptos at its [website](https://leptos.dev/).
- **Rust Stability**: Wisp uses Rust's stable development channel for reliability. This means it won't be able to use some experimental features available in nightly Rust releases.
