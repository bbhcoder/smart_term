use db::connection::get_connection;
use db::queries::bindings::get_binding;
use db::models::SessionState;
use std::path::Path;

pub fn process(cwd: &str, _exit_code: i32, state: &mut SessionState) -> Result<(), String> {
    if let Ok(conn) = get_connection() {
        let path = Path::new(cwd);
        for ancestor in path.ancestors() {
            let p = ancestor.to_string_lossy();
            if let Some((bind_type, value)) = get_binding(&conn, &p) {
                if bind_type == "user" {
                    state.active_user = Some(value);
                } else if bind_type == "project" {
                    state.active_project = Some(value);
                }
                break;
            }
        }
    }
    Ok(())
}
