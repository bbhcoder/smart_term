use crate::parser::interceptor::CommandInterceptor;
use crate::history::queries;
use std::io::{self, Write};

pub fn set_default(remove: bool) {
    let inject_unix = "\n# SmartTerm\nif [ -z \"$SMART_TERM_ACTIVE\" ] && command -v smart >/dev/null 2>&1; then exec smart; fi\n";
    let inject_win = "\n# SmartTerm\nif (!$env:SMART_TERM_ACTIVE) { if (Get-Command smart -ErrorAction SilentlyContinue) { smart; exit } }\n";
    
    if cfg!(target_os = "windows") {
        if let Some(mut p) = dirs::document_dir() {
            p.push("WindowsPowerShell"); let _ = std::fs::create_dir_all(&p); p.push("Microsoft.PowerShell_profile.ps1");
            if let Ok(c) = std::fs::read_to_string(&p) {
                let mut lines: Vec<&str> = c.lines().filter(|l| !l.contains("SMART_TERM_ACTIVE") && !l.contains("# SmartTerm") && !l.contains("# Launch SmartTerm")).collect();
                if !remove { lines.push(inject_win); } let _ = std::fs::write(&p, lines.join("\n"));
            } else if !remove { let _ = std::fs::write(&p, inject_win); }
        }
    } else {
        for rc in [".bashrc", ".zshrc", ".bash_profile", ".zprofile"] {
            if let Some(mut p) = dirs::home_dir() {
                p.push(rc);
                if let Ok(c) = std::fs::read_to_string(&p) {
                    let mut lines: Vec<&str> = c.lines().filter(|l| !l.contains("SMART_TERM_ACTIVE") && !l.contains("# SmartTerm") && !l.contains("# Launch SmartTerm")).collect();
                    if !remove { lines.push(inject_unix); } let _ = std::fs::write(&p, lines.join("\n"));
                } else if !remove { let _ = std::fs::write(&p, inject_unix); }
            }
        }
    }
}

pub fn handle(ctx: &mut CommandInterceptor, cmd: &str) -> Option<Vec<u8>> {
    let mut res = None;
    if cmd == "help" || cmd == "smart help" || cmd == "smartdev help" {
        let h = "\r\n\x1b[1;36m  SmartTerm Commands\x1b[0m\r\n\x1b[33m[System]\x1b[0m\r\n  smart exit (Remove from default)\r\n  smart org (Set as default)\r\n  smartdev exit (Quit dev mode)\r\n  smartdev to smart / smart rollback\r\n";
        let _ = io::stdout().write_all(h.as_bytes()); res = Some(vec![21, b'\n']);
    } else if cmd == "smartdev exit" {
        let _ = io::stdout().write_all(b"\r\n\x1b[33m[System] You have exited the Smart Dev environment.\x1b[0m\r\n");
        return Some(b"\x15exit\n".to_vec());
    } else if cmd == "smart exit" {
        set_default(true); let _ = io::stdout().write_all(b"\r\n\x1b[33m[System] You exited from SmartTerm. To return type `smart`. To make it default type `smart org`.\x1b[0m\r\n"); 
        return Some(b"\x15exit\n".to_vec());
    } else if cmd == "smart" {
        if std::env::var("CARGO_MANIFEST_DIR").is_ok() { let _ = io::stdout().write_all(b"\r\n\x1b[33m[System] Smart Dev is active in this environment.\x1b[0m\r\n"); res = Some(vec![21, b'\n']); }
    } else if cmd == "smartdev to smart" {
        let exe = std::env::current_exe().unwrap_or_default().to_string_lossy().to_string();
        let _ = io::stdout().write_all(format!("\r\n\x1b[36m[System] Equivalent to: sudo cp {} /usr/local/bin/smart\x1b[0m\r\n", exe).as_bytes());
        let status = std::process::Command::new("sh").arg("-c").arg(&format!("sudo rm -f /usr/local/bin/smart.bak; sudo mv /usr/local/bin/smart /usr/local/bin/smart.bak 2>/dev/null || true; sudo cp {} /usr/local/bin/smart && sudo chmod +x /usr/local/bin/smart", exe)).status();
        if status.is_ok() && status.unwrap().success() { set_default(false); let _ = io::stdout().write_all(b"\x1b[32m[System] Dev deployed and set as default globally!\x1b[0m\r\n"); }
        else { let _ = io::stdout().write_all(b"\x1b[31m[System] Deployment failed or cancelled.\x1b[0m\r\n"); }
        res = Some(vec![21, b'\n']);
    } else if cmd == "smart rollback" {
        let _ = io::stdout().write_all(b"\r\n\x1b[36m[System] Equivalent to: sudo mv /usr/local/bin/smart.bak /usr/local/bin/smart\x1b[0m\r\n");
        let status = std::process::Command::new("sh").arg("-c").arg("if [ -f /usr/local/bin/smart.bak ]; then sudo rm -f /usr/local/bin/smart; sudo mv /usr/local/bin/smart.bak /usr/local/bin/smart; exit 0; else exit 1; fi").status();
        if status.is_ok() && status.unwrap().success() { let _ = io::stdout().write_all(b"\x1b[32m[System] Rollback successful!\x1b[0m\r\n"); }
        else { let _ = io::stdout().write_all(b"\x1b[31m[System] No backup found or rollback failed!\x1b[0m\r\n"); }
        res = Some(vec![21, b'\n']);
    } else if cmd == "smart org" || cmd == "smartdev org" {
        set_default(false); let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] SmartTerm is now the default shell again!\x1b[0m\r\n"); res = Some(vec![21, b'\n']);
    } else if cmd == "cexit" {
        ctx.history.clear(); res = Some(vec![21]);
    } else if cmd.starts_with("restore ") {
        ctx.history = queries::get_tool_hist(cmd[8..].trim()).unwrap_or_default(); res = Some(vec![21]);
    } else if cmd.ends_with(" ccopy") {
        let cln = cmd.strip_suffix(" ccopy").unwrap().trim(); let log = dirs::home_dir().unwrap_or_default().join(".scopy.log").to_string_lossy().to_string(); let mut out = vec![21]; out.extend_from_slice(format!("{} | tee {}; if command -v pbcopy >/dev/null 2>&1; then pbcopy < {}; elif command -v wl-copy >/dev/null 2>&1; then wl-copy < {}; elif command -v xclip >/dev/null 2>&1; then xclip -selection clipboard < {}; fi; printf '\\r\\n\\x1b[32m[System] Copied!\\x1b[0m\\r\\n'\n", cln, log, log, log, log).as_bytes()); res = Some(out);
    } else if cmd.starts_with("namespace") {
        if cmd.starts_with("namespace use ") { ctx.active_namespace = cmd.split_whitespace().nth(2).map(String::from); let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] NS: {}\x1b[0m\r\n", ctx.active_namespace.as_deref().unwrap_or("")).as_bytes()); } else if cmd == "namespace exit" { ctx.active_namespace = None; let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[32m[System] NS Exited\x1b[0m\r\n"); } res = Some(vec![21, b'\n']);
    } else if cmd.starts_with("login project ") || cmd.starts_with("save project ") {
        ctx.active_project = cmd.split_whitespace().nth(2).map(String::from); crate::parser::auth_state::check_bindings(ctx); res = Some(vec![21, b'\n']);
    } else if cmd.starts_with("exit project") {
        ctx.active_project = None; crate::parser::auth_state::check_bindings(ctx); res = Some(vec![21, b'\n']);
    }
    res
}