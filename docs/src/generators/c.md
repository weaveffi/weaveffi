# C

The C generator emits a single header `weaveffi.h` containing function prototypes,
error types, and memory helpers; it also includes an optional `weaveffi.c` placeholder
for future convenience wrappers.

## Generated artifacts

- `generated/c/weaveffi.h`
- `generated/c/weaveffi.c`

Key declarations:

```c
typedef struct weaveffi_error { int32_t code; const char* message; } weaveffi_error;
void weaveffi_error_clear(weaveffi_error* err);
void weaveffi_free_string(const char* ptr);
void weaveffi_free_bytes(uint8_t* ptr, size_t len);
```

## Build and run (calculator sample)

```bash
# Build the Rust sample library (produces libcalculator)
cargo build -p calculator

cd examples/c
cc -I ../../generated/c main.c -L ../../target/debug -lcalculator -o c_example
DYLD_LIBRARY_PATH=../../target/debug ./c_example
# On Linux, use LD_LIBRARY_PATH
```

See `examples/c/main.c` for usage of errors and returned strings.
