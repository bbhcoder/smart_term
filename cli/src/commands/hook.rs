use crate::HookCommands;
use crate::ipc::client::send_fire_and_forget;
use smartcore::abi::messages::ClientRequest;
use std::env;

pub fn handle(action: HookCommands) {
    let cwd = env::current_dir().unwrap_or_default().to_string_lossy().into_owned();
    let req = match action {
        HookCommands::PreExec { command } => ClientRequest::PreExec { command, cwd },
        HookCommands::PreCmd { exit_code } => ClientRequest::PreCmd { cwd, exit_code },
    };
    send_fire_and_forget(req);
}
