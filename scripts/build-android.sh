#!/usr/bin/env bash
set -euo pipefail

# Build libbsdiff_android.so for Android (all ABIs) using cargo-ndk.
#
# Prerequisites:
#   rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
#   cargo install cargo-ndk
#   ANDROID_NDK_HOME must be set
#
# Output: bindings/android/jniLibs/{abi}/libbsdiff_android.so

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUTPUT_DIR="$REPO_ROOT/bindings/android/jniLibs"
RUST_DIR="$REPO_ROOT/bindings/android/rust"

if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    echo "ERROR: ANDROID_NDK_HOME is not set"
    exit 1
fi

echo "==> Building bsdiff_android for Android (cargo-ndk)..."

# Clean previous output
rm -rf "$OUTPUT_DIR"

cargo ndk \
    --target aarch64-linux-android \
    --target armv7-linux-androideabi \
    --target x86_64-linux-android \
    --target i686-linux-android \
    --platform 21 \
    --output-dir "$OUTPUT_DIR" \
    build --manifest-path "$RUST_DIR/Cargo.toml" --release

echo "==> Done. jniLibs:"
find "$OUTPUT_DIR" -name "*.so" -exec ls -lh {} \;
