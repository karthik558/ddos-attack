use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct UdpSocketWrapper {
    socket: Option<UdpSocket>,
}

impl UdpSocketWrapper {
    pub async fn new() -> Result<Self> {
        Ok(Self { socket: None })
    }
    
    pub async fn bind(&mut self, addr: &SocketAddr) -> Result<()> {
        self.socket = Some(UdpSocket::bind(addr).await?);
        Ok(())
    }
    
    pub async fn send_to(&self, data: &[u8], target: &SocketAddr) -> Result<usize> {
        if let Some(socket) = &self.socket {
            Ok(socket.send_to(data, target).await?)
        } else {
            // Create a temporary socket for sending
            let socket = UdpSocket::bind("0.0.0.0:0").await?;
            Ok(socket.send_to(data, target).await?)
        }
    }
    
    pub async fn recv_from(&self, buffer: &mut [u8]) -> Result<(usize, SocketAddr)> {
        if let Some(socket) = &self.socket {
            Ok(socket.recv_from(buffer).await?)
        } else {
            anyhow::bail!("Socket not bound")
        }
    }
    
    pub async fn send_broadcast(data: &[u8], port: u16) -> Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.set_broadcast(true)?;
        
        let broadcast_addr: SocketAddr = format!("255.255.255.255:{}", port).parse()?;
        socket.send_to(data, &broadcast_addr).await?;
        
        Ok(())
    }
}
