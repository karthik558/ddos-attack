use rand::Rng;
use std::collections::HashMap;

pub struct FingerprintEvasion {
    ja3_profiles: Vec<JA3Profile>,
    http2_settings: Vec<Http2Settings>,
    tls_extensions: Vec<TlsExtension>,
}

#[derive(Clone)]
pub struct JA3Profile {
    pub name: String,
    pub version: String,
    pub cipher_suites: Vec<u16>,
    pub extensions: Vec<u16>,
    pub elliptic_curves: Vec<u16>,
    pub ec_point_formats: Vec<u8>,
}

#[derive(Clone)]
pub struct Http2Settings {
    pub header_table_size: u32,
    pub enable_push: bool,
    pub max_concurrent_streams: u32,
    pub initial_window_size: u32,
    pub max_frame_size: u32,
    pub max_header_list_size: u32,
}

#[derive(Clone)]
pub struct TlsExtension {
    pub extension_type: u16,
    pub data: Vec<u8>,
}

impl FingerprintEvasion {
    pub fn new() -> Self {
        Self {
            ja3_profiles: Self::load_ja3_profiles(),
            http2_settings: Self::load_http2_settings(),
            tls_extensions: Self::load_tls_extensions(),
        }
    }

    fn load_ja3_profiles() -> Vec<JA3Profile> {
        vec![
            // Chrome 120
            JA3Profile {
                name: "Chrome".to_string(),
                version: "120.0".to_string(),
                cipher_suites: vec![
                    0x1301, 0x1302, 0x1303, 0xc02c, 0xc02b, 0xc030, 0xc02f,
                    0xc028, 0xc027, 0xc014, 0xc013, 0x009f, 0x009e, 0x006b,
                    0x0067, 0x0039, 0x0033, 0x009d, 0x009c, 0x003d, 0x003c,
                    0x0035, 0x002f, 0x00ff
                ],
                extensions: vec![
                    0x0000, 0x0005, 0x000a, 0x000b, 0x000d, 0x0012, 0x0015,
                    0x0017, 0x0018, 0x001b, 0x0023, 0x002b, 0x002d, 0x0033,
                    0x4469, 0xff01
                ],
                elliptic_curves: vec![0x001d, 0x0017, 0x0018, 0x0019],
                ec_point_formats: vec![0x00],
            },
            
            // Firefox 121
            JA3Profile {
                name: "Firefox".to_string(),
                version: "121.0".to_string(),
                cipher_suites: vec![
                    0x1301, 0x1302, 0x1303, 0xc02c, 0xc02b, 0xc030, 0xc02f,
                    0xc028, 0xc027, 0xc014, 0xc013, 0x009f, 0x009e, 0x006b,
                    0x0067, 0x0039, 0x0033
                ],
                extensions: vec![
                    0x0000, 0x0005, 0x000a, 0x000b, 0x000d, 0x0012, 0x0015,
                    0x0017, 0x0018, 0x001b, 0x0023, 0x002b, 0x002d, 0x0033
                ],
                elliptic_curves: vec![0x001d, 0x0017, 0x001e, 0x0019, 0x0018],
                ec_point_formats: vec![0x00],
            },

            // Safari 17
            JA3Profile {
                name: "Safari".to_string(),
                version: "17.2".to_string(),
                cipher_suites: vec![
                    0x1301, 0x1302, 0x1303, 0xc02c, 0xc02b, 0xc030, 0xc02f,
                    0xc028, 0xc027, 0xc014, 0xc013
                ],
                extensions: vec![
                    0x0000, 0x0005, 0x000a, 0x000b, 0x000d, 0x0012, 0x0015,
                    0x0017, 0x0018, 0x001b, 0x0023, 0x002b, 0x002d
                ],
                elliptic_curves: vec![0x001d, 0x0017, 0x0018],
                ec_point_formats: vec![0x00],
            },
        ]
    }

    fn load_http2_settings() -> Vec<Http2Settings> {
        vec![
            // Chrome HTTP/2 settings
            Http2Settings {
                header_table_size: 65536,
                enable_push: false,
                max_concurrent_streams: 1000,
                initial_window_size: 6291456,
                max_frame_size: 16777215,
                max_header_list_size: 0,
            },
            
            // Firefox HTTP/2 settings
            Http2Settings {
                header_table_size: 65536,
                enable_push: true,
                max_concurrent_streams: 100,
                initial_window_size: 131072,
                max_frame_size: 16384,
                max_header_list_size: 0,
            },

            // Safari HTTP/2 settings
            Http2Settings {
                header_table_size: 4096,
                enable_push: false,
                max_concurrent_streams: 100,
                initial_window_size: 2097152,
                max_frame_size: 16384,
                max_header_list_size: 0,
            },
        ]
    }

    fn load_tls_extensions() -> Vec<TlsExtension> {
        vec![
            // Server Name Indication
            TlsExtension {
                extension_type: 0x0000,
                data: vec![],
            },
            
            // Status Request
            TlsExtension {
                extension_type: 0x0005,
                data: vec![0x01, 0x00, 0x00, 0x00, 0x00],
            },
            
            // Supported Groups
            TlsExtension {
                extension_type: 0x000a,
                data: vec![0x00, 0x08, 0x00, 0x1d, 0x00, 0x17, 0x00, 0x18, 0x00, 0x19],
            },
            
            // Signature Algorithms
            TlsExtension {
                extension_type: 0x000d,
                data: vec![
                    0x00, 0x20, 0x04, 0x03, 0x05, 0x03, 0x06, 0x03, 0x08, 0x07,
                    0x08, 0x08, 0x08, 0x09, 0x08, 0x0a, 0x08, 0x0b, 0x08, 0x04,
                    0x08, 0x05, 0x08, 0x06, 0x04, 0x01, 0x05, 0x01, 0x06, 0x01
                ],
            },
        ]
    }

    // Generate JA3 fingerprint string
    pub fn generate_ja3_string(&self, profile: &JA3Profile) -> String {
        let version = "771"; // TLS 1.2
        
        let ciphers: String = profile.cipher_suites
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("-");
            
        let extensions: String = profile.extensions
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("-");
            
        let curves: String = profile.elliptic_curves
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("-");
            
        let formats: String = profile.ec_point_formats
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("-");

        format!("{},{},{},{},{}", version, ciphers, extensions, curves, formats)
    }

    // Get random JA3 profile
    pub fn get_random_ja3_profile(&self) -> &JA3Profile {
        let mut rng = rand::thread_rng();
        &self.ja3_profiles[rng.gen_range(0..self.ja3_profiles.len())]
    }

    // Generate HTTP/2 fingerprint
    pub fn generate_http2_fingerprint(&self, settings: &Http2Settings) -> HashMap<String, String> {
        let mut fingerprint = HashMap::new();
        
        fingerprint.insert("HEADER_TABLE_SIZE".to_string(), settings.header_table_size.to_string());
        fingerprint.insert("ENABLE_PUSH".to_string(), (settings.enable_push as u8).to_string());
        fingerprint.insert("MAX_CONCURRENT_STREAMS".to_string(), settings.max_concurrent_streams.to_string());
        fingerprint.insert("INITIAL_WINDOW_SIZE".to_string(), settings.initial_window_size.to_string());
        fingerprint.insert("MAX_FRAME_SIZE".to_string(), settings.max_frame_size.to_string());
        fingerprint.insert("MAX_HEADER_LIST_SIZE".to_string(), settings.max_header_list_size.to_string());
        
        fingerprint
    }

    // Randomize TLS extensions order
    pub fn randomize_extensions(&self, extensions: &[TlsExtension]) -> Vec<TlsExtension> {
        let mut randomized = extensions.to_vec();
        let mut rng = rand::thread_rng();
        
        // Fisher-Yates shuffle
        for i in (1..randomized.len()).rev() {
            let j = rng.gen_range(0..=i);
            randomized.swap(i, j);
        }
        
        randomized
    }

    // Generate custom cipher suite order
    pub fn randomize_cipher_suites(&self, base_ciphers: &[u16]) -> Vec<u16> {
        let mut randomized = base_ciphers.to_vec();
        let mut rng = rand::thread_rng();
        
        // Shuffle while keeping secure ciphers at the front
        let secure_count = 8; // Keep first 8 secure ciphers in order
        if randomized.len() > secure_count {
            let (_secure, rest) = randomized.split_at_mut(secure_count);
            
            // Shuffle the rest
            for i in (1..rest.len()).rev() {
                let j = rng.gen_range(0..=i);
                rest.swap(i, j);
            }
        }
        
        randomized
    }

    // Create browser-specific TCP window size
    pub fn get_tcp_window_size(&self, browser: &str) -> u16 {
        match browser.to_lowercase().as_str() {
            "chrome" => 65535,
            "firefox" => 32768,
            "safari" => 65535,
            "edge" => 65535,
            _ => {
                let mut rng = rand::thread_rng();
                rng.gen_range(32768..=65535)
            }
        }
    }

    // Generate realistic TLS hello packet timing
    pub fn get_hello_timing(&self, browser: &str) -> (u64, u64) {
        let mut rng = rand::thread_rng();
        
        match browser.to_lowercase().as_str() {
            "chrome" => (
                rng.gen_range(50..150),   // Client Hello delay (ms)
                rng.gen_range(100..300),  // Server Hello response (ms)
            ),
            "firefox" => (
                rng.gen_range(80..200),
                rng.gen_range(120..350),
            ),
            "safari" => (
                rng.gen_range(60..180),
                rng.gen_range(90..280),
            ),
            _ => (
                rng.gen_range(50..200),
                rng.gen_range(100..350),
            ),
        }
    }

    // Generate realistic packet sizes
    pub fn get_packet_sizes(&self, browser: &str) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        
        match browser.to_lowercase().as_str() {
            "chrome" => vec![
                rng.gen_range(512..1024),   // Client Hello
                rng.gen_range(1024..1500),  // Certificate
                rng.gen_range(64..128),     // Finished
            ],
            "firefox" => vec![
                rng.gen_range(400..800),
                rng.gen_range(800..1200),
                rng.gen_range(48..96),
            ],
            "safari" => vec![
                rng.gen_range(300..600),
                rng.gen_range(600..1000),
                rng.gen_range(32..80),
            ],
            _ => vec![
                rng.gen_range(300..1024),
                rng.gen_range(600..1500),
                rng.gen_range(32..128),
            ],
        }
    }

    // Anti-fingerprinting header order
    pub fn get_header_order(&self, browser: &str) -> Vec<String> {
        match browser.to_lowercase().as_str() {
            "chrome" => vec![
                "host".to_string(),
                "connection".to_string(),
                "cache-control".to_string(),
                "sec-ch-ua".to_string(),
                "sec-ch-ua-mobile".to_string(),
                "sec-ch-ua-platform".to_string(),
                "upgrade-insecure-requests".to_string(),
                "user-agent".to_string(),
                "accept".to_string(),
                "sec-fetch-site".to_string(),
                "sec-fetch-mode".to_string(),
                "sec-fetch-user".to_string(),
                "sec-fetch-dest".to_string(),
                "accept-encoding".to_string(),
                "accept-language".to_string(),
            ],
            "firefox" => vec![
                "host".to_string(),
                "user-agent".to_string(),
                "accept".to_string(),
                "accept-language".to_string(),
                "accept-encoding".to_string(),
                "dnt".to_string(),
                "connection".to_string(),
                "upgrade-insecure-requests".to_string(),
                "sec-fetch-dest".to_string(),
                "sec-fetch-mode".to_string(),
                "sec-fetch-site".to_string(),
            ],
            "safari" => vec![
                "host".to_string(),
                "connection".to_string(),
                "upgrade-insecure-requests".to_string(),
                "user-agent".to_string(),
                "accept".to_string(),
                "sec-fetch-site".to_string(),
                "sec-fetch-mode".to_string(),
                "sec-fetch-dest".to_string(),
                "accept-language".to_string(),
                "accept-encoding".to_string(),
            ],
            _ => {
                let mut default_order = vec![
                    "host".to_string(),
                    "connection".to_string(),
                    "user-agent".to_string(),
                    "accept".to_string(),
                    "accept-language".to_string(),
                    "accept-encoding".to_string(),
                ];
                
                // Randomize for unknown browsers
                let mut rng = rand::thread_rng();
                for i in (1..default_order.len()).rev() {
                    let j = rng.gen_range(0..=i);
                    default_order.swap(i, j);
                }
                
                default_order
            }
        }
    }

    // Check if fingerprint matches known bot patterns
    pub fn is_bot_fingerprint(&self, ja3: &str, user_agent: &str) -> bool {
        let bot_ja3_patterns = [
            "769,47-53-5-10-49171-49172",  // Common bot pattern
            "771,4865-4866-4867",          // Simplified TLS
            "0,0-0-0-0-0,0,0,0",          // Empty/default pattern
        ];
        
        let ua_lower = user_agent.to_lowercase();
        let is_bot_ua = ua_lower.contains("bot") || 
                       ua_lower.contains("crawler") || 
                       ua_lower.contains("spider") ||
                       ua_lower.contains("curl") ||
                       ua_lower.contains("wget");
        
        let is_bot_ja3 = bot_ja3_patterns.iter().any(|&pattern| ja3.contains(pattern));
        
        is_bot_ua || is_bot_ja3
    }
}
