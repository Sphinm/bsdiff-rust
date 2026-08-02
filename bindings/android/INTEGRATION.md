# Android Integration Guide

## 1. Build the .so files

```bash
cd bsdiff-rust

# One-time setup:
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
cargo install cargo-ndk

# Build:
bash scripts/build-android.sh
# → bindings/android/jniLibs/{arm64-v8a,armeabi-v7a,x86_64,x86}/libbsdiff_core.so
```

## 2. Copy to the Android project

```bash
cp -r bindings/android/jniLibs/* \
  native/paylater-android/commonBusinessCodes/src/main/jniLibs/
```

And copy the Kotlin files:
```bash
cp bindings/android/jni/BsdiffCoreJni.kt \
  native/paylater-android/commonBusinessCodes/src/main/java/com/atome/bsdiff/
```

## 3. Replace the existing PatchUtils usage

### Current code (MergePatchWork3.kt)

```kotlin
import com.atome.bsdiff.PatchUtils  // ← Remove

// In merge():
PatchUtils().patch(oldFilePath, newFilePath, patchFilePath)
```

### After

```kotlin
import com.atome.bsdiff.BsdiffCore  // ← New

// In merge():
BsdiffCore.patch(oldFilePath, newFilePath, patchFilePath)
```

## 4. Optional: Validate patch before applying

```kotlin
if (BsdiffCore.isValidPatch(patchFilePath)) {
    BsdiffCore.patch(oldFilePath, newFilePath, patchFilePath)
} else {
    // fallback: download full package
    throw BsdiffException("Invalid patch file: $patchFilePath")
}
```

## 5. Remove old dependency

In `commonBusinessCodes/build.gradle`:
```groovy
// Remove the old bsdiff dependency if it's pulled separately
// (PatchUtils is part of offlinepackage SDK, but if bsdiff is a separate dep:)
// implementation 'com.atome.bsdiff:bsdiff:x.x.x'  ← Remove
```
