#!/usr/bin/env bash
set -euo pipefail

# Build bsdiff-core as XCFramework for iOS.
# Output: bindings/ios/BsdiffCore.xcframework/
#
# Prerequisites:
#   rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
#   Full Xcode installed (not just CommandLineTools)

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUTPUT_XCFRAMEWORK="$REPO_ROOT/bindings/ios/BsdiffCore.xcframework"
RUST_DIR="$REPO_ROOT/core"

IOS_ARM64="aarch64-apple-ios"
IOS_SIM_ARM64="aarch64-apple-ios-sim"
IOS_SIM_X86_64="x86_64-apple-ios"

echo "==> Building bsdiff-core for iOS..."

# Build device target (arm64)
echo "  -> $IOS_ARM64"
cargo build --manifest-path "$RUST_DIR/Cargo.toml" --release --target "$IOS_ARM64"

# Build simulator targets
echo "  -> $IOS_SIM_ARM64"
cargo build --manifest-path "$RUST_DIR/Cargo.toml" --release --target "$IOS_SIM_ARM64"

echo "  -> $IOS_SIM_X86_64"
cargo build --manifest-path "$RUST_DIR/Cargo.toml" --release --target "$IOS_SIM_X86_64"

# Prepare staging — separate dirs for device and simulator
STAGING="$REPO_ROOT/target/ios-staging"
rm -rf "$STAGING"
mkdir -p "$STAGING/device/Headers" "$STAGING/simulator/Headers"

# Copy header
cp "$REPO_ROOT/bindings/ios/bsdiff.h" "$STAGING/device/Headers/"
cp "$REPO_ROOT/bindings/ios/bsdiff.h" "$STAGING/simulator/Headers/"

# Device: just arm64
cp "target/$IOS_ARM64/release/libbsdiff_core.a" "$STAGING/device/"

# Simulator: arm64 + x86_64 fat lib
lipo -create \
    "target/$IOS_SIM_ARM64/release/libbsdiff_core.a" \
    "target/$IOS_SIM_X86_64/release/libbsdiff_core.a" \
    -output "$STAGING/simulator/libbsdiff_core.a"

echo "==> Creating XCFramework..."
rm -rf "$OUTPUT_XCFRAMEWORK"
xcodebuild -create-xcframework \
    -library "$STAGING/device/libbsdiff_core.a" \
    -headers "$STAGING/device/Headers" \
    -library "$STAGING/simulator/libbsdiff_core.a" \
    -headers "$STAGING/simulator/Headers" \
    -output "$OUTPUT_XCFRAMEWORK"

echo "==> Done: $OUTPUT_XCFRAMEWORK"
echo ""
echo "=== XCFramework structure ==="
find "$OUTPUT_XCFRAMEWORK" -type f -exec ls -lh {} \;
echo ""
echo "=== Architectures ==="
for f in "$OUTPUT_XCFRAMEWORK"/*/libbsdiff_core.a; do
    echo "$f: $(lipo -info "$f" 2>/dev/null || file "$f")"
done
