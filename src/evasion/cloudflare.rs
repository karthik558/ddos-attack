use anyhow::Result;
use reqwest::{Client, header::HeaderMap};
use std::collections::HashMap;
use std::time::Duration;
use rand::Rng;

pub struct CloudflareBypass {
    client: Client,
    ja3_fingerprints: Vec<String>,
    challenge_cache: HashMap<String, String>,
}

impl CloudflareBypass {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            ja3_fingerprints: Self::load_ja3_fingerprints(),
            challenge_cache: HashMap::new(),
        })
    }

    // Load common JA3 fingerprints to mimic real browsers
    fn load_ja3_fingerprints() -> Vec<String> {
        vec![
            // Chrome
            "769,47-53-5-10-49171-49172-49161-49162-49".to_string(),
            // Firefox
            "771,4865-4866-4867-49195-49199-49196-49200".to_string(),
            // Safari
            "772,4865-4866-4867-49195-49199-52393-52392".to_string(),
            // Edge
            "773,4865-4866-4867-49195-49199-49196-49200".to_string(),
        ]
    }

    // Bypass Cloudflare's JavaScript challenge
    pub async fn bypass_js_challenge(&mut self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;

        // Check if it's a Cloudflare challenge page
        if html.contains("Checking your browser before accessing") {
            log::info!("Detected Cloudflare JS challenge, attempting bypass...");
            
            // Extract challenge parameters
            if let Some(challenge_params) = self.extract_challenge_params(&html) {
                return self.solve_js_challenge(url, challenge_params).await;
            }
        }

        Ok(html)
    }

    // Extract challenge parameters from Cloudflare page
    fn extract_challenge_params(&self, html: &str) -> Option<HashMap<String, String>> {
        let mut params = HashMap::new();
        
        // Extract common Cloudflare challenge parameters
        if let Some(r_value) = self.extract_between(html, "name=\"r\" value=\"", "\"") {
            params.insert("r".to_string(), r_value);
        }
        
        if let Some(jschl_vc) = self.extract_between(html, "name=\"jschl_vc\" value=\"", "\"") {
            params.insert("jschl_vc".to_string(), jschl_vc);
        }
        
        if let Some(pass) = self.extract_between(html, "name=\"pass\" value=\"", "\"") {
            params.insert("pass".to_string(), pass);
        }

        if params.is_empty() { None } else { Some(params) }
    }

    // Solve JavaScript challenge
    async fn solve_js_challenge(&mut self, url: &str, params: HashMap<String, String>) -> Result<String> {
        // Wait for challenge timeout (usually 5 seconds)
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        // Create challenge solution
        let mut form_data = params.clone();
        
        // Add computed answer (simplified - real implementation would parse JS)
        if let Some(domain) = url::Url::parse(url)?.host_str() {
            let answer = self.compute_challenge_answer(domain, &params);
            form_data.insert("jschl_answer".to_string(), answer.to_string());
        }

        // Submit challenge solution
        let challenge_url = format!("{}/cdn-cgi/l/chk_jschl", url);
        let response = self.client
            .post(&challenge_url)
            .form(&form_data)
            .send()
            .await?;

        Ok(response.text().await?)
    }

    // Compute challenge answer (simplified version)
    fn compute_challenge_answer(&self, domain: &str, params: &HashMap<String, String>) -> i32 {
        let mut answer = domain.len() as i32;
        
        // Add some randomization based on parameters
        if let Some(r_val) = params.get("r") {
            answer += r_val.len() as i32;
        }
        
        answer
    }

    // Generate realistic HTTP headers to bypass detection
    pub fn generate_stealth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let mut rng = rand::thread_rng();

        // Randomize User-Agent
        let user_agents = [
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
        ];
        
        headers.insert(
            "User-Agent",
            user_agents[rng.gen_range(0..user_agents.len())].parse().unwrap()
        );

        // Add realistic headers
        headers.insert("Accept", "*/*".parse().unwrap());
        headers.insert("Accept-Language", "en-US,en;q=0.9".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
        headers.insert("DNT", "1".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());

        // Randomize some headers
        if rng.gen_bool(0.7) {
            headers.insert("Cache-Control", "max-age=0".parse().unwrap());
        }
        
        if rng.gen_bool(0.5) {
            let referers = ["https://www.google.com/", "https://www.bing.com/", "https://duckduckgo.com/"];
            headers.insert("Referer", referers[rng.gen_range(0..referers.len())].parse().unwrap());
        }

        headers
    }

    // TLS fingerprint randomization
    pub async fn create_stealth_client(&self) -> Result<Client> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::limited(10))
            .danger_accept_invalid_certs(false)
            .build()?;

        Ok(client)
    }

    // Bypass rate limiting with smart delays
    pub async fn smart_delay(&self) {
        let mut rng = rand::thread_rng();
        let delay_ms = rng.gen_range(100..2000); // Random delay between 100ms-2s
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }

    // Helper function to extract text between delimiters
    fn extract_between(&self, text: &str, start: &str, end: &str) -> Option<String> {
        let start_pos = text.find(start)? + start.len();
        let end_pos = text[start_pos..].find(end)? + start_pos;
        Some(text[start_pos..end_pos].to_string())
    }

    // Check if response indicates Cloudflare protection
    pub fn is_cloudflare_protected(&self, response_text: &str) -> bool {
        response_text.contains("Checking your browser before accessing") ||
        response_text.contains("DDoS protection by Cloudflare") ||
        response_text.contains("__cf_bm") ||
        response_text.contains("cf-ray")
    }

    // Generate realistic cookies to bypass detection
    pub fn generate_stealth_cookies(&self) -> HashMap<String, String> {
        let mut cookies = HashMap::new();
        let mut rng = rand::thread_rng();

        // Generate realistic session cookies
        let session_id: String = (0..32)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        
        cookies.insert("sessionid".to_string(), session_id);
        
        // Add some common tracking cookies
        if rng.gen_bool(0.8) {
            let ga_id: String = format!("GA1.2.{}.{}", 
                rng.gen_range(100000000..999999999),
                rng.gen_range(1000000000..9999999999i64)
            );
            cookies.insert("_ga".to_string(), ga_id);
        }

        cookies
    }
}
