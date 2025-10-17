## WeaveFFI Roadmap (Rust core)

This roadmap tracks high-level goals for the first five releases, with a detailed
step-by-step plan to reach 0.1.0. The project uses a Rust core that exposes an
FFI-friendly, stable C ABI used by language-specific bindings.

### Release goals

- **0.1.0 — MVP foundation**: Rust workspace, IDL/IR v0, stable C ABI, basic type
  coverage, error and memory model, code generators for C header, Swift (SwiftPM
  System Library), Android (JNI skeleton), Node.js (TypeScript + ffi-napi),
  minimal Web/WASM target, CLI, samples, and docs.
- **0.2.0 — Type system + packaging**: Structs/enums/options, arrays/slices, richer
  string/byte handling, first-class async surface (callbacks → futures), packaging
  improvements (SwiftPM/Gradle/npm templates), better cross-compilation UX.
- **0.3.0 — Annotated Rust input**: Support reading an annotated Rust crate as input
  (derive/proc-macro), advanced async/concurrency (streams), improved diagnostics,
  and template customization hooks.
- **0.4.0 — Safety + performance**: Zero-copy where safe, arena/pool patterns,
  lifetime-safe handles, incremental codegen, template plugins, caching, and DX
  polish across all targets. (High-level goals; detailed plan TBD.)
- **0.5.0 — Ecosystem expansion**: Additional languages (e.g., Python, .NET),
  distribution story (prebuilt artifacts), stability hardening, and release
  automation. (High-level goals; detailed plan TBD.)

---

### 0.1.0 detailed plan (MVP)

Focus: Deliver a usable MVP that accepts a simple IDL/IR, generates a stable C
ABI plus minimal bindings/templates for Swift, Android, Node.js, and Web/WASM,
and ships a CLI with a working sample.

#### 1) Workspace scaffolding
- Create a Rust workspace with crates:
  - `weaveffi-ir`: in-memory IR + (de)serialization via `serde` (YAML/TOML/JSON)
  - `weaveffi-core`: core logic (validation, codegen orchestration, templates)
  - `weaveffi-gen-c`: C header generator (and helper C stubs if needed)
  - `weaveffi-gen-swift`: SwiftPM System Library template + thin Swift wrapper
  - `weaveffi-gen-android`: JNI glue + Gradle module template (Kotlin)
  - `weaveffi-gen-node`: TypeScript template using `ffi-napi` to load the .dylib/.so
  - `weaveffi-cli`: end-user CLI (`weaveffi`) invoking core/generators
  - `samples/calculator`: tiny Rust lib compiled to C ABI for end-to-end testing

#### 2) IR/IDL v0 (input model)
- Define a minimal but practical schema to describe:
  - Functions (name, doc, params, return, async=false for 0.1.0)
  - Types: primitives (i32, u32, i64, f64, bool), `string` (UTF-8), `bytes`,
    and `handle` (opaque resource IDs); no nested structs for 0.1.0
  - Errors: named error domain with numeric codes + messages
- Implement parsers via `serde` for YAML and/or TOML; emit helpful diagnostics.
- Validate IR (name collisions, reserved keywords, unsupported shapes, etc.).

#### 3) ABI, memory, and error model
- Establish a stable C ABI surface convention (prefix, naming, versioning):
  - `weaveffi_<module>_<function>(... , weaveffi_error* out_err)` style
  - All strings returned are UTF-8, owned by the Rust core; provide
    `weaveffi_free_string(const char*)` and `weaveffi_free_bytes(uint8_t*, size_t)`
  - Opaque `handle` represented as `uint64_t` (or `uintptr_t`) from user perspective
- Error model:
  - A compact `weaveffi_error` struct with `{ code: int32_t, const char* message }`
  - Map Rust `Result<T, E>` to C: fill `out_err->code != 0` on error
  - Provide `weaveffi_error_clear(weaveffi_error*)` to release message buffers

#### 4) Code generators (templates)
- C generator (`weaveffi-gen-c`):
  - Emit a single `.h` with function prototypes, error types, free functions
  - Optionally emit a tiny `.c` convenience layer if helpful for some targets
- Swift generator (`weaveffi-gen-swift`):
  - SwiftPM System Library template with `module.modulemap` referencing the header
  - Thin Swift wrapper translating to ergonomic Swift types and throwing errors
- Android generator (`weaveffi-gen-android`):
  - Kotlin (or Java) JNI wrapper class + Gradle `android-library` template
  - C JNI shims that forward to the C ABI; sample `CMakeLists.txt`
- Node generator (`weaveffi-gen-node`):
  - TypeScript wrapper using `ffi-napi` to load the C ABI shared library at runtime
  - Generate `.d.ts` types from the IR; include basic build scripts
- Web/WASM minimal (`weaveffi-core`):
  - Documented `wasm32-unknown-unknown` build with a thin JS glue stub; full
    ergonomics can wait for 0.2.x/0.3.x

#### 5) CLI
- `weaveffi new <name>`: create a starter layout (IDL + example module)
- `weaveffi generate`: read IDL → validate IR → emit C header + platform templates
- `weaveffi doctor`: check for toolchain prerequisites (Rust, Xcode, Android NDK,
  Node toolchain), reporting actionable guidance

#### 6) Sample: calculator
- Rust `samples/calculator` crate exporting a few functions via the C ABI:
  - `add(i32, i32) -> i32`, `mul(i32, i32) -> i32`, `echo(string) -> string`
  - One fallible function returning an error code/message
- Include ready-to-run generated outputs for each target in a `examples/` folder.

#### 7) Tooling and CI
- GitHub Actions workflow: build `weaveffi-cli` for macOS and Linux; run unit tests
- Basic integration test: generate bindings from the calculator IDL and compile
  the produced templates (at least header + Node wrapper) in CI

#### 8) Documentation
- `README` quickstart and link to this roadmap
- Docs pages for: IDL schema, memory & error model, per-platform setup/run steps
- End-to-end tutorial using the calculator sample

#### 9) Release and versioning
- Tag `v0.1.0`; attach CLI binaries for macOS (arm64/x86_64) and Linux (x86_64)
- Publish npm template (if applicable) as a starter, and provide SwiftPM/Gradle
  template repos or archives

#### 10) Acceptance checklist
- CLI can read a calculator IDL, generate artifacts, and the artifacts compile
- C header compiles; Node wrapper can call into the shared library successfully
- SwiftPM System Library builds and links on macOS/iOS simulator locally
- Android template builds an `.aar` with JNI stubs (smoke test)
- Docs explain memory management and error handling clearly
