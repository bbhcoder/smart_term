use crate::ipc::client::send_request;
use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use std::env;
use std::io::{self, Write};

pub fn handle(limit: u32, all: bool) {
    let cwd = if all { None } else { Some(env::current_dir().unwrap_or_default().to_string_lossy().into_owned()) };
    let req = ClientRequest::GetHistory { cwd, limit };
    match send_request(req) {
        Ok(DaemonResponse::History(cmds)) => {
            if cmds.is_empty() { println!("\x1b[33mNo history found.\x1b[0m"); }
            else { for (i, cmd) in cmds.iter().enumerate() { println!("{}\t{}", i + 1, cmd); } }
        }
        Ok(DaemonResponse::Error(e)) => println!("\x1b[31m[Error] {}\x1b[0m", e),
        Err(_) => println!("\x1b[31m[System] SmartD Daemon is not running!\x1b[0m"),
        _ => println!("\x1b[31m[Error] Unexpected response format\x1b[0m"),
    }
}

pub fn handle_nav(index: usize) {
    if index == 0 { return; }
    let cwd = Some(env::current_dir().unwrap_or_default().to_string_lossy().into_owned());
    let req = ClientRequest::GetHistory { cwd, limit: index as u32 };
    if let Ok(DaemonResponse::History(cmds)) = send_request(req) {
        if let Some(cmd) = cmds.last() { print!("{}", cmd); }
    }
}

pub fn handle_reset() {
    print!("\x1b[33mAre you sure you want to clear history for this specific path/context? [y/N]: \x1b[0m");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() && input.trim().to_lowercase() == "y" {
        let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
        match send_request(ClientRequest::ResetHistory { cwd }) {
            Ok(DaemonResponse::Success) => println!("\x1b[32m[System] Context history successfully cleared.\x1b[0m"),
            Ok(DaemonResponse::Error(e)) => println!("\x1b[31m[Error] {}\x1b[0m", e),
            _ => println!("\x1b[31m[Error] Failed to execute reset.\x1b[0m"),
        }
    } else { println!("\x1b[31m[System] Reset cancelled.\x1b[0m"); }
}

pub fn handle_reset_all() {
    print!("\x1b[31mWARNING: This will permanently wipe ALL history across all contexts. Proceed? [y/N]: \x1b[0m");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() && input.trim().to_lowercase() == "y" {
        match send_request(ClientRequest::ResetAllHistory) {
            Ok(DaemonResponse::Success) => println!("\x1b[32m[System] Entire database history completely wiped.\x1b[0m"),
            Ok(DaemonResponse::Error(e)) => println!("\x1b[31m[Error] {}\x1b[0m", e),
            _ => println!("\x1b[31m[Error] Failed to execute reset.\x1b[0m"),
        }
    } else { println!("\x1b[31m[System] Global reset cancelled.\x1b[0m"); }
}
