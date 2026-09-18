use rusqlite::{Connection, Result};

pub fn get_db_path() -> String {
    let mut p = dirs::home_dir().unwrap_or_default();
    if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
        p.push(".smart_term_dev_history.sqlite");
    } else {
        p.push(".smart_term_history.sqlite");
    }
    p.to_string_lossy().to_string()
}

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open(&get_db_path())?;
    conn.execute("CREATE TABLE IF NOT EXISTS commands (id INTEGER PRIMARY KEY, cmd TEXT NOT NULL, cwd TEXT NOT NULL, project TEXT, namespace TEXT, user TEXT, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)", [])?;
    conn.execute("CREATE TABLE IF NOT EXISTS users (username TEXT PRIMARY KEY, password TEXT NOT NULL, is_default BOOLEAN DEFAULT 0)", [])?;
    conn.execute("CREATE TABLE IF NOT EXISTS bindings (target TEXT PRIMARY KEY, type TEXT NOT NULL, username TEXT NOT NULL)", [])?;
    Ok(conn)
}