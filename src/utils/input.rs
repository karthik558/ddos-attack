use std::io::{self, Write};
use colored::*;

pub fn ask_for_tor() -> bool {
    print!("{}", "\n🔒 Do you want to use Tor for anonymity? (y/N): ".bright_yellow());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let input = input.trim().to_lowercase();
    matches!(input.as_str(), "y" | "yes")
}

pub fn ask_for_spoofing() -> bool {
    print!("{}", "🎭 Do you want to enable IP spoofing? (requires root) (y/N): ".bright_yellow());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let input = input.trim().to_lowercase();
    matches!(input.as_str(), "y" | "yes")
}
