use rusqlite::{params, Connection, Result};
use crate::models::SessionState;

pub fn get_state(conn: &Connection) -> Result<SessionState> {
    let get_val = |k: &str| -> Option<String> { conn.query_row("SELECT value FROM session_state WHERE key = ?1", params![k], |r| r.get(0)).ok() };
    Ok(SessionState { active_user: get_val("active_user"), active_namespace: get_val("active_namespace"), active_project: get_val("active_project") })
}

pub fn set_state(conn: &Connection, key: &str, value: Option<&str>) -> Result<()> {
    if let Some(v) = value { conn.execute("INSERT OR REPLACE INTO session_state (key, value) VALUES (?1, ?2)", params![key, v])?; }
    else { conn.execute("DELETE FROM session_state WHERE key = ?1", params![key])?; }
    Ok(())
}

pub fn set_namespace_and_project(conn: &Connection, ns: &str, proj: &str) -> Result<()> {
    conn.execute("INSERT OR IGNORE INTO namespaces (name) VALUES (?1)", params![ns])?;
    conn.execute("INSERT OR IGNORE INTO projects (name, namespace_name) VALUES (?1, ?2)", params![proj, ns])?;
    Ok(())
}

pub fn insert_user(conn: &Connection, username: &str) -> Result<()> {
    conn.execute("INSERT OR IGNORE INTO users (username, password) VALUES (?1, 'local')", params![username])?;
    Ok(())
}

pub fn list_namespaces(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT name FROM namespaces ORDER BY name ASC")?;
    let rows = stmt.query_map([], |r| r.get(0))?;
    Ok(rows.filter_map(Result::ok).collect())
}

pub fn list_users(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT username FROM users ORDER BY username ASC")?;
    let rows = stmt.query_map([], |r| r.get(0))?;
    Ok(rows.filter_map(Result::ok).collect())
}
