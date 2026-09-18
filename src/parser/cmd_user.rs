use crate::parser::interceptor::CommandInterceptor;
use crate::{history::queries, os::sys_cmd};
use std::io::{self, Write};

pub fn handle(ctx: &mut CommandInterceptor, cmd: &str) -> Option<Vec<u8>> {
    let mut res = None;
    let get_all = || {
        let mut a = sys_cmd::get_os_users();
        if let Ok(us) = queries::get_users() { for (u, _) in us { if !a.contains(&u) { a.push(u); } } }
        a
    };

    if cmd.starts_with("user add ") {
        if let Some(u) = cmd.split_whitespace().nth(2) {
            if !queries::user_exists(u) {
                ctx.pending_user = Some(u.to_string()); ctx.pwd_mode = 2;
                let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] Password for {}:\x1b[0m\r\n> ", u).as_bytes());
                res = Some(vec![21]);
            } else { 
                let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[31m[System] User exists!\x1b[0m\r\n"); res = Some(vec![21, b'\n']); 
            }
        } else { res = Some(vec![21, b'\n']); }
    } else if cmd.starts_with("user remove ") {
        if let Some(u) = cmd.split_whitespace().nth(2) {
            let sys_usr = std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_default();
            if u == sys_usr {
                let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[31m[System] Cannot delete the currently logged-in OS user!\x1b[0m\r\n");
                res = Some(vec![21, b'\n']);
            } else if get_all().contains(&u.to_string()) {
                ctx.del_target = Some(u.to_string()); ctx.del_mode = 1;
                let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[33m[System] Keep history for '{}'? (y/n):\x1b[0m\r\n> ", u).as_bytes());
                res = Some(vec![21]);
            } else { 
                let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[31m[System] Not found!\x1b[0m\r\n"); res = Some(vec![21, b'\n']); 
            }
        } else { res = Some(vec![21, b'\n']); }
    } else if cmd.starts_with("user ") && cmd.ends_with(" pass change") {
        let p: Vec<&str> = cmd.split_whitespace().collect();
        if p.len() == 4 && queries::user_exists(p[1]) {
            ctx.pending_user = Some(p[1].to_string()); ctx.pwd_mode = 4;
            let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] New pass for {}:\x1b[0m\r\n> ", p[1]).as_bytes());
            res = Some(vec![21]);
        } else { 
            let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[31m[System] Not found!\x1b[0m\r\n"); res = Some(vec![21, b'\n']); 
        }
    } else if cmd.starts_with("login user ") {
        if let Some(u) = cmd.split_whitespace().nth(2) {
            if queries::user_exists(u) {
                ctx.pending_user = Some(u.to_string()); ctx.pwd_mode = 1;
                let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] Pass for {}:\x1b[0m\r\n> ", u).as_bytes());
                res = Some(vec![21]);
            } else { 
                let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[31m[System] Not found!\x1b[0m\r\n"); res = Some(vec![21, b'\n']); 
            }
        } else { res = Some(vec![21, b'\n']); }
    } else if cmd == "user logout" {
        ctx.session.logout(); res = Some(vec![21, b'\n']);
    } else if cmd == "user info" {
        let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] User: {}\x1b[0m\r\n", ctx.session.active_user()).as_bytes());
        res = Some(vec![21, b'\n']);
    } else if cmd == "user list" {
        let mut m = String::from("\r\n\x1b[2K\x1b[32m[System] Users:\x1b[0m\r\n");
        let def_u = queries::get_def_user().unwrap_or_default();
        for u in get_all() { m.push_str(&format!("  - {}{}\r\n", u, if u == def_u {" (Def)"} else {""})); }
        let _ = io::stdout().write_all(m.as_bytes()); res = Some(vec![21, b'\n']);
    } else if cmd.starts_with("user set default ") {
        if let Some(u) = cmd.split_whitespace().nth(3) {
            let _ = queries::set_def_user(u); let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] Def: {}\x1b[0m\r\n", u).as_bytes());
        }
        res = Some(vec![21, b'\n']);
    } else if cmd.starts_with("bind user ") {
        if let Some(u) = cmd.split_whitespace().nth(2) {
            if get_all().contains(&u.to_string()) || queries::user_exists(u) {
                let (tgt, typ) = if let Some(ref p) = ctx.active_project { (p.clone(), "project") } else { (ctx.current_dir.clone(), "path") };
                let _ = queries::bind_user(&tgt, typ, u); crate::parser::auth_state::check_bindings(ctx);
                let _ = io::stdout().write_all(format!("\r\n\x1b[2K\x1b[32m[System] Bound '{}'\x1b[0m\r\n", u).as_bytes());
            } else {
                let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[31m[System] User not found!\x1b[0m\r\n");
            }
        } else { res = Some(vec![21, b'\n']); }
    } else if cmd == "unbind user" {
        let tgt = if let Some(ref p) = ctx.active_project { p.clone() } else { ctx.current_dir.clone() };
        let _ = queries::unbind_user(&tgt); crate::parser::auth_state::check_bindings(ctx);
        let _ = io::stdout().write_all(b"\r\n\x1b[2K\x1b[32m[System] Unbound\x1b[0m\r\n"); res = Some(vec![21, b'\n']);
    }
    res
}