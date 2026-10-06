use crate::BindCommands;
use crate::ipc::client::send_request;
use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use std::env;

pub fn handle(action: BindCommands) {
    let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
    let req = match action {
        BindCommands::Project { name } => ClientRequest::BindPath { target: cwd, bind_type: "project".to_string(), value: name },
        BindCommands::User { username } => ClientRequest::BindPath { target: cwd, bind_type: "user".to_string(), value: username },
        BindCommands::Clear => ClientRequest::ClearBind { target: cwd },
    };
    match send_request(req) {
        Ok(DaemonResponse::Success) => println!("\x1b[32m[System] Directory binding successfully updated.\x1b[0m"),
        Ok(DaemonResponse::Error(e)) => println!("\x1b[31m[Error] {}\x1b[0m", e),
        _ => println!("\x1b[31m[Error] Failed to execute bind command.\x1b[0m"),
    }
}

pub fn handle_unbind() {
    let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
    match send_request(ClientRequest::ClearBind { target: cwd }) {
        Ok(DaemonResponse::StateSync(_)) | Ok(DaemonResponse::Success) => println!("\x1b[32m[System] Path unbound successfully.\x1b[0m"),
        _ => println!("\x1b[31m[Error] Failed to unbind path.\x1b[0m"),
    }
}
