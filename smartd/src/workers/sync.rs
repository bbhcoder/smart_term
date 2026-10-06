use std::time::Duration;
use tokio::time;
use smartcore::cache::state_cache::StateCache;
use db::connection::get_connection;
use db::queries::state::set_state;

pub async fn start_sync_worker(cache: StateCache) {
    let mut interval = time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        let current_state = cache.get();
        let _ = tokio::task::spawn_blocking(move || {
            if let Ok(conn) = get_connection() {
                let _ = set_state(&conn, "active_user", current_state.active_user.as_deref());
                let _ = set_state(&conn, "active_namespace", current_state.active_namespace.as_deref());
                let _ = set_state(&conn, "active_project", current_state.active_project.as_deref());
            }
        }).await;
    }
}
