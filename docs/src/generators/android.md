# Android

The Android generator produces a Gradle `android-library` template with:
- Kotlin wrapper `WeaveFFI` that declares `external fun`s
- JNI C shims that call into the generated C ABI
- `CMakeLists.txt` for building the shared library

## Generated artifacts

- `generated/android/settings.gradle`
- `generated/android/build.gradle`
- `generated/android/src/main/java/com/weaveffi/WeaveFFI.kt`
- `generated/android/src/main/cpp/{weaveffi_jni.c,CMakeLists.txt}`

## Build steps

1. Ensure Android SDK and NDK are installed (Android Studio recommended).
2. Open `generated/android` in Android Studio.
3. Sync Gradle and build the `:weaveffi` AAR.
4. Integrate the AAR into your app module. Ensure your app loads the Rust-produced
   native library (e.g., `libcalculator`) at runtime on device/emulator.

The JNI shims convert strings/bytes and propagate errors by throwing `RuntimeException`.
