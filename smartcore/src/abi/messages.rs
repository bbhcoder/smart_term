use serde::{Deserialize, Serialize};
use db::models::SessionState;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientRequest {
    PreExec { command: String, cwd: String },
    PreCmd { cwd: String, exit_code: i32 },
    Interactive { command: String, args: Vec<String> },
    GetHistory { cwd: Option<String>, limit: u32 },
    ResetHistory { cwd: String },
    ResetAllHistory,
    BindPath { target: String, bind_type: String, value: String },
    ClearBind { target: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DaemonResponse {
    Success,
    Error(String),
    StateSync(SessionState),
    History(Vec<String>),
    List(Vec<String>),
}
