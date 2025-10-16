Android example notes

The `generated/android` folder contains a Gradle library template and JNI stubs.
To build:

1. Ensure Android SDK and NDK are installed (Android Studio recommended).
2. Open `generated/android` in Android Studio as a project.
3. Sync Gradle and build the `:weaveffi` AAR.
4. Integrate the AAR into your app module, and ensure the native library
   loader finds `libcalculator` at runtime on device/emulator.
