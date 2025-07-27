#!/bin/bash

echo "📦 Installing Advanced DDoS Attack Tool..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if binary exists
if [ ! -f "./target/release/ddos-attack" ]; then
    echo "❌ Binary not found! Please run ./build.sh first."
    exit 1
fi

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "⚠️  This script requires root privileges for system-wide installation."
    echo "   Please run: sudo ./install.sh"
    exit 1
fi

# Install locations
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="ddos-attack"
DOCS_DIR="/usr/local/share/doc/ddos-attack"

echo "🔧 Installing to system directories..."

# Copy binary
cp "./target/release/ddos-attack" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Create docs directory
mkdir -p "$DOCS_DIR"

# Copy documentation
cp README.md "$DOCS_DIR/"
cp LICENSE "$DOCS_DIR/"

# Create examples directory
mkdir -p "$DOCS_DIR/examples"
if [ -d "examples" ]; then
    cp examples/* "$DOCS_DIR/examples/"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Installation completed successfully!"
echo ""
echo "📍 Binary installed to: $INSTALL_DIR/$BINARY_NAME"
echo "📚 Documentation: $DOCS_DIR"
echo ""
echo "🎯 You can now run the tool from anywhere:"
echo "  ddos-attack --help"
echo "  ddos-attack layer7 --targets \"https://example.com\" --cloudflare-bypass"
echo ""
echo "🔧 Advanced Features:"
echo "  • Cloudflare Challenge Solving"
echo "  • WAF Bypass Techniques"
echo "  • DNS Amplification (70x factor)"
echo "  • User-Agent Database (1000+ signatures)"
echo "  • Tor Integration"
echo ""
echo "⚠️  LEGAL NOTICE:"
echo "   This tool is for authorized penetration testing only."
echo "   The developers are not responsible for misuse."
echo "   Always ensure you have proper authorization."
