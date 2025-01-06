# Distributed Denial of Service (DDoS) Attack

A Distributed Denial of Service (DDoS) attack is a type of denial of service attack in which the incoming traffic flooding the victim so that it cannot respond to legitimate traffic. This project is a simple implementation of a DDoS attack using Python and Scapy.

![](https://img.shields.io/github/license/karthik558/ddos-attack?style=for-the-badge)
![](https://img.shields.io/github/forks/karthik558/ddos-attack?style=for-the-badge)
![](https://img.shields.io/github/stars/karthik558/ddos-attack?style=for-the-badge)
![](https://img.shields.io/github/issues/karthik558/ddos-attack?style=for-the-badge)
![](https://img.shields.io/github/languages/code-size/karthik558/ddos-attack?style=for-the-badge)

![IMAGE](./assets/banner.png)

## Table of Contents

- [Getting Started](#getting-started)
- [Requirements](#requirements)
- [Usage](#usage)
- [Configuration](#configuration)
- [Example](#example)
- [Logging](#logging)
- [Disclaimer](#disclaimer)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Getting Started

To get started with the project, simply clone the repository to your local machine:

```
git clone https://github.com/karthik558/ddos-attack.git
```

## Requirements

- Python 3.6 or higher
- `scapy` library for packet crafting
- `PySocks` library for Tor support
- Tor service installed and running on your machine

## Usage

- `cd ddos-attack`
- `pip install -r requirements.txt`
- `python3 ddos.py`

### Input Prompts

- IP Targets: List of IPs separated by commas (e.g., 192.168.0.1, 192.168.0.2).
- Ports: List of ports to attack, separated by commas.
- Proxy Size: Number of packets sent per iteration (default: 10).
- Rate Limit: Time delay between packets (default: 0.1 seconds).
- Threads: Number of threads to use (default: 20).
- Send via Tor: Option to use Tor for anonymity (y/n, default: y).

### Configuration

- Ports: [80, 443]
- Rate Limit: 0.1 seconds
- Threads: 20
- Send via Tor: Yes

### Example

```
$ python ddos_attack_tool.py
IP Targets (separated by commas): 192.168.0.1, 192.168.0.2
Ports (separated by commas): 80, 443
Proxy Size: 10
Rate Limit (seconds between packets): 0.1
User-Agent: Mozilla/5.0
Data Size (bytes): 600
Threads: 20
Send via Tor? (y/n): y
```

## Logging

This tool is for educational purposes only. Do not use it for any malicious or illegal activity. Always ensure you have permission to test any network.

## Disclaimer

The author is not responsible for any misuse of this tool. Please use responsibly and ethically.

## Contributing:

Contributions to the project are welcome. If you would like to suggest an improvement or report a bug, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Acknowledgments

- [Python](https://www.python.org/)
- [Tor](https://www.torproject.org/)
