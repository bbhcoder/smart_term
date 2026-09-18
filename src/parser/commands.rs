use crate::parser::interceptor::CommandInterceptor;
use std::io::{self, Write};

pub fn handle_special(ctx: &mut CommandInterceptor, cmd: &str) -> Option<Vec<u8>> {
    let res = if cmd.starts_with("user ") || cmd.starts_with("login ") || cmd.starts_with("bind user") || cmd == "unbind user" {
        crate::parser::cmd_user::handle(ctx, cmd)
    } else {
        crate::parser::cmd_sys::handle(ctx, cmd)
    };
    
    if res.is_some() {
        let _ = io::stdout().flush();
        ctx.buffer.clear();
        ctx.history_idx = 0;
    }
    res
}