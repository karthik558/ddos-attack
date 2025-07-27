#!/bin/bash

# Deployment script for DDoS Attack Tool - Rust Edition
# This script sets up the environment and installs dependencies

set -e

INSTALL_DIR="/opt/ddos-attack"
CONFIG_DIR="$HOME/.config/ddos-attack"
BIN_DIR="/usr/local/bin"

echo "DDoS Attack Tool - Rust Edition Installer"
echo "========================================="

# Check for root privileges
if [[ $EUID -eq 0 ]]; then
    echo "Running as root - system-wide installation"
    SYSTEM_INSTALL=true
else
    echo "Running as user - local installation"
    SYSTEM_INSTALL=false
    INSTALL_DIR="$HOME/.local/share/ddos-attack"
    BIN_DIR="$HOME/.local/bin"
fi

# Check dependencies
echo "Checking dependencies..."

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Check Tor (optional)
if ! command -v tor &> /dev/null; then
    echo "Warning: Tor not found. Installing Tor..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y tor
        elif command -v yum &> /dev/null; then
            sudo yum install -y tor
        elif command -v pacman &> /dev/null; then
            sudo pacman -S tor
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v brew &> /dev/null; then
            brew install tor
        else
            echo "Please install Homebrew first, then run: brew install tor"
        fi
    fi
fi

# Create directories
echo "Creating directories..."
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"
mkdir -p "$BIN_DIR"

# Copy files
echo "Copying files..."
cp -r . "$INSTALL_DIR"

# Build the project
echo "Building project..."
cd "$INSTALL_DIR"
cargo build --release

# Create symlink
echo "Creating symlink..."
ln -sf "$INSTALL_DIR/target/release/ddos-attack" "$BIN_DIR/ddos-attack"

# Copy config file
if [ ! -f "$CONFIG_DIR/config.toml" ]; then
    cp config.toml "$CONFIG_DIR/config.toml"
    echo "Configuration file created at $CONFIG_DIR/config.toml"
fi

# Set permissions
if [ "$SYSTEM_INSTALL" = true ]; then
    # For IP spoofing capabilities
    setcap cap_net_raw+ep "$INSTALL_DIR/target/release/ddos-attack" 2>/dev/null || {
        echo "Warning: Could not set raw socket capabilities. Run as root for IP spoofing."
    }
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "Binary installed to: $BIN_DIR/ddos-attack"
echo "Configuration: $CONFIG_DIR/config.toml"
echo ""
echo "Usage examples:"
echo "  ddos-attack layer4 --targets \"192.168.1.1\" --protocol tcp"
echo "  ddos-attack layer7 --targets \"https://example.com\""
echo "  ddos-attack tor --start"
echo ""
echo "⚠️  IMPORTANT: Use this tool only for authorized testing!"
echo "⚠️  Read the documentation and legal notices before use."

# Add to PATH if not already there
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo ""
    echo "Add to your shell profile:"
    echo "  export PATH=\"$BIN_DIR:\$PATH\""
fi
