#!/bin/bash
set -euo pipefail

if ! command -v cargo &> /dev/null; then
    echo "Rust is not installed. Install it from https://rustup.rs"
    exit 1
fi

if ! command -v cargo-ndk &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
fi

echo "Adding Android aarch64 target..."
rustup target add aarch64-linux-android

if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    echo "ANDROID_NDK_HOME not set. Searching for NDK..."
    POSSIBLE_DIRS=(
        "$HOME/Android/Sdk/ndk"
        "$HOME/android-ndk"
        "/opt/android-ndk"
        "/usr/local/android-ndk"
    )
    for dir in "${POSSIBLE_DIRS[@]}"; do
        if [ -d "$dir" ]; then
            export ANDROID_NDK_HOME=$(ls -d "$dir"/*/ 2>/dev/null | head -1 | tr -d '\n')
            if [ -n "$ANDROID_NDK_HOME" ]; then
                echo "Found NDK at $ANDROID_NDK_HOME"
                break
            fi
        fi
    done
fi

if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    echo "ANDROID_NDK_HOME not set. Please set it to your NDK path."
    exit 1
fi

echo "Building for arm64-v8a..."
cargo ndk -t arm64-v8a -o module/zygisk build --release
mv module/zygisk/arm64-v8a/libzygisk.so module/zygisk/arm64-v8a.so
rm -rf module/zygisk/arm64-v8a

echo "Creating module zip..."
(cd module && zip -r9 "$OLDPWD/zygisk_spoof.zip" . -x ".*" -x "*/.*")

echo ""
echo "Done! Module zip: zygisk_spoof.zip"
echo "Install via Magisk Manager"
