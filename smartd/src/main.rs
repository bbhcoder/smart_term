pub mod server;
use smartcore::cache::state_cache::StateCache;
use db::models::SessionState;
use db::connection::get_connection;
use db::queries::state::get_state;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    let base_dir = env::var("SMART_BASE_DIR").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
        format!("{}/.smart_dev", home)
    });
    let _ = fs::create_dir_all(&base_dir);
    let socket_path = format!("{}/smart_term.sock", base_dir);
    
    let initial_state = get_connection()
        .and_then(|conn| get_state(&conn))
        .unwrap_or(SessionState {
            active_user: None,
            active_namespace: None,
            active_project: None,
        });
        
    let cache = StateCache::new(initial_state);
    let _ = server::ipc::start_server(&socket_path, cache).await;
}
