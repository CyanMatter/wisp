Here there be dragons.

## Build

### Check the prerequisites
**Please ensure**…
- … that [`Rust`](https://www.rust-lang.org/tools/install) is installed. This installation will include `cargo`;
- … that [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) is installed;
- … that [`rsw-rs`](https://github.com/rwasm/rsw-rs) is installed;

… **and optionally, if you choose to run the development server, also ensure**…
- … that [`Node.js`](https://nodejs.org/en/download) is installed (v14.21.3 Fermium, or more recent);
- … that [`pnpm`](https://pnpm.io/installation) is installed.

### How to build once
To build the application, open a command line in the root of the repository, and run:
```
pnpm build
```
… then run the following to view the application at [localhost:3000](http://localhost:3000/).
```
pnpm preview
```

### How to watch for changes
Alternatively, to have the application reflect your modifications continuously at [localhost:3000](http://localhost:3000), run:
```
pnpm dev
```