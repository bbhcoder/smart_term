use crate::{parser::flags, auth::session::UserSession, os::sys_cmd, history::queries};
use crate::parser::{commands, auth_state};
use std::{env, io::{self, Write}};

pub struct CommandInterceptor {
    pub(crate) buffer: String,
    pub(crate) current_dir: String,
    pub(crate) nav_history: String,
    pub(crate) in_escape: bool,
    pub(crate) history: Vec<String>,
    pub(crate) history_idx: usize,
    pub(crate) active_project: Option<String>,
    pub(crate) active_namespace: Option<String>,
    pub(crate) session: UserSession,
    pub(crate) pwd_mode: u8,
    pub(crate) pwd_buf: String,
    pub(crate) tmp_pwd: String,
    pub(crate) pending_user: Option<String>,
    pub(crate) os_wait_pass: bool,
    pub(crate) del_mode: u8,
    pub(crate) del_target: Option<String>,
    pub(crate) del_users: Vec<String>,
}

impl CommandInterceptor {
    pub fn new() -> Self {
        let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
        Self {
            buffer: String::new(), current_dir: cwd.clone(), nav_history: cwd,
            in_escape: false, history: vec![], history_idx: 0,
            active_project: None, active_namespace: None, session: UserSession::new(),
            pwd_mode: 0, pwd_buf: String::new(), tmp_pwd: String::new(),
            pending_user: None, os_wait_pass: false, del_mode: 0,
            del_target: None, del_users: Vec::new()
        }
    }

    pub fn process_bytes(&mut self, bytes: &[u8]) -> Vec<u8> {
        if self.os_wait_pass || self.del_mode > 0 || self.pwd_mode > 0 {
            return auth_state::process_auth(self, bytes);
        }
        if bytes == [27, 91, 65] || bytes == [27, 91, 66] {
            return auth_state::handle_arrows(self, bytes);
        }
        self.history.clear();
        self.history_idx = 0;
        for &b in bytes {
            if b == 3 { self.buffer.clear(); let _ = io::stdout().write_all(b"\r\n"); return vec![21, 3]; }
            if self.in_escape { if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || b == b'~' { self.in_escape = false; } continue; }
            if b == 27 { self.in_escape = true; continue; }
            match b {
                b'\r' | b'\n' => {
                    let cmd = self.buffer.trim().to_string();
                    let mut is_sec = false;
                    
                    if !cmd.is_empty() && flags::should_log(&cmd) {
                        let cln = flags::clean_command(&cmd);
                        // شناسایی دستوراتی که احتمالاً تو خط بعدی پسورد می‌خوان
                        if cln.starts_with("sudo ") || cln.starts_with("su ") || cln.starts_with("ssh ") {
                            is_sec = true;
                        }
                        let sys_usr = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_default();
                        let active = self.session.active_user();
                        let usr = if active != "default" && active != sys_usr { Some(active.to_string()) } else { None };
                        let _ = queries::insert_cmd(&cln, &self.current_dir, self.active_project.as_deref(), self.active_namespace.as_deref(), usr.as_deref());
                    }

                    if let Some(fb) = commands::handle_special(self, &cmd) { 
                        if is_sec { self.os_wait_pass = true; } // بی‌صدا کردن خط بعدی برای امنیت
                        return fb; 
                    }

                    if !cmd.is_empty() && flags::should_log(&cmd) {
                        let cln = flags::clean_command(&cmd);
                        auth_state::update_cwd(self, &cln);
                        let sys_usr = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_default();
                        let active = self.session.active_user();
                        let usr = if active != "default" && active != sys_usr { Some(active.to_string()) } else { None };
                        if let Some(u) = usr {
                            if !cln.starts_with("cd") && !cln.starts_with("su ") && !cln.starts_with("sudo ") && !cln.starts_with("runas ") {
                                self.os_wait_pass = true; self.buffer.clear();
                                let mut out = vec![21]; out.extend_from_slice(sys_cmd::get_switch_user_cmd(&u, &cln).as_bytes()); 
                                return out;
                            }
                        }
                    }
                    
                    if is_sec { self.os_wait_pass = true; }
                    self.buffer.clear();
                }
                127 | 8 => { self.buffer.pop(); }
                b if b >= 32 && b <= 126 => { self.buffer.push(b as char); }
                _ => {}
            }
        }
        bytes.to_vec()
    }
}