#!/usr/bin/env bash
# Build SCMessengerCore.xcframework for iOS
#
# Usage: scripts/build_xcframework.sh
#
# Produces: ios/SCMessengerCore.xcframework
# Contains: arm64 (device) + arm64-sim (simulator) static libraries
# with the generated Swift bindings.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

DEVICE_TARGET="aarch64-apple-ios"
SIM_TARGET="aarch64-apple-ios-sim"
BUILD_DIR="$ROOT_DIR/target/xcframework"
OUTPUT="$ROOT_DIR/ios/SCMessengerCore.xcframework"

echo "Building Rust static libraries..."

cargo build --target "$DEVICE_TARGET" -p scmessenger-core --release
cargo build --target "$SIM_TARGET" -p scmessenger-core --release

echo "Generating Swift bindings..."

cargo run --bin gen_swift --features gen-bindings

echo "Creating xcframework..."

rm -rf "$OUTPUT"
mkdir -p "$BUILD_DIR"

xcodebuild -create-xcframework \
    -library "$ROOT_DIR/target/$DEVICE_TARGET/release/libscmessenger_core.a" \
    -headers "$ROOT_DIR/core/target/generated-sources/uniffi/swift/" \
    -library "$ROOT_DIR/target/$SIM_TARGET/release/libscmessenger_core.a" \
    -headers "$ROOT_DIR/core/target/generated-sources/uniffi/swift/" \
    -output "$OUTPUT"

rm -rf "$BUILD_DIR"

echo "xcframework created at: $OUTPUT"
