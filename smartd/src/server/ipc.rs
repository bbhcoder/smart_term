use tokio::io::{AsyncReadExt, AsyncWriteExt};
use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use smartcore::abi::protocol::{deserialize_request, serialize_response};
use smartcore::cache::state_cache::StateCache;
use smartcore::engine::{preexec, precmd};
use db::connection::get_connection;
use db::queries::{history::{get_history, delete_history, delete_all_history}, state::{set_namespace_and_project, insert_user, list_namespaces, list_users}, bindings::{insert_binding, delete_binding}};

#[cfg(unix)]
pub async fn start_server(sock: &str, cache: StateCache) -> Result<(), String> {
    let _ = std::fs::remove_file(sock);
    let listener = tokio::net::UnixListener::bind(sock).map_err(|e| e.to_string())?;
    loop { if let Ok((mut stream, _)) = listener.accept().await { let c = cache.clone(); tokio::spawn(async move { handle_stream(&mut stream, c).await; }); } }
}

#[cfg(windows)]
pub async fn start_server(_sock: &str, cache: StateCache) -> Result<(), String> {
    use tokio::net::windows::named_pipe::ServerOptions;
    loop { if let Ok(mut server) = ServerOptions::new().create(r"\\.\pipe\smart_term_ipc") { if server.connect().await.is_ok() { let c = cache.clone(); tokio::spawn(async move { handle_stream(&mut server, c).await; }); } } }
}

async fn handle_stream<T: AsyncReadExt + AsyncWriteExt + Unpin>(stream: &mut T, cache: StateCache) {
    let mut buf = vec![0u8; 8192];
    if let Ok(n) = stream.read(&mut buf).await {
        if n > 0 { if let Ok(req_str) = String::from_utf8(buf[..n].to_vec()) { if let Ok(req) = deserialize_request(&req_str) { if let Ok(res_str) = serialize_response(&handle_request(req, &cache).await) { let _ = stream.write_all(res_str.as_bytes()).await; } } } }
    }
}

async fn handle_request(req: ClientRequest, cache: &StateCache) -> DaemonResponse {
    match req {
        ClientRequest::PreExec { command, cwd } => { let state = cache.get(); match tokio::task::spawn_blocking(move || preexec::process(&command, &cwd, &state)).await { Ok(Ok(_)) => DaemonResponse::Success, Ok(Err(e)) => DaemonResponse::Error(e), _ => DaemonResponse::Error("Err".into()) } }
        ClientRequest::PreCmd { cwd, exit_code } => { let mut state = cache.get(); match tokio::task::spawn_blocking(move || { (precmd::process(&cwd, exit_code, &mut state), state) }).await { Ok((Ok(_), s)) => { cache.update(s.clone()); DaemonResponse::StateSync(s) }, _ => DaemonResponse::Error("Err".into()) } }
        ClientRequest::Interactive { command, args } => {
            let mut state = cache.get();
            match tokio::task::spawn_blocking(move || {
                if command == "namespace use" && args.len() >= 2 { if let Ok(c) = get_connection() { let _ = set_namespace_and_project(&c, &args[0], &args[1]); } state.active_namespace = Some(args[0].clone()); state.active_project = Some(args[1].clone()); Ok(DaemonResponse::StateSync(state))
                } else if command == "namespace exit" { state.active_namespace = None; state.active_project = None; Ok(DaemonResponse::StateSync(state))
                } else if command == "user login" && !args.is_empty() { if let Ok(c) = get_connection() { let _ = insert_user(&c, &args[0]); } state.active_user = Some(args[0].clone()); Ok(DaemonResponse::StateSync(state))
                } else if command == "user logout" { state.active_user = None; Ok(DaemonResponse::StateSync(state))
                } else if command == "project set" && !args.is_empty() { if let Ok(c) = get_connection() { let _ = set_namespace_and_project(&c, state.active_namespace.as_deref().unwrap_or("default"), &args[0]); } if state.active_namespace.is_none() { state.active_namespace = Some("default".into()); } state.active_project = Some(args[0].clone()); Ok(DaemonResponse::StateSync(state))
                } else if command == "project clear" { state.active_project = None; Ok(DaemonResponse::StateSync(state))
                } else if command.starts_with("unset ") {
                    let t = command.trim_start_matches("unset ");
                    if t == "user" { state.active_user = None; } else if t == "project" { state.active_project = None; } else if t == "namespace" { state.active_namespace = None; } else { state.active_project = None; state.active_namespace = None; state.active_user = None; }
                    Ok(DaemonResponse::StateSync(state))
                } else if command == "namespace list" { Ok(DaemonResponse::List(get_connection().and_then(|c| list_namespaces(&c)).unwrap_or_default()))
                } else if command == "user list" { Ok(DaemonResponse::List(get_connection().and_then(|c| list_users(&c)).unwrap_or_default()))
                } else { Err("Unknown".to_string()) }
            }).await { Ok(Ok(DaemonResponse::StateSync(s))) => { cache.update(s.clone()); DaemonResponse::StateSync(s) } Ok(Ok(r)) => r, Ok(Err(e)) => DaemonResponse::Error(e), _ => DaemonResponse::Error("Err".into()) }
        }
        ClientRequest::GetHistory { cwd, limit } => { let state = cache.get(); match tokio::task::spawn_blocking(move || { get_history(&get_connection().map_err(|_| "DB")?, cwd.as_deref(), state.active_namespace.as_deref(), state.active_project.as_deref(), state.active_user.as_deref(), limit).map_err(|e| e.to_string()) }).await { Ok(Ok(cmds)) => DaemonResponse::History(cmds), Ok(Err(e)) => DaemonResponse::Error(e), _ => DaemonResponse::Error("Err".into()) } }
        ClientRequest::ResetHistory { cwd } => { let state = cache.get(); match tokio::task::spawn_blocking(move || { delete_history(&get_connection().map_err(|_| "DB")?, &cwd, &state).map_err(|e| e.to_string()) }).await { Ok(Ok(_)) => DaemonResponse::Success, Ok(Err(e)) => DaemonResponse::Error(e), _ => DaemonResponse::Error("Err".into()) } }
        ClientRequest::ResetAllHistory => { match tokio::task::spawn_blocking(move || { delete_all_history(&get_connection().map_err(|_| "DB")?).map_err(|e| e.to_string()) }).await { Ok(Ok(_)) => DaemonResponse::Success, Ok(Err(e)) => DaemonResponse::Error(e), _ => DaemonResponse::Error("Err".into()) } }
        ClientRequest::BindPath { target, bind_type, value } => { match tokio::task::spawn_blocking(move || { insert_binding(&get_connection().map_err(|_| "DB")?, &target, &bind_type, &value).map_err(|e| e.to_string()) }).await { Ok(Ok(_)) => DaemonResponse::Success, Ok(Err(e)) => DaemonResponse::Error(e), _ => DaemonResponse::Error("Err".into()) } }
        ClientRequest::ClearBind { target } => {
            let mut state = cache.get();
            match tokio::task::spawn_blocking(move || {
                if let Ok(c) = get_connection() { let _ = delete_binding(&c, &target); }
                state.active_project = None; state.active_namespace = None; state.active_user = None;
                Ok(DaemonResponse::StateSync(state))
            }).await { Ok(Ok(DaemonResponse::StateSync(s))) => { cache.update(s.clone()); DaemonResponse::StateSync(s) } Ok(Ok(r)) => r, Ok(Err(e)) => DaemonResponse::Error(e), _ => DaemonResponse::Error("Err".into()) }
        }
    }
}
