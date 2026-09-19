use rusqlite::{params, Connection, Result};
use crate::history::db::get_db_path;

pub fn insert_cmd(cmd: &str, cwd: &str, proj: Option<&str>, ns: Option<&str>, usr: Option<&str>) -> Result<()> {
    let c = Connection::open(&get_db_path())?;
    let _ = c.execute("DELETE FROM commands WHERE cmd = ?1", params![cmd]);
    c.execute("INSERT INTO commands (cmd, cwd, project, namespace, user) VALUES (?1, ?2, ?3, ?4, ?5)", params![cmd, cwd, proj, ns, usr])?;
    Ok(())
}
pub fn get_hist(cwd: &str, proj: Option<&str>, ns: Option<&str>, usr: Option<&str>) -> Result<Vec<String>> {
    let c = Connection::open(&get_db_path())?;
    let mut cmds = Vec::new();
    if let Some(p) = proj {
        let mut s = c.prepare("SELECT cmd FROM commands WHERE project = ?1 AND (namespace = ?2 OR namespace IS NULL) ORDER BY id DESC LIMIT 50")?;
        for r in s.query_map(params![p, ns], |row| row.get::<_, String>(0))?.flatten() { cmds.push(r); }
    } else {
        let mut s = c.prepare("SELECT cmd FROM commands WHERE cwd = ?1 AND project IS NULL AND (namespace = ?2 OR namespace IS NULL) AND (user = ?3 OR user IS NULL) ORDER BY id DESC LIMIT 50")?;
        for r in s.query_map(params![cwd, ns, usr], |row| row.get::<_, String>(0))?.flatten() { cmds.push(r); }
    }
    Ok(cmds)
}
pub fn get_tool_hist(t: &str) -> Result<Vec<String>> {
    let mut cmds = Vec::new();
    for r in Connection::open(&get_db_path())?.prepare("SELECT DISTINCT cmd FROM commands WHERE cmd = ?1 OR cmd LIKE ?2 ORDER BY id DESC LIMIT 50")?.query_map(params![t, format!("{} %", t)], |row| row.get::<_, String>(0))?.flatten() { cmds.push(r); }
    Ok(cmds)
}
pub fn remove_last_cmd() -> Result<()> {
    Connection::open(&get_db_path())?.execute("DELETE FROM commands WHERE id = (SELECT MAX(id) FROM commands)", [])?;
    Ok(())
}
pub fn remove_cmd(cmd: &str) -> Result<()> {
    Connection::open(&get_db_path())?.execute("DELETE FROM commands WHERE cmd = ?1", params![cmd])?;
    Ok(())
}
pub fn clear_all_history() -> Result<()> {
    Connection::open(&get_db_path())?.execute("DELETE FROM commands", [])?;
    Ok(())
}
pub fn add_user(u: &str, p: &str) -> Result<bool> { Ok(Connection::open(&get_db_path())?.execute("INSERT INTO users (username, password) VALUES (?1, ?2)", params![u, p]).is_ok()) }
pub fn update_user_pass(u: &str, p: &str) -> Result<()> { Connection::open(&get_db_path())?.execute("UPDATE users SET password = ?2 WHERE username = ?1", params![u, p])?; Ok(()) }
pub fn verify_user(u: &str, p: &str) -> bool { Connection::open(&get_db_path()).and_then(|c| c.query_row("SELECT password FROM users WHERE username = ?1", params![u], |r| r.get::<_, String>(0))).map_or(false, |db_p| db_p == p) }
pub fn user_exists(u: &str) -> bool { Connection::open(&get_db_path()).and_then(|c| c.query_row("SELECT 1 FROM users WHERE username = ?1", params![u], |r| r.get::<_, i32>(0))).is_ok() }
pub fn get_users() -> Result<Vec<(String, bool)>> { let mut users = Vec::new(); for r in Connection::open(&get_db_path())?.prepare("SELECT username, is_default FROM users")?.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.flatten() { users.push(r); } Ok(users) }
pub fn set_def_user(u: &str) -> Result<()> { let c = Connection::open(&get_db_path())?; c.execute("UPDATE users SET is_default = 0", [])?; c.execute("UPDATE users SET is_default = 1 WHERE username = ?1", params![u])?; Ok(()) }
pub fn get_def_user() -> Option<String> { Connection::open(&get_db_path()).and_then(|c| c.query_row("SELECT username FROM users WHERE is_default = 1", [], |r| r.get(0))).ok() }
pub fn bind_user(t: &str, ty: &str, u: &str) -> Result<()> { Connection::open(&get_db_path())?.execute("INSERT OR REPLACE INTO bindings (target, type, username) VALUES (?1, ?2, ?3)", params![t, ty, u])?; Ok(()) }
pub fn unbind_user(t: &str) -> Result<()> { Connection::open(&get_db_path())?.execute("DELETE FROM bindings WHERE target = ?1", params![t])?; Ok(()) }
pub fn get_bound_user(t: &str) -> Option<String> { Connection::open(&get_db_path()).and_then(|c| c.query_row("SELECT username FROM bindings WHERE target = ?1", params![t], |r| r.get(0))).ok() }
pub fn delete_user(u: &str) -> Result<()> { Connection::open(&get_db_path())?.execute("DELETE FROM users WHERE username = ?1", params![u])?; Ok(()) }
pub fn delete_user_history(u: &str) -> Result<()> { let c = Connection::open(&get_db_path())?; c.execute("DELETE FROM commands WHERE user = ?1", params![u])?; c.execute("DELETE FROM bindings WHERE username = ?1", params![u])?; Ok(()) }
pub fn reassign_history(old_u: &str, new_u: &str) -> Result<()> { let c = Connection::open(&get_db_path())?; c.execute("UPDATE commands SET user = ?2 WHERE user = ?1", params![old_u, new_u])?; c.execute("UPDATE bindings SET username = ?2 WHERE username = ?1", params![old_u, new_u])?; Ok(()) }