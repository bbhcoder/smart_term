use crate::{parser::flags, history::queries, auth::session::UserSession};
use std::{env, path::Path, io::{self, Write}};

pub struct CommandInterceptor {
    buffer: String, current_dir: String, nav_history: String, in_escape: bool,
    history: Vec<String>, history_idx: usize, active_project: Option<String>,
    active_namespace: Option<String>, session: UserSession,
    pwd_mode: bool, pwd_buf: String, pending_user: Option<String>,
}

impl CommandInterceptor {
    pub fn new() -> Self {
        let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
        Self { buffer: String::new(), current_dir: cwd.clone(), nav_history: cwd, in_escape: false, history: vec![], history_idx: 0, active_project: None, active_namespace: None, session: UserSession::new(), pwd_mode: false, pwd_buf: String::new(), pending_user: None }
    }

    pub fn process_bytes(&mut self, bytes: &[u8]) -> Vec<u8> {
        if self.pwd_mode {
            for &b in bytes {
                match b {
                    b'\r' | b'\n' => {
                        if let Some(u) = &self.pending_user {
                            if queries::verify_user(u, &self.pwd_buf) {
                                self.session.login(u);
                                let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Logged in successfully\x1b[0m\r\n");
                            } else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Error: Incorrect password\x1b[0m\r\n"); }
                        }
                        let _ = io::stdout().flush();
                        self.pwd_mode = false; self.pwd_buf.clear(); self.pending_user = None; self.buffer.clear();
                        return vec![21, b'\n'];
                    }
                    127 | 8 => { if self.pwd_buf.pop().is_some() { let _ = io::stdout().write_all(b"\x08 \x08"); let _ = io::stdout().flush(); } }
                    c if c >= 32 && c <= 126 => { self.pwd_buf.push(c as char); let _ = io::stdout().write_all(b"*"); let _ = io::stdout().flush(); }
                    _ => {}
                }
            }
            return vec![];
        }

        if bytes == [27, 91, 65] {
            if self.history.is_empty() { self.history = queries::get_hist(&self.current_dir, self.active_project.as_deref(), self.active_namespace.as_deref(), self.session.current_user.as_deref()).unwrap_or_default(); self.history_idx = 0; }
            if self.history_idx < self.history.len() { self.buffer = self.history[self.history_idx].clone(); self.history_idx += 1; let mut out = vec![21]; out.extend_from_slice(self.buffer.as_bytes()); return out; }
            return vec![];
        }
        if bytes == [27, 91, 66] {
            if self.history_idx > 0 { self.history_idx -= 1; let mut out = vec![21]; if self.history_idx > 0 { self.buffer = self.history[self.history_idx - 1].clone(); out.extend_from_slice(self.buffer.as_bytes()); } else { self.buffer.clear(); } return out; }
            return vec![];
        }
        self.history.clear(); self.history_idx = 0;

        for &b in bytes {
            if self.in_escape { if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || b == b'~' { self.in_escape = false; } continue; }
            if b == 27 { self.in_escape = true; continue; }
            match b {
                b'\r' | b'\n' => {
                    let cmd = self.buffer.trim().to_string();
                    if let Some(fb) = self.handle_special(&cmd) { return fb; }
                    if !cmd.is_empty() && flags::should_log(&cmd) {
                        let cln = flags::clean_command(&cmd); let ed = self.current_dir.clone(); self.update_cwd(&cln);
                        let _ = queries::insert_cmd(&cln, &ed, self.active_project.as_deref(), self.active_namespace.as_deref(), self.session.current_user.as_deref());
                    }
                    self.buffer.clear();
                }
                127 | 8 => { self.buffer.pop(); }
                b if b >= 32 && b <= 126 => { self.buffer.push(b as char); }
                _ => {}
            }
        }
        bytes.to_vec()
    }

    fn handle_special(&mut self, cmd: &str) -> Option<Vec<u8>> {
        let mut res = None;
        if cmd == "cexit" { self.history.clear(); res = Some(vec![21]); }
        else if cmd.starts_with("restore ") { self.history = queries::get_tool_hist(cmd[8..].trim()).unwrap_or_default(); res = Some(vec![21]); }
        else if cmd.starts_with("login project ") || cmd.starts_with("save project ") { self.active_project = cmd.split_whitespace().nth(2).map(String::from); res = Some(vec![21, b'\n']); }
        else if cmd.starts_with("exit project") { self.active_project = None; res = Some(vec![21, b'\n']); }
        else if cmd.starts_with("namespace use ") { self.active_namespace = cmd.split_whitespace().nth(2).map(String::from); res = Some(vec![21, b'\n']); }
        else if cmd == "namespace exit" { self.active_namespace = None; res = Some(vec![21, b'\n']); }
        else if cmd.starts_with("user add ") {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.len() >= 4 { let _ = queries::add_user(parts[2], parts[3]); let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] User added successfully!\x1b[0m\r\n"); }
            else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Usage: user add <name> <password>\x1b[0m\r\n"); }
            let _ = io::stdout().flush(); res = Some(vec![21, b'\n']);
        }
        else if cmd.starts_with("login user ") {
            if let Some(u) = cmd.split_whitespace().nth(2) {
                if queries::user_exists(u) {
                    self.pending_user = Some(u.to_string()); self.pwd_mode = true;
                    let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] Password for {}: \x1b[0m", u).as_bytes());
                } else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Error: User not found in DB!\x1b[0m\r\n"); }
                let _ = io::stdout().flush();
            }
            res = Some(vec![21]);
        }
        else if cmd == "user logout" { self.session.logout(); res = Some(vec![21, b'\n']); }
        else if cmd == "user info" { let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] Current User: {}\x1b[0m\r\n", self.session.active_user()).as_bytes()); let _ = io::stdout().flush(); res = Some(vec![21, b'\n']); }
        else if cmd == "user list" {
            let mut m = String::from("\r\n\x1b[32m[System] Registered Users:\x1b[0m\r\n");
            if let Ok(us) = queries::get_users() { for (u, d) in us { m.push_str(&format!("  - {}{}{}\r\n", u, if d {" (Default)"} else {""}, if Some(&u)==self.session.current_user.as_ref() {" [*Active]"} else {""})); } }
            let _ = io::stdout().write_all(m.as_bytes()); let _ = io::stdout().flush(); res = Some(vec![21, b'\n']);
        }
        else if cmd.starts_with("user set default ") {
            if let Some(u) = cmd.split_whitespace().nth(3) {
                if queries::user_exists(u) { let _ = queries::set_def_user(u); let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] Default user: {}\x1b[0m\r\n", u).as_bytes()); }
                else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Error: User not found!\x1b[0m\r\n"); }
                let _ = io::stdout().flush();
            }
            res = Some(vec![21, b'\n']);
        }
        if res.is_some() { self.buffer.clear(); self.history_idx = 0; }
        res
    }

    fn update_cwd(&mut self, cmd: &str) {
        if cmd.starts_with("cd") {
            let tgt = cmd.split_whitespace().nth(1).unwrap_or("~"); let new_dir = if tgt == "-" { self.nav_history.clone() } else if tgt.starts_with('~') { dirs::home_dir().map(|mut h| { if tgt.len()>2 { h.push(&tgt[2..]); } h.to_string_lossy().to_string() }).unwrap_or_default() } else { let p = Path::new(tgt); if p.is_absolute() { p.to_string_lossy().to_string() } else { Path::new(&self.current_dir).join(p).to_string_lossy().to_string() } };
            if let Ok(c) = std::fs::canonicalize(&new_dir) { self.nav_history = self.current_dir.clone(); self.current_dir = c.to_string_lossy().to_string(); }
        }
    }
}