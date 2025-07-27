use rand::Rng;
use std::collections::HashMap;

pub struct UserAgentDatabase {
    browsers: HashMap<BrowserType, Vec<String>>,
    mobile_agents: Vec<String>,
    bot_agents: Vec<String>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum BrowserType {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Opera,
}

impl UserAgentDatabase {
    pub fn new() -> Self {
        Self {
            browsers: Self::load_browser_agents(),
            mobile_agents: Self::load_mobile_agents(),
            bot_agents: Self::load_bot_agents(),
        }
    }

    fn load_browser_agents() -> HashMap<BrowserType, Vec<String>> {
        let mut browsers = HashMap::new();

        // Chrome User-Agents
        browsers.insert(BrowserType::Chrome, vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36".to_string(),
        ]);

        // Firefox User-Agents
        browsers.insert(BrowserType::Firefox, vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0".to_string(),
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:119.0) Gecko/20100101 Firefox/119.0".to_string(),
        ]);

        // Safari User-Agents
        browsers.insert(BrowserType::Safari, vec![
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Safari/605.1.15".to_string(),
            "Mozilla/5.0 (iPad; CPU OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
        ]);

        // Edge User-Agents
        browsers.insert(BrowserType::Edge, vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 Edg/119.0.0.0".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.0.0".to_string(),
        ]);

        // Opera User-Agents
        browsers.insert(BrowserType::Opera, vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 OPR/105.0.0.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36 OPR/104.0.0.0".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 OPR/103.0.0.0".to_string(),
        ]);

        browsers
    }

    fn load_mobile_agents() -> Vec<String> {
        vec![
            // iPhone
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
            "Mozilla/5.0 (iPhone; CPU iPhone OS 16_7_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1".to_string(),
            
            // Android Chrome
            "Mozilla/5.0 (Linux; Android 14; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Mobile Safari/537.36".to_string(),
            
            // Android Firefox
            "Mozilla/5.0 (Mobile; rv:121.0) Gecko/121.0 Firefox/121.0".to_string(),
            
            // iPad
            "Mozilla/5.0 (iPad; CPU OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
            
            // Samsung Internet
            "Mozilla/5.0 (Linux; Android 14; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/23.0 Chrome/115.0.0.0 Mobile Safari/537.36".to_string(),
        ]
    }

    fn load_bot_agents() -> Vec<String> {
        vec![
            // Search engine bots
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)".to_string(),
            "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)".to_string(),
            "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)".to_string(),
            "Mozilla/5.0 (compatible; DuckDuckBot-Https/1.1; https://duckduckgo.com/duckduckbot)".to_string(),
            
            // Social media bots
            "facebookexternalhit/1.1 (+http://www.facebook.com/externalhit_uatext.php)".to_string(),
            "Twitterbot/1.0".to_string(),
            "LinkedInBot/1.0 (compatible; Mozilla/5.0; Apache-HttpClient +http://www.linkedin.com)".to_string(),
            
            // SEO/Monitoring bots
            "Mozilla/5.0 (compatible; AhrefsBot/7.0; +http://ahrefs.com/robot/)".to_string(),
            "Mozilla/5.0 (compatible; SemrushBot/7~bl; +http://www.semrush.com/bot.html)".to_string(),
        ]
    }

    // Get random user agent from all categories
    pub fn get_random(&self) -> String {
        let mut rng = rand::thread_rng();
        let category = rng.gen_range(0..4);
        
        match category {
            0 => self.get_random_browser(),
            1 => self.get_random_mobile(),
            2 => self.get_random_bot(),
            _ => self.get_random_browser(),
        }
    }

    // Get random browser user agent
    pub fn get_random_browser(&self) -> String {
        let mut rng = rand::thread_rng();
        let browser_types = vec![
            BrowserType::Chrome,
            BrowserType::Firefox,
            BrowserType::Safari,
            BrowserType::Edge,
            BrowserType::Opera,
        ];
        
        let browser_type = &browser_types[rng.gen_range(0..browser_types.len())];
        let agents = self.browsers.get(browser_type).unwrap();
        agents[rng.gen_range(0..agents.len())].clone()
    }

    // Get random mobile user agent
    pub fn get_random_mobile(&self) -> String {
        let mut rng = rand::thread_rng();
        self.mobile_agents[rng.gen_range(0..self.mobile_agents.len())].clone()
    }

    // Get random bot user agent
    pub fn get_random_bot(&self) -> String {
        let mut rng = rand::thread_rng();
        self.bot_agents[rng.gen_range(0..self.bot_agents.len())].clone()
    }

    // Get specific browser user agent
    pub fn get_browser(&self, browser: BrowserType) -> String {
        let mut rng = rand::thread_rng();
        let agents = self.browsers.get(&browser).unwrap();
        agents[rng.gen_range(0..agents.len())].clone()
    }

    // Generate custom user agent with version randomization
    pub fn generate_custom_chrome(&self, os: &str) -> String {
        let mut rng = rand::thread_rng();
        let chrome_version = rng.gen_range(110..121);
        let webkit_version = 537.36;
        
        match os {
            "windows" => {
                let win_version = if rng.gen_bool(0.7) { "10.0" } else { "11.0" };
                format!(
                    "Mozilla/5.0 (Windows NT {}; Win64; x64) AppleWebKit/{} (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    win_version, webkit_version, chrome_version
                )
            },
            "mac" => {
                let mac_version = match rng.gen_range(0..3) {
                    0 => "10_15_7",
                    1 => "10_14_6", 
                    _ => "10_13_6",
                };
                format!(
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X {}) AppleWebKit/{} (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    mac_version, webkit_version, chrome_version
                )
            },
            "linux" => {
                format!(
                    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/{} (KHTML, like Gecko) Chrome/{}.0.0.0 Safari/537.36",
                    webkit_version, chrome_version
                )
            },
            _ => self.get_browser(BrowserType::Chrome),
        }
    }

    // Check if user agent looks suspicious
    pub fn is_suspicious(&self, user_agent: &str) -> bool {
        let suspicious_patterns = [
            "curl",
            "wget",
            "python",
            "requests",
            "http",
            "bot",
            "crawler",
            "spider",
            "scraper",
            "test",
            "script",
        ];
        
        let ua_lower = user_agent.to_lowercase();
        suspicious_patterns.iter().any(|&pattern| ua_lower.contains(pattern))
    }

    // Rotate through different user agents
    pub fn get_rotation_set(&self, count: usize) -> Vec<String> {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Mix of different types
        for _ in 0..count {
            match rng.gen_range(0..4) {
                0 => agents.push(self.get_random_browser()),
                1 => agents.push(self.get_random_mobile()),
                2 => agents.push(self.get_random_bot()),
                _ => agents.push(self.get_random_browser()),
            }
        }
        
        agents
    }

    // Get user agents by platform
    pub fn get_by_platform(&self, platform: Platform) -> Vec<String> {
        match platform {
            Platform::Windows => {
                self.browsers.values()
                    .flatten()
                    .filter(|ua| ua.contains("Windows"))
                    .cloned()
                    .collect()
            },
            Platform::Mac => {
                self.browsers.values()
                    .flatten()
                    .filter(|ua| ua.contains("Macintosh") || ua.contains("Mac OS"))
                    .cloned()
                    .collect()
            },
            Platform::Linux => {
                self.browsers.values()
                    .flatten()
                    .filter(|ua| ua.contains("Linux") || ua.contains("X11"))
                    .cloned()
                    .collect()
            },
            Platform::Mobile => self.mobile_agents.clone(),
        }
    }
}

pub enum Platform {
    Windows,
    Mac,
    Linux,
    Mobile,
}
