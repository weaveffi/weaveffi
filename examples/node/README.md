Run the Node example (N-API addon)

Prereqs:
- Node.js 16+ installed
- Rust toolchain installed

Steps (from repo root unless noted):

1) Build the Rust libraries

```
cargo build -p calculator
cargo build -p weaveffi-node-addon
```

2) Place the addon where the generated loader expects it

```
cp target/debug/libindex.dylib generated/node/index.node
```

3) Run the example (from this folder)

```
cd examples/node
DYLD_LIBRARY_PATH=../../target/debug npm start
```

Notes:
- On Linux, use `LD_LIBRARY_PATH` instead of `DYLD_LIBRARY_PATH`.
- The generated loader `generated/node/index.js` requires `index.node` in the same directory.
