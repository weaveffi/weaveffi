# Node

The Node generator produces a small CommonJS loader and `.d.ts` types describing
your functions. For the examples in this repo, a separate N-API addon crate
(`weaveffi-node-addon`) loads the C ABI symbols and exposes JS-friendly functions.

## Generated artifacts

- `generated/node/index.js` – CommonJS loader that requires `./index.node`
- `generated/node/types.d.ts` – function signatures inferred from your IDL
- `generated/node/package.json`

## Running the example

```bash
# Build the Rust libraries
cargo build -p calculator
cargo build -p weaveffi-node-addon

# Place the addon where the loader expects it
cp target/debug/libindex.dylib generated/node/index.node

# Run the example
cd examples/node
DYLD_LIBRARY_PATH=../../target/debug npm start
```

Notes:
- On Linux, use `LD_LIBRARY_PATH` instead of `DYLD_LIBRARY_PATH`.
- The loader expects the compiled addon next to it as `index.node`.
