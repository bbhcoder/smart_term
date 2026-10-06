use rusqlite::{Connection, Result};
pub fn init_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (username TEXT PRIMARY KEY, password TEXT NOT NULL, is_default BOOLEAN DEFAULT 0);
         CREATE TABLE IF NOT EXISTS namespaces (name TEXT PRIMARY KEY);
         CREATE TABLE IF NOT EXISTS projects (name TEXT PRIMARY KEY, namespace_name TEXT NOT NULL, FOREIGN KEY(namespace_name) REFERENCES namespaces(name) ON DELETE CASCADE);
         CREATE TABLE IF NOT EXISTS history (id INTEGER PRIMARY KEY AUTOINCREMENT, command TEXT NOT NULL, cwd TEXT NOT NULL, namespace TEXT, project TEXT, user TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY(namespace) REFERENCES namespaces(name) ON DELETE SET NULL, FOREIGN KEY(project) REFERENCES projects(name) ON DELETE SET NULL, FOREIGN KEY(user) REFERENCES users(username) ON DELETE SET NULL);
         CREATE TABLE IF NOT EXISTS session_state (key TEXT PRIMARY KEY, value TEXT NOT NULL);
         CREATE TABLE IF NOT EXISTS bindings (target TEXT PRIMARY KEY, bind_type TEXT NOT NULL, value TEXT NOT NULL);"
    )?;
    Ok(())
}
