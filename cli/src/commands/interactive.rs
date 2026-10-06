use crate::{NamespaceCommands, UserCommands, ProjectCommands};
use crate::ipc::client::send_request;
use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use std::io::{self, Write};

pub fn handle_namespace(action: NamespaceCommands) {
    match action {
        NamespaceCommands::Use { name } => {
            print!("\x1b[33mEnter project for '{}' [default]: \x1b[0m", name); io::stdout().flush().unwrap();
            let mut project = String::new(); let _ = io::stdin().read_line(&mut project); let mut proj = project.trim().to_string();
            if proj.is_empty() { proj = "default".to_string(); }
            if let Ok(_) = send_request(ClientRequest::Interactive { command: "namespace use".to_string(), args: vec![name, proj] }) { println!("\x1b[32m[System] Namespace changed successfully.\x1b[0m"); }
        }
        NamespaceCommands::Exit => { if let Ok(_) = send_request(ClientRequest::Interactive { command: "namespace exit".to_string(), args: vec![] }) { println!("\x1b[32m[System] Namespace cleared.\x1b[0m"); } }
        NamespaceCommands::List => { if let Ok(DaemonResponse::List(items)) = send_request(ClientRequest::Interactive { command: "namespace list".to_string(), args: vec![] }) { if items.is_empty() { println!("\x1b[33mNo namespaces found.\x1b[0m"); } else { println!("\x1b[36mNamespaces:\x1b[0m\n{}", items.join("\n")); } } }
    }
}

pub fn handle_user(action: UserCommands) {
    match action {
        UserCommands::Login { username } => { if let Ok(_) = send_request(ClientRequest::Interactive { command: "user login".to_string(), args: vec![username] }) { println!("\x1b[32m[System] User changed successfully.\x1b[0m"); } }
        UserCommands::Logout => { if let Ok(_) = send_request(ClientRequest::Interactive { command: "user logout".to_string(), args: vec![] }) { println!("\x1b[32m[System] User cleared.\x1b[0m"); } }
        UserCommands::List => { if let Ok(DaemonResponse::List(items)) = send_request(ClientRequest::Interactive { command: "user list".to_string(), args: vec![] }) { if items.is_empty() { println!("\x1b[33mNo users found.\x1b[0m"); } else { println!("\x1b[36mUsers:\x1b[0m\n{}", items.join("\n")); } } }
    }
}

pub fn handle_project(action: ProjectCommands) {
    match action {
        ProjectCommands::Set { name } => { if let Ok(_) = send_request(ClientRequest::Interactive { command: "project set".to_string(), args: vec![name] }) { println!("\x1b[32m[System] Project changed successfully.\x1b[0m"); } }
        ProjectCommands::Clear => { if let Ok(_) = send_request(ClientRequest::Interactive { command: "project clear".to_string(), args: vec![] }) { println!("\x1b[32m[System] Project cleared.\x1b[0m"); } }
    }
}

pub fn handle_unset(target: Option<String>) {
    let t = target.unwrap_or_else(|| "all".to_string()).to_lowercase();
    if let Ok(_) = send_request(ClientRequest::Interactive { command: format!("unset {}", t), args: vec![] }) {
        println!("\x1b[32m[System] Successfully unset '{}'.\x1b[0m", t);
    }
}
