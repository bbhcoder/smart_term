mod pty; mod parser; mod history; mod auth; mod os;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "org" {
            parser::cmd_sys::set_default(false);
            println!("\x1b[32m[System] SmartTerm is now the default shell globally!\x1b[0m");
        } else if args[1] == "exit" {
            parser::cmd_sys::set_default(true);
            println!("\x1b[33m[System] SmartTerm removed from default global shell.\x1b[0m");
            return;
        }
    }

    let is_dev = std::env::var("CARGO_MANIFEST_DIR").is_ok();
    
    if std::env::var("SMART_TERM_ACTIVE").is_ok() {
        if is_dev {
            println!("\r\n\x1b[41;37m [ERROR] Nested Execution Detected! \x1b[0m");
            println!("\x1b[33m[System] You are already inside a SmartTerm session.\x1b[0m");
            println!("\x1b[33m[System] Type 'smartdev exit' first before running 'cargo run' again.\x1b[0m\r\n");
        }
        return;
    }

    if let Err(e) = history::db::init_db() {
        eprintln!("Database Init Error: {}", e);
        return;
    }
    
    if is_dev {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(mut local_bin) = dirs::home_dir() {
                local_bin.push(".local/bin");
                let _ = std::fs::create_dir_all(&local_bin);
                let _ = std::fs::copy(&exe, local_bin.join("smartdev"));
            }
        }
    }
    
    let _raw_mode = pty::io_handler::RawModeGuard::new().unwrap();
    if let Err(e) = pty::spawner::run_pty() {
        eprintln!("Error: {}", e);
    }
}