use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use crate::deploy;

pub fn execute() {
    let home = env::var("HOME").unwrap_or_default();
    let dev_path_file = format!("{}/.smart_dev/source_path", home);
    let current_version = env!("CARGO_PKG_VERSION");
    let mut target_version = String::new();
    let mut is_dev = false;
    let mut source_dir = String::new();
    
    if Path::new(&dev_path_file).exists() {
        if let Ok(sd) = fs::read_to_string(&dev_path_file) {
            source_dir = sd.trim().to_string();
            let cargo_toml_path = format!("{}/Cargo.toml", source_dir);
            if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
                for line in content.lines() {
                    if line.trim().starts_with("version") && line.contains('=') {
                        let parts: Vec<&str> = line.split('=').collect();
                        if parts.len() == 2 { target_version = parts[1].trim().trim_matches(|c| c == '"' || c == ' ').to_string(); is_dev = true; break; }
                    }
                }
            }
        }
    }
    
    if !is_dev { target_version = "X.X.X".to_string(); }
    if current_version == target_version {
        println!("\x1b[32m[System] You are up to date! (v{})\x1b[0m", current_version);
        return;
    }
    
    if is_dev { print!("\x1b[33m[System] The latest dev build version is {}. Update? [y/N]: \x1b[0m", target_version); } 
    else { print!("\x1b[33m[System] The latest GitHub version is {}. Update? [y/N]: \x1b[0m", target_version); }
    
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() && input.trim().to_lowercase() == "y" {
        if is_dev {
            println!("\x1b[36m[System] Starting Local Compilation Update...\x1b[0m");
            deploy::deploy(&source_dir);
        } else {
            println!("\x1b[36m[System] Starting OTA Update from GitHub...\x1b[0m");
            println!("\x1b[33m[Warning] GitHub Repository URL is not configured yet. OTA update bypassed.\x1b[0m");
        }
    } else { println!("\x1b[31m[System] Update cancelled.\x1b[0m"); }
}
