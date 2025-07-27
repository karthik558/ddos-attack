use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct TcpSocket {
    listener: Option<TcpListener>,
}

impl TcpSocket {
    pub async fn new() -> Result<Self> {
        Ok(Self { listener: None })
    }
    
    pub async fn bind(&mut self, addr: &SocketAddr) -> Result<()> {
        self.listener = Some(TcpListener::bind(addr).await?);
        Ok(())
    }
    
    pub async fn connect(addr: &SocketAddr) -> Result<tokio::net::TcpStream> {
        Ok(tokio::net::TcpStream::connect(addr).await?)
    }
    
    pub async fn send_data(stream: &mut tokio::net::TcpStream, data: &[u8]) -> Result<()> {
        stream.write_all(data).await?;
        Ok(())
    }
    
    pub async fn receive_data(stream: &mut tokio::net::TcpStream, buffer: &mut [u8]) -> Result<usize> {
        Ok(stream.read(buffer).await?)
    }
}
