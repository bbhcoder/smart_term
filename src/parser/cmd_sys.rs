use crate::parser::interceptor::CommandInterceptor;
use crate::history::queries;
use std::{env, io::{self, Write}};

pub fn set_default(remove: bool) {
    let inject_unix = "\n# SmartTerm\nif [ -z \"$SMART_TERM_ACTIVE\" ] && command -v smart >/dev/null 2>&1; then smart; fi\n";
    let inject_win = "\n# SmartTerm\nif (!$env:SMART_TERM_ACTIVE) { if (Get-Command smart -ErrorAction SilentlyContinue) { smart } }\n";
    if cfg!(target_os = "windows") {
        if let Some(mut p) = dirs::document_dir() {
            p.push("WindowsPowerShell"); let _ = std::fs::create_dir_all(&p); p.push("Microsoft.PowerShell_profile.ps1");
            if let Ok(c) = std::fs::read_to_string(&p) {
                let mut lines: Vec<&str> = c.lines().filter(|l| !l.contains("SMART_TERM_ACTIVE") && !l.contains("# SmartTerm")).collect();
                if !remove { lines.push(inject_win); } let _ = std::fs::write(&p, lines.join("\n"));
            } else if !remove { let _ = std::fs::write(&p, inject_win); }
        }
    } else {
        for rc in [".bashrc", ".zshrc", ".bash_profile", ".zprofile"] {
            if let Some(mut p) = dirs::home_dir() {
                p.push(rc);
                if let Ok(c) = std::fs::read_to_string(&p) {
                    let mut lines: Vec<&str> = c.lines().filter(|l| !l.contains("SMART_TERM_ACTIVE") && !l.contains("# SmartTerm")).collect();
                    if !remove { lines.push(inject_unix); } let _ = std::fs::write(&p, lines.join("\n"));
                } else if !remove { let _ = std::fs::write(&p, inject_unix); }
            }
        }
    }
}

pub fn handle(ctx: &mut CommandInterceptor, cmd: &str) -> Option<Vec<u8>> {
    let mut res = None;
    let is_dev = env::var("CARGO_MANIFEST_DIR").is_ok();
    
    if cmd == "help" || cmd == "smart help" || cmd == "smartdev help" {
        let h = "\r\n\x1b[1;36m  SmartTerm Commands\x1b[0m\r\n\x1b[33m[History]\x1b[0m\r\n  history clear\r\n  rmc\r\n  <cmd> rmc\r\n\x1b[33m[System]\x1b[0m\r\n  smart exit\r\n  smart org\r\n  smartdev deploy\r\n  smart rollback\r\n  smartdev reset\r\n";
        let _ = io::stdout().write_all(h.as_bytes()); res = Some(vec![3]);
    } else if cmd == "history clear" {
        let _ = queries::clear_all_history(); ctx.history.clear(); ctx.history_idx = 0;
        let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] All history cleared!\x1b[0m\r\n"); res = Some(vec![3]);
    } else if cmd == "smartdev reset" {
        let script = "sudo rm -f /usr/local/bin/smart.bak && printf '\\r\\n\\x1b[32m[System] Rollback backups cleared!\\x1b[0m\\r\\n'\n";
        let mut out = vec![21]; out.extend_from_slice(script.as_bytes()); ctx.os_wait_pass = true; res = Some(out);
    } else if cmd == "rmc" {
        let _ = queries::remove_last_cmd(); ctx.history.clear(); ctx.history_idx = 0;
        let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Last command removed from history.\x1b[0m\r\n"); res = Some(vec![3]);
    } else if cmd.ends_with(" rmc") {
        let cln = crate::parser::flags::clean_command(cmd.strip_suffix(" rmc").unwrap().trim());
        let _ = queries::remove_cmd(&cln); ctx.history.clear(); ctx.history_idx = 0;
        let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] '{}' removed.\x1b[0m\r\n", cln).as_bytes()); res = Some(vec![3]);
    } else if cmd == "smartdev exit" {
        if is_dev { let _ = io::stdout().write_all(b"\r\n\x1b[33m[System] Exited Smart Dev.\x1b[0m\r\n"); return Some(b"\x15exit\n".to_vec()); }
    } else if cmd == "smart exit" {
        set_default(true); let _ = io::stdout().write_all(b"\r\n\x1b[33m[System] Removed from default. Exiting...\x1b[0m\r\n"); return Some(b"\x15exit\n".to_vec());
    } else if cmd == "smart" {
        if is_dev { let _ = io::stdout().write_all(b"\r\n\x1b[33m[System] Smart Dev active.\x1b[0m\r\n"); res = Some(vec![3]); }
    } else if cmd == "smartdev deploy" || cmd == "smartdev to smart" {
        if is_dev {
            let manifest = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
            let clean_exe = format!("{}/target/debug/smart_term", manifest);
            let script = format!("sudo sh -c 'rm -f /usr/local/bin/smart.bak; mv /usr/local/bin/smart /usr/local/bin/smart.bak 2>/dev/null || true; cp \"{}\" /usr/local/bin/smart && chmod +x /usr/local/bin/smart' && smartdev org\n", clean_exe);
            let mut out = vec![21]; out.extend_from_slice(script.as_bytes()); ctx.os_wait_pass = true; res = Some(out);
        }
    } else if cmd == "smart rollback" {
        let script = "sudo sh -c 'if [ -f /usr/local/bin/smart.bak ]; then rm -f /usr/local/bin/smart; mv /usr/local/bin/smart.bak /usr/local/bin/smart; exit 0; else exit 1; fi' && printf '\\r\\n\\x1b[32m[System] Rollback successful!\\x1b[0m\\r\\n' || printf '\\r\\n\\x1b[31m[System] No backup found!\\x1b[0m\\r\\n'\n";
        let mut out = vec![21]; out.extend_from_slice(script.as_bytes()); ctx.os_wait_pass = true; res = Some(out);
    } else if cmd == "smart org" || cmd == "smartdev org" {
        if !(cmd == "smartdev org" && !is_dev) { set_default(false); let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Set as default!\x1b[0m\r\n"); res = Some(vec![3]); }
    } else if cmd == "cexit" {
        ctx.history.clear(); res = Some(vec![3]);
    } else if cmd.starts_with("restore ") {
        ctx.history = queries::get_tool_hist(cmd[8..].trim()).unwrap_or_default(); res = Some(vec![3]);
    } else if cmd.ends_with(" ccopy") {
        let cln = cmd.strip_suffix(" ccopy").unwrap().trim(); let log = dirs::home_dir().unwrap_or_default().join(".scopy.log").to_string_lossy().to_string(); let mut out = vec![21]; out.extend_from_slice(format!("{} | tee {}; if command -v pbcopy >/dev/null 2>&1; then pbcopy < {}; elif command -v wl-copy >/dev/null 2>&1; then wl-copy < {}; elif command -v xclip >/dev/null 2>&1; then xclip -selection clipboard < {}; fi; printf '\\r\\n\\x1b[32m[System] Copied!\\x1b[0m\\r\\n'\n", cln, log, log, log, log).as_bytes()); res = Some(out);
    } else if cmd.ends_with("-none") {
        let cln = crate::parser::flags::clean_command(cmd);
        let mut out = vec![21];
        out.extend_from_slice(cln.as_bytes());
        out.push(b'\n');
        res = Some(out);    
    } else if cmd.starts_with("namespace") {
        if cmd.starts_with("namespace use ") { ctx.active_namespace = cmd.split_whitespace().nth(2).map(String::from); let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] NS: {}\x1b[0m\r\n", ctx.active_namespace.as_deref().unwrap_or("")).as_bytes()); } else if cmd == "namespace exit" { ctx.active_namespace = None; let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[32m[System] NS Exited\x1b[0m\r\n"); } res = Some(vec![3]);
    } else if cmd.starts_with("login project ") || cmd.starts_with("save project ") {
        ctx.active_project = cmd.split_whitespace().nth(2).map(String::from); crate::parser::auth_state::check_bindings(ctx); res = Some(vec![3]);
    } else if cmd.starts_with("exit project") {
        ctx.active_project = None; crate::parser::auth_state::check_bindings(ctx); res = Some(vec![3]);
    }
    res
}