use anyhow::Result;
use log::{info, error};
use std::net::SocketAddr;
use std::process::Stdio;
use tokio::net::TcpStream;
use tokio::process::Command;

pub struct TorClient {
    is_running: bool,
}

impl TorClient {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            is_running: false,
        })
    }
    
    pub async fn start(&mut self) -> Result<()> {
        if self.is_running {
            return Ok(());
        }
        
        info!("Starting Tor client...");
        
        // Start Tor daemon with basic configuration
        self.start_tor_daemon().await?;
        
        self.is_running = true;
        
        info!("Tor client started successfully");
        Ok(())
    }
    
    pub async fn stop(&mut self) -> Result<()> {
        if !self.is_running {
            return Ok(());
        }
        
        info!("Stopping Tor client...");
        
        self.stop_tor_daemon().await?;
        self.is_running = false;
        
        info!("Tor client stopped");
        Ok(())
    }
    
    pub async fn renew_identity(&self) -> Result<()> {
        // Send NEWNYM signal to Tor
        self.send_tor_signal("NEWNYM").await?;
        info!("Tor identity renewed");
        Ok(())
    }
    
    pub async fn connect_tcp(&self, _addr: &SocketAddr) -> Result<TcpStream> {
        // This is a simplified implementation
        // In practice, you would connect through the SOCKS proxy
        anyhow::bail!("Direct Tor TCP connection requires SOCKS proxy implementation");
    }
    
    pub async fn get_tor_ip(&self) -> Result<String> {
        use reqwest::Client;
        
        let client = Client::builder()
            .proxy(reqwest::Proxy::all("socks5://127.0.0.1:9050")?)
            .build()?;
            
        let response = client
            .get("https://httpbin.org/ip")
            .send()
            .await?
            .text()
            .await?;
            
        Ok(response)
    }
    
    async fn start_tor_daemon(&self) -> Result<()> {
        info!("Starting Tor daemon...");
        
        // Create Tor configuration
        let tor_config = r#"
SocksPort 9050
ControlPort 9051
DataDirectory /tmp/tor-data-rust
Log notice file /tmp/tor.log
RunAsDaemon 1
"#;
        
        // Write config to temp file
        tokio::fs::write("/tmp/torrc-rust", tor_config).await?;
        
        let mut child = Command::new("tor")
            .arg("-f")
            .arg("/tmp/torrc-rust")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
            
        // Wait a moment and check if process is still running
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        match child.try_wait()? {
            Some(status) => {
                if !status.success() {
                    anyhow::bail!("Tor process exited with error: {}", status);
                }
            }
            None => {
                // Process is still running, which is good
                info!("Tor daemon started successfully");
            }
        }
        
        // Wait for Tor to be ready
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        
        Ok(())
    }
    
    async fn stop_tor_daemon(&self) -> Result<()> {
        info!("Stopping Tor daemon...");
        
        let output = Command::new("pkill")
            .arg("-f")
            .arg("tor")
            .output()
            .await?;
            
        if !output.status.success() {
            error!("Failed to stop Tor daemon cleanly");
        }
        
        // Clean up temp files
        let _ = tokio::fs::remove_file("/tmp/torrc-rust").await;
        let _ = tokio::fs::remove_dir_all("/tmp/tor-data-rust").await;
        
        info!("Tor daemon stopped");
        Ok(())
    }
    
    async fn send_tor_signal(&self, signal: &str) -> Result<()> {
        use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
        
        let stream = TcpStream::connect("127.0.0.1:9051").await?;
        let (read_half, mut write_half) = stream.into_split();
        
        // Authenticate (no password for simplicity)
        write_half.write_all(b"AUTHENTICATE\r\n").await?;
        
        let mut reader = BufReader::new(read_half);
        let mut response = String::new();
        reader.read_line(&mut response).await?;
        
        if !response.starts_with("250") {
            anyhow::bail!("Authentication failed: {}", response);
        }
        
        // Send signal
        let command = format!("SIGNAL {}\r\n", signal);
        write_half.write_all(command.as_bytes()).await?;
        
        response.clear();
        reader.read_line(&mut response).await?;
        
        if !response.starts_with("250") {
            anyhow::bail!("Signal failed: {}", response);
        }
        
        Ok(())
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn get_socks_proxy_url(&self) -> String {
        "socks5://127.0.0.1:9050".to_string()
    }
}
