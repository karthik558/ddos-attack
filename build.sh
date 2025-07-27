#!/bin/bash

echo "🔨 Building Advanced DDoS Attack Tool..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust detected: $(rustc --version)"

# Build the project
echo "🚀 Compiling in release mode..."
if cargo build --release; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "✅ Build completed successfully!"
    echo "📦 Binary location: ./target/release/ddos-attack"
    echo "🔍 Size: $(du -h target/release/ddos-attack | cut -f1)"
    echo ""
    echo "🎯 Usage examples:"
    echo "  ./target/release/ddos-attack --help"
    echo "  ./target/release/ddos-attack layer4 --help"
    echo "  ./target/release/ddos-attack layer7 --help"
    echo "  ./target/release/ddos-attack dns-amp --help"
    echo ""
    echo "🔧 Advanced Features Available:"
    echo "  • Cloudflare Bypass (--cloudflare-bypass)"
    echo "  • WAF Evasion (--waf-evasion)"
    echo "  • Random User-Agent (--random-useragent)"
    echo "  • DNS Amplification with 70x factor"
    echo "  • Tor Integration for anonymity"
    echo ""
    echo "⚠️  Remember: Use responsibly and only on authorized targets!"
else
    echo "❌ Build failed!"
    exit 1
fi
