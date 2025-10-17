# WASM

The WASM generator produces a minimal JS loader and README to help get started
with `wasm32-unknown-unknown`. Full ergonomics are planned for future releases.

## Generated artifacts

- `generated/wasm/weaveffi_wasm.js`
- `generated/wasm/README.md`

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Serve the `.wasm` and load it with the provided JS helper.
