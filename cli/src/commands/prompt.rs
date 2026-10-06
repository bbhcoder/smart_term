use crate::ipc::client::send_request;
use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use std::env;

pub fn handle() {
    let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
    let req = ClientRequest::PreCmd { cwd, exit_code: 0 };
    if let Ok(DaemonResponse::StateSync(state)) = send_request(req) {
        let mut parts = Vec::new();
        if let Some(ns) = state.active_namespace { parts.push(format!("NS:{}", ns)); }
        if let Some(p) = state.active_project { parts.push(format!("Proj:{}", p)); }
        if let Some(u) = state.active_user { parts.push(format!("User:{}", u)); }
        if parts.is_empty() {
            print!("[Smart] ");
        } else {
            print!("[{}] ", parts.join("|"));
        }
    }
}
