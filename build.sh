#!/bin/bash

echo "Building DDoS Attack Tool - Rust Edition"
echo "========================================"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust toolchain."
    exit 1
fi

echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Build in release mode
echo "Building in release mode..."
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo "Binary location: ./target/release/ddos-attack"
    echo ""
    echo "Usage examples:"
    echo "  Layer 4 TCP: ./target/release/ddos-attack layer4 --targets \"192.168.1.1\" --protocol tcp"
    echo "  Layer 7 HTTP: ./target/release/ddos-attack layer7 --targets \"https://example.com\""
    echo "  Tor management: ./target/release/ddos-attack tor --start"
    echo ""
    echo "⚠️  Note: Some features require root privileges (IP spoofing)"
    echo "⚠️  Remember: Use this tool only for authorized testing!"
else
    echo "❌ Build failed!"
    exit 1
fi
