Run the Swift example

1. Ensure `libcalculator.dylib` is built in `target/debug`.
2. From this directory:

```
# Build the Rust library (from repo root)
cargo build -p calculator

# Compile and run the Swift example directly against the C system module
mkdir -p .build/debug
swiftc \
  -I ../../generated/swift/Sources/CWeaveFFI \
  -L ../../target/debug -lcalculator \
  -Xlinker -rpath -Xlinker ../../target/debug \
  Sources/App/main.swift -o .build/debug/App

DYLD_LIBRARY_PATH=../../target/debug .build/debug/App
```
