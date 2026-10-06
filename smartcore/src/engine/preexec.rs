use db::connection::get_connection;
use db::queries::history::insert_command;
use db::models::SessionState;

pub fn process(command: &str, cwd: &str, state: &SessionState) -> Result<(), String> {
    let cmd = command.trim();
    // هیچ استثنای کثیفی اینجا نداریم. هسته فقط داده‌های معتبر دریافت می‌کند.
    if cmd.is_empty() {
        return Ok(());
    }
    
    let conn = get_connection().map_err(|e| e.to_string())?;
    insert_command(&conn, cmd, cwd, state).map_err(|e| e.to_string())?;
    Ok(())
}
