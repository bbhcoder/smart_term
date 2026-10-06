use std::{env, process::Command};

pub fn deploy(_cwd: &str) {
    println!("\x1b[33m[System] Compiling V2 Workspace to Stable...\x1b[0m");
    let ws = env::current_dir().unwrap();
    let status = Command::new("cargo").current_dir(&ws).args(["build", "--release", "--workspace"]).status();
    
    if status.is_err() || !status.unwrap().success() {
        println!("\x1b[31m[Error] Compilation failed.\x1b[0m"); return;
    }
    
    let c_bin = ws.join("target/release/cli");
    let d_bin = ws.join("target/release/smartd");
    
    println!("\x1b[33m[System] Requesting sudo for /usr/local/bin installation...\x1b[0m");
    let script = format!(
        "rm -f /usr/local/bin/smart.bak; mv /usr/local/bin/smart /usr/local/bin/smart.bak 2>/dev/null || true; \
         cp {} /usr/local/bin/smart && chmod +x /usr/local/bin/smart; \
         cp {} /usr/local/bin/smartd && chmod +x /usr/local/bin/smartd",
        c_bin.display(), d_bin.display()
    );
    
    let cp_status = Command::new("sudo").arg("sh").arg("-c").arg(&script).status();
    if cp_status.is_ok() && cp_status.unwrap().success() {
        println!("\x1b[32m[System] V2 Deployed successfully!\x1b[0m");
    } else {
        println!("\x1b[31m[Error] Copy failed.\x1b[0m");
    }
}

pub fn enable_org() { println!("\x1b[33m[System] Org Mode is managed natively via Hooks in V2.\x1b[0m"); }
pub fn disable_org() { println!("\x1b[33m[System] Org Mode is managed natively via Hooks in V2.\x1b[0m"); }
pub fn rollback() { println!("\x1b[31m[System] Rollback not implemented for V2 yet.\x1b[0m"); }
