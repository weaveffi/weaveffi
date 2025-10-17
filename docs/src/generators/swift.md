# Swift

The Swift generator emits a SwiftPM System Library (`CWeaveFFI`) that references
the generated C header and a thin Swift module (`WeaveFFI`) that wraps the C API
with Swift types and `throws`-based error handling.

## Generated artifacts

- `generated/swift/Package.swift`
- `generated/swift/WeaveFFI/module.modulemap` – points at `../c/weaveffi.h`
- `generated/swift/Sources/WeaveFFI/WeaveFFI.swift` – thin wrapper

## Try the example app

```bash
# Build the Rust sample
cargo build -p calculator

cd examples/swift
swiftc \
  -I ../../generated/swift/Sources/CWeaveFFI \
  -L ../../target/debug -lcalculator \
  -Xlinker -rpath -Xlinker ../../target/debug \
  Sources/App/main.swift -o .build/debug/App
DYLD_LIBRARY_PATH=../../target/debug .build/debug/App
```

Integration via SwiftPM in a real app can be done by adding the System Library
and linking it with your target; see the module map for header linkage and name.
