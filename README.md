WeaveFFI is a code generator that takes a single C API (from an annotated header
or IDL) and produces idiomatic bindings/packages for Swift (iOS), Kotlin/Java
(Android), Node.js (N-API + TypeScript), and Web (WASM + JS). Each platform
calls the same C core, eliminating separate re-implementations and hand-written
JNI/bridging. It handles types, memory/ownership, errors, and async (mapping
callbacks to async/await or Promises), and ships build/packaging scaffolds
(SwiftPM, Gradle, npm). Result: one audited, high-performance core with
consistent behavior and lockstep releases across mobile, desktop, and web.
