use rusqlite::{params, Connection, Result};

pub fn insert_cmd(cmd: &str, cwd: &str, proj: Option<&str>, ns: Option<&str>, usr: Option<&str>) -> Result<()> {
    Connection::open("history.sqlite")?.execute("INSERT INTO commands (cmd, cwd, project, namespace, user) VALUES (?1, ?2, ?3, ?4, ?5)", params![cmd, cwd, proj, ns, usr])?;
    Ok(())
}
pub fn get_hist(cwd: &str, proj: Option<&str>, ns: Option<&str>, usr: Option<&str>) -> Result<Vec<String>> {
    let conn = Connection::open("history.sqlite")?;
    let mut cmds = Vec::new();
    if let Some(p) = proj {
        let mut stmt = conn.prepare("SELECT cmd FROM commands WHERE project = ?1 AND (namespace = ?2 OR namespace IS NULL) ORDER BY id DESC LIMIT 50")?;
        let rows = stmt.query_map(params![p, ns], |row| row.get::<_, String>(0))?;
        for c in rows.flatten() { cmds.push(c); }
    } else {
        let mut stmt = conn.prepare("SELECT cmd FROM commands WHERE cwd = ?1 AND project IS NULL AND (namespace = ?2 OR namespace IS NULL) AND (user = ?3 OR user IS NULL) ORDER BY id DESC LIMIT 50")?;
        let rows = stmt.query_map(params![cwd, ns, usr], |row| row.get::<_, String>(0))?;
        for c in rows.flatten() { cmds.push(c); }
    }
    Ok(cmds)
}
pub fn get_tool_hist(tool: &str) -> Result<Vec<String>> {
    let conn = Connection::open("history.sqlite")?;
    let mut stmt = conn.prepare("SELECT DISTINCT cmd FROM commands WHERE cmd = ?1 OR cmd LIKE ?2 ORDER BY id DESC LIMIT 50")?;
    Ok(stmt.query_map(params![tool, format!("{} %", tool)], |row| row.get::<_, String>(0))?.flatten().collect())
}
pub fn add_user(u: &str, p: &str) -> Result<bool> { Ok(Connection::open("history.sqlite")?.execute("INSERT INTO users (username, password) VALUES (?1, ?2)", params![u, p]).is_ok()) }
pub fn update_user_pass(u: &str, p: &str) -> Result<()> { Connection::open("history.sqlite")?.execute("UPDATE users SET password = ?2 WHERE username = ?1", params![u, p])?; Ok(()) }
pub fn verify_user(u: &str, p: &str) -> bool { Connection::open("history.sqlite").and_then(|c| c.query_row("SELECT password FROM users WHERE username = ?1", params![u], |r| r.get::<_, String>(0))).map_or(false, |db_p| db_p == p) }
pub fn user_exists(u: &str) -> bool { Connection::open("history.sqlite").and_then(|c| c.query_row("SELECT 1 FROM users WHERE username = ?1", params![u], |r| r.get::<_, i32>(0))).is_ok() }
pub fn get_users() -> Result<Vec<(String, bool)>> {
    let conn = Connection::open("history.sqlite")?; let mut stmt = conn.prepare("SELECT username, is_default FROM users")?;
    Ok(stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.flatten().collect())
}
pub fn set_def_user(u: &str) -> Result<()> {
    let conn = Connection::open("history.sqlite")?; conn.execute("UPDATE users SET is_default = 0", [])?; conn.execute("UPDATE users SET is_default = 1 WHERE username = ?1", params![u])?; Ok(())
}
pub fn get_def_user() -> Option<String> { Connection::open("history.sqlite").and_then(|c| c.query_row("SELECT username FROM users WHERE is_default = 1", [], |r| r.get(0))).ok() }
pub fn bind_user(t: &str, ty: &str, u: &str) -> Result<()> { Connection::open("history.sqlite")?.execute("INSERT OR REPLACE INTO bindings (target, type, username) VALUES (?1, ?2, ?3)", params![t, ty, u])?; Ok(()) }
pub fn unbind_user(t: &str) -> Result<()> { Connection::open("history.sqlite")?.execute("DELETE FROM bindings WHERE target = ?1", params![t])?; Ok(()) }
pub fn get_bound_user(t: &str) -> Option<String> { Connection::open("history.sqlite").and_then(|c| c.query_row("SELECT username FROM bindings WHERE target = ?1", params![t], |r| r.get(0))).ok() }