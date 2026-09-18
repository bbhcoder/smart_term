use crate::{parser::flags, history::queries, auth::session::UserSession, os::sys_cmd};
use std::{env, path::Path, io::{self, Write}};

pub struct CommandInterceptor {
    buffer: String, current_dir: String, nav_history: String, in_escape: bool,
    history: Vec<String>, history_idx: usize, active_project: Option<String>,
    active_namespace: Option<String>, session: UserSession,
    pwd_mode: u8, pwd_buf: String, tmp_pwd: String, pending_user: Option<String>,
    os_wait_pass: bool,
}

impl CommandInterceptor {
    pub fn new() -> Self {
        let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
        Self { buffer: String::new(), current_dir: cwd.clone(), nav_history: cwd, in_escape: false, history: vec![], history_idx: 0, active_project: None, active_namespace: None, session: UserSession::new(), pwd_mode: 0, pwd_buf: String::new(), tmp_pwd: String::new(), pending_user: None, os_wait_pass: false }
    }

    pub fn process_bytes(&mut self, bytes: &[u8]) -> Vec<u8> {
        if self.os_wait_pass {
            if bytes == [27, 91, 65] || bytes == [27, 91, 66] { return vec![]; }
            let mut has_enter = false;
            for &b in bytes { if b == b'\r' || b == b'\n' { has_enter = true; } }
            if has_enter { self.os_wait_pass = false; self.buffer.clear(); }
            return bytes.to_vec();
        }

        if self.pwd_mode > 0 {
            if bytes == [27, 91, 65] || bytes == [27, 91, 66] { return vec![]; }
            for &b in bytes {
                if self.in_escape { if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || b == b'~' { self.in_escape = false; } continue; }
                if b == 27 { self.in_escape = true; continue; }
                match b {
                    b'\r' | b'\n' => {
                        let mut out = vec![21, b'\n'];
                        if self.pwd_mode == 1 {
                            if let Some(u) = &self.pending_user {
                                if queries::verify_user(u, &self.pwd_buf) { self.session.login(u); let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Logged in\x1b[0m\r\n"); }
                                else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Incorrect password\x1b[0m\r\n"); }
                            }
                            self.pwd_mode = 0;
                        } else if self.pwd_mode == 2 || self.pwd_mode == 4 {
                            self.tmp_pwd = self.pwd_buf.clone(); self.pwd_mode += 1; self.pwd_buf.clear();
                            let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Confirm: \x1b[0m"); let _ = io::stdout().flush(); return vec![21];
                        } else if self.pwd_mode == 3 || self.pwd_mode == 5 {
                            if self.pwd_buf == self.tmp_pwd {
                                if let Some(u) = &self.pending_user {
                                    if self.pwd_mode == 3 {
                                        let _ = queries::add_user(u, &self.pwd_buf); out.extend_from_slice(sys_cmd::get_create_user_cmd(u, &self.pwd_buf).as_bytes());
                                        let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Creating...\x1b[0m\r\n");
                                    } else {
                                        let _ = queries::update_user_pass(u, &self.pwd_buf); out.extend_from_slice(sys_cmd::get_change_pass_cmd(u, &self.pwd_buf).as_bytes());
                                        let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Updating...\x1b[0m\r\n");
                                    }
                                    self.os_wait_pass = true;
                                }
                            } else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Mismatch!\x1b[0m\r\n"); }
                            self.pwd_mode = 0;
                        }
                        let _ = io::stdout().flush(); self.pwd_buf.clear(); self.tmp_pwd.clear(); self.pending_user = None; self.buffer.clear(); return out;
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
                        let sys_usr = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_default();
                        let active = self.session.active_user(); let usr = if active != "default" && active != sys_usr { Some(active) } else { None };
                        let _ = queries::insert_cmd(&cln, &ed, self.active_project.as_deref(), self.active_namespace.as_deref(), usr);
                        if let Some(u) = usr {
                            if !cln.starts_with("cd") && !cln.starts_with("su ") && !cln.starts_with("sudo ") && !cln.starts_with("runas ") {
                                self.os_wait_pass = true;
                                self.buffer.clear(); let mut out = vec![21]; out.extend_from_slice(sys_cmd::get_switch_user_cmd(u, &cln).as_bytes()); return out;
                            }
                        }
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
        else if cmd.starts_with("login project ") || cmd.starts_with("save project ") { self.active_project = cmd.split_whitespace().nth(2).map(String::from); self.check_bindings(); res = Some(vec![21, b'\n']); }
        else if cmd.starts_with("exit project") { self.active_project = None; self.check_bindings(); res = Some(vec![21, b'\n']); }
        else if cmd.starts_with("namespace use ") { self.active_namespace = cmd.split_whitespace().nth(2).map(String::from); res = Some(vec![21, b'\n']); }
        else if cmd == "namespace exit" { self.active_namespace = None; res = Some(vec![21, b'\n']); }
        else if cmd.starts_with("user ") && cmd.ends_with(" pass change") {
            let p: Vec<&str> = cmd.split_whitespace().collect();
            if p.len() == 4 && queries::user_exists(p[1]) { self.pending_user = Some(p[1].to_string()); self.pwd_mode = 4; let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] New pass for {}: \x1b[0m", p[1]).as_bytes()); }
            else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] User not found!\x1b[0m\r\n"); }
            let _ = io::stdout().flush(); res = Some(vec![21]);
        }
        else if cmd.starts_with("user add ") {
            if let Some(u) = cmd.split_whitespace().nth(2) { self.pending_user = Some(u.to_string()); self.pwd_mode = 2; let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] Pass for {}: \x1b[0m", u).as_bytes()); }
            else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Usage: user add <name>\x1b[0m\r\n"); }
            let _ = io::stdout().flush(); res = Some(vec![21]);
        }
        else if cmd.starts_with("login user ") { if let Some(u) = cmd.split_whitespace().nth(2) { if queries::user_exists(u) { self.pending_user = Some(u.to_string()); self.pwd_mode = 1; let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] Pass for {}: \x1b[0m", u).as_bytes()); } else { let _ = io::stdout().write_all(b"\r\n\x1b[31m[System] Not found!\x1b[0m\r\n"); } let _ = io::stdout().flush(); } res = Some(vec![21]); }
        else if cmd == "user logout" { self.session.logout(); res = Some(vec![21, b'\n']); }
        else if cmd == "user info" { let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] User: {}\x1b[0m\r\n", self.session.active_user()).as_bytes()); let _ = io::stdout().flush(); res = Some(vec![21, b'\n']); }
        else if cmd == "user list" { 
            let mut m = String::from("\r\n\x1b[32m[System] Users:\x1b[0m\r\n"); 
            let mut all = sys_cmd::get_os_users();
            let def_u = queries::get_def_user().unwrap_or_default();
            if let Ok(us) = queries::get_users() { for (u, _) in us { if !all.contains(&u) { all.push(u); } } }
            for u in all { m.push_str(&format!("  - {}{}\r\n", u, if u == def_u {" (Def)"} else {""})); }
            let _ = io::stdout().write_all(m.as_bytes()); let _ = io::stdout().flush(); res = Some(vec![21, b'\n']); 
        }
        else if cmd.starts_with("user set default ") { if let Some(u) = cmd.split_whitespace().nth(3) { let _ = queries::set_def_user(u); let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] Def: {}\x1b[0m\r\n", u).as_bytes()); let _ = io::stdout().flush(); } res = Some(vec![21, b'\n']); }
        else if cmd.starts_with("bind user ") { if let Some(u) = cmd.split_whitespace().nth(2) { let (tgt, typ) = if let Some(ref p) = self.active_project { (p.clone(), "project") } else { (self.current_dir.clone(), "path") }; let _ = queries::bind_user(&tgt, typ, u); self.check_bindings(); let _ = io::stdout().write_all(format!("\r\n\x1b[32m[System] Bound '{}'\x1b[0m\r\n", u).as_bytes()); let _ = io::stdout().flush(); } res = Some(vec![21, b'\n']); }
        else if cmd == "unbind user" { let tgt = if let Some(ref p) = self.active_project { p.clone() } else { self.current_dir.clone() }; let _ = queries::unbind_user(&tgt); self.check_bindings(); let _ = io::stdout().write_all(b"\r\n\x1b[32m[System] Unbound\x1b[0m\r\n"); let _ = io::stdout().flush(); res = Some(vec![21, b'\n']); }
        if res.is_some() { self.buffer.clear(); self.history_idx = 0; }
        res
    }

    fn update_cwd(&mut self, cmd: &str) {
        if cmd.starts_with("cd") {
            let tgt = cmd.split_whitespace().nth(1).unwrap_or("~"); let new_dir = if tgt == "-" { self.nav_history.clone() } else if tgt.starts_with('~') { dirs::home_dir().map(|mut h| { if tgt.len()>2 { h.push(&tgt[2..]); } h.to_string_lossy().to_string() }).unwrap_or_default() } else { let p = Path::new(tgt); if p.is_absolute() { p.to_string_lossy().to_string() } else { Path::new(&self.current_dir).join(p).to_string_lossy().to_string() } };
            if let Ok(c) = std::fs::canonicalize(&new_dir) { self.nav_history = self.current_dir.clone(); self.current_dir = c.to_string_lossy().to_string(); self.check_bindings(); }
        }
    }

    fn check_bindings(&mut self) {
        let tgt = if let Some(ref p) = self.active_project { p.clone() } else { self.current_dir.clone() };
        if let Some(u) = queries::get_bound_user(&tgt) { self.session.login(&u); } else { self.session.logout(); }
    }
}