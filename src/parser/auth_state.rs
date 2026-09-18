use crate::parser::interceptor::CommandInterceptor;
use crate::{history::queries, os::sys_cmd};
use std::{path::Path, io::{self, Write}};

pub fn process_auth(ctx: &mut CommandInterceptor, bytes: &[u8]) -> Vec<u8> {
    if ctx.os_wait_pass {
        if bytes == [27, 91, 65] || bytes == [27, 91, 66] { return vec![]; }
        if bytes.iter().any(|&b| b == b'\r' || b == b'\n' || b == 3) { ctx.os_wait_pass = false; ctx.buffer.clear(); }
        return bytes.to_vec();
    }
    if ctx.del_mode > 0 {
        if bytes == [27, 91, 65] || bytes == [27, 91, 66] { return vec![]; }
        for &b in bytes {
            if b == 3 { ctx.del_mode = 0; ctx.buffer.clear(); ctx.del_target = None; ctx.del_users.clear(); let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[33m[System] Cancelled\x1b[0m\r\n"); return vec![21, b'\n']; }
            if ctx.del_mode == 1 {
                let ch = b.to_ascii_lowercase(); let tgt = ctx.del_target.clone().unwrap_or_default();
                if ch == b'n' { 
                    if ctx.session.active_user() == tgt { ctx.session.logout(); }
                    let _ = queries::delete_user_history(&tgt); let _ = queries::delete_user(&tgt); let mut out = vec![21, b'\n']; out.extend_from_slice(sys_cmd::get_delete_user_cmd(&tgt).as_bytes()); let _ = io::stdout().write_all(b"n\r\n\x1b[32m[System] Deleted\x1b[0m\r\n"); ctx.os_wait_pass = true; ctx.del_mode = 0; ctx.del_target = None; return out; 
                } else if ch == b'y' {
                    ctx.del_mode = 2;
                    let mut all = sys_cmd::get_os_users(); if let Ok(us) = queries::get_users() { for (u, _) in us { if !all.contains(&u) { all.push(u); } } }
                    all.retain(|x| x != &tgt); ctx.del_users = all;
                    let mut m = String::from("y\r\n\x1b[32m[System] Select user for history:\x1b[0m\r\n");
                    for (i, u) in ctx.del_users.iter().enumerate() { m.push_str(&format!("  {}. {}\r\n", i + 1, u)); }
                    m.push_str("\x1b[33mEnter number:\x1b[0m\r\n> "); let _ = io::stdout().write_all(m.as_bytes()); let _ = io::stdout().flush(); return vec![];
                }
            }
            match b {
                b'\r' | b'\n' => {
                    let mut out = vec![21, b'\n']; let input = ctx.buffer.trim().to_string(); let tgt = ctx.del_target.clone().unwrap_or_default();
                    if ctx.del_mode == 2 {
                        if let Ok(idx) = input.parse::<usize>() {
                            if idx > 0 && idx <= ctx.del_users.len() {
                                let new_u = &ctx.del_users[idx - 1]; let _ = queries::reassign_history(&tgt, new_u); let _ = queries::delete_user(&tgt);
                                if ctx.session.active_user() == tgt { ctx.session.logout(); }
                                out.extend_from_slice(sys_cmd::get_delete_user_cmd(&tgt).as_bytes()); let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] Assigned to '{}'\x1b[0m\r\n", new_u).as_bytes());
                                ctx.os_wait_pass = true; ctx.del_mode = 0; ctx.del_target = None; ctx.del_users.clear();
                            } else { let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[31m[System] Invalid!\x1b[0m\r\n"); }
                        }
                    }
                    let _ = io::stdout().flush(); ctx.buffer.clear(); return if out.len() > 2 { out } else { vec![21, b'\n'] };
                }
                127 | 8 => { if ctx.buffer.pop().is_some() { let _ = io::stdout().write_all(b"\x08 \x08"); let _ = io::stdout().flush(); } }
                c if c >= 32 && c <= 126 => { ctx.buffer.push(c as char); let _ = io::stdout().write_all(&[c]); let _ = io::stdout().flush(); }
                _ => {}
            }
        }
        return vec![];
    }
    if ctx.pwd_mode > 0 {
        if bytes == [27, 91, 65] || bytes == [27, 91, 66] { return vec![]; }
        for &b in bytes {
            if b == 3 {
                ctx.pwd_mode = 0; ctx.buffer.clear(); ctx.pwd_buf.clear(); ctx.tmp_pwd.clear(); ctx.pending_user = None;
                let _ = io::stdout().write_all(b"\r\n"); return vec![21, b'\n'];
            }
            match b {
                b'\r' | b'\n' => {
                    let mut out = vec![21, b'\n'];
                    if ctx.pwd_mode == 1 {
                        if let Some(u) = &ctx.pending_user {
                            if queries::verify_user(u, &ctx.pwd_buf) { ctx.session.login(u); let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[32m[System] Logged in\x1b[0m\r\n"); }
                        }
                        ctx.pwd_mode = 0;
                    } else if ctx.pwd_mode == 2 || ctx.pwd_mode == 4 {
                        ctx.tmp_pwd = ctx.pwd_buf.clone(); ctx.pwd_mode += 1; ctx.pwd_buf.clear();
                        let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[32m[System] Confirm password:\x1b[0m\r\n> "); let _ = io::stdout().flush(); return vec![21];
                    } else if ctx.pwd_mode == 3 || ctx.pwd_mode == 5 {
                        if ctx.pwd_buf == ctx.tmp_pwd {
                            if let Some(u) = &ctx.pending_user {
                                if ctx.pwd_mode == 3 { let _ = queries::add_user(u, &ctx.pwd_buf); out.extend_from_slice(sys_cmd::get_create_user_cmd(u, &ctx.pwd_buf).as_bytes()); }
                                else { let _ = queries::update_user_pass(u, &ctx.pwd_buf); out.extend_from_slice(sys_cmd::get_change_pass_cmd(u, &ctx.pwd_buf).as_bytes()); }
                                ctx.os_wait_pass = true;
                            }
                        }
                        ctx.pwd_mode = 0;
                    }
                    let _ = io::stdout().flush(); ctx.pwd_buf.clear(); ctx.tmp_pwd.clear(); ctx.pending_user = None; ctx.buffer.clear(); return out;
                }
                127 | 8 => { if ctx.pwd_buf.pop().is_some() { let _ = io::stdout().write_all(b"\x08 \x08"); let _ = io::stdout().flush(); } }
                c if c >= 32 && c <= 126 => { ctx.pwd_buf.push(c as char); let _ = io::stdout().write_all(b"*"); let _ = io::stdout().flush(); }
                _ => {}
            }
        }
    }
    vec![]
}

pub fn handle_arrows(ctx: &mut CommandInterceptor, bytes: &[u8]) -> Vec<u8> {
    if bytes == [27, 91, 65] {
        if ctx.history.is_empty() { ctx.history = queries::get_hist(&ctx.current_dir, ctx.active_project.as_deref(), ctx.active_namespace.as_deref(), ctx.session.current_user.as_deref()).unwrap_or_default(); ctx.history_idx = 0; }
        if ctx.history_idx < ctx.history.len() { ctx.buffer = ctx.history[ctx.history_idx].clone(); ctx.history_idx += 1; let mut out = vec![21]; out.extend_from_slice(ctx.buffer.as_bytes()); return out; }
    }
    if bytes == [27, 91, 66] {
        if ctx.history_idx > 0 { ctx.history_idx -= 1; let mut out = vec![21]; if ctx.history_idx > 0 { ctx.buffer = ctx.history[ctx.history_idx - 1].clone(); out.extend_from_slice(ctx.buffer.as_bytes()); } else { ctx.buffer.clear(); } return out; }
    }
    vec![]
}

pub fn update_cwd(ctx: &mut CommandInterceptor, cmd: &str) {
    if cmd.starts_with("cd") {
        let tgt = cmd.split_whitespace().nth(1).unwrap_or("~");
        let new_dir = if tgt == "-" { ctx.nav_history.clone() } else if tgt.starts_with('~') { dirs::home_dir().map(|mut h| { if tgt.len()>2 { h.push(&tgt[2..]); } h.to_string_lossy().to_string() }).unwrap_or_default() } else { let p = Path::new(tgt); if p.is_absolute() { p.to_string_lossy().to_string() } else { Path::new(&ctx.current_dir).join(p).to_string_lossy().to_string() } };
        if let Ok(c) = std::fs::canonicalize(&new_dir) { ctx.nav_history = ctx.current_dir.clone(); ctx.current_dir = c.to_string_lossy().to_string(); check_bindings(ctx); }
    }
}

pub fn check_bindings(ctx: &mut CommandInterceptor) {
    let tgt = if let Some(ref p) = ctx.active_project { p.clone() } else { ctx.current_dir.clone() };
    if let Some(u) = queries::get_bound_user(&tgt) { ctx.session.login(&u); } else { ctx.session.logout(); }
}