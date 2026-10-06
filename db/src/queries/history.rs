use rusqlite::{params, Connection, Result};
use crate::models::{HistoryEntry, SessionState};

pub fn insert_command(conn: &Connection, cmd: &str, cwd: &str, state: &SessionState) -> Result<()> {
    conn.execute("INSERT INTO history (command, cwd, namespace, project, user) VALUES (?1, ?2, ?3, ?4, ?5)", params![cmd, cwd, state.active_namespace, state.active_project, state.active_user])?;
    Ok(())
}

pub fn get_history(conn: &Connection, cwd: Option<&str>, ns: Option<&str>, proj: Option<&str>, user: Option<&str>, limit: u32) -> Result<Vec<String>> {
    let mut q = if cwd.is_some() { String::from("SELECT command FROM history WHERE 1=1") } else { String::from("SELECT '[' || cwd || '] ' || command FROM history WHERE 1=1") };
    let mut p: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    if let Some(c) = cwd { q.push_str(" AND cwd = ?"); p.push(Box::new(c.to_string())); }
    if let Some(n) = ns { q.push_str(" AND namespace = ?"); p.push(Box::new(n.to_string())); } else { q.push_str(" AND namespace IS NULL"); }
    if let Some(pr) = proj { q.push_str(" AND project = ?"); p.push(Box::new(pr.to_string())); } else { q.push_str(" AND project IS NULL"); }
    if let Some(u) = user { q.push_str(" AND user = ?"); p.push(Box::new(u.to_string())); } else { q.push_str(" AND user IS NULL"); }
    if cwd.is_some() { q.push_str(" GROUP BY command ORDER BY MAX(id) DESC LIMIT ?"); } else { q.push_str(" GROUP BY command, cwd ORDER BY cwd ASC, MAX(id) DESC LIMIT ?"); }
    p.push(Box::new(limit));
    let mut stmt = conn.prepare(&q)?;
    let p_refs: Vec<&dyn rusqlite::ToSql> = p.iter().map(|x| x.as_ref()).collect();
    let rows = stmt.query_map(rusqlite::params_from_iter(p_refs), |r| r.get(0))?;
    Ok(rows.filter_map(Result::ok).collect())
}

pub fn export_history(conn: &Connection, cwd: Option<&str>, ns: Option<&str>, proj: Option<&str>, user: Option<&str>) -> Result<Vec<HistoryEntry>> {
    let mut q = String::from("SELECT id, command, cwd, namespace, project, user, created_at FROM history WHERE 1=1");
    let mut p: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    if let Some(c) = cwd { q.push_str(" AND cwd = ?"); p.push(Box::new(c.to_string())); }
    if let Some(n) = ns { q.push_str(" AND namespace = ?"); p.push(Box::new(n.to_string())); }
    if let Some(pr) = proj { q.push_str(" AND project = ?"); p.push(Box::new(pr.to_string())); }
    if let Some(u) = user { q.push_str(" AND user = ?"); p.push(Box::new(u.to_string())); }
    q.push_str(" ORDER BY id ASC");
    let mut stmt = conn.prepare(&q)?;
    let p_refs: Vec<&dyn rusqlite::ToSql> = p.iter().map(|x| x.as_ref()).collect();
    let rows = stmt.query_map(rusqlite::params_from_iter(p_refs), |r| { Ok(HistoryEntry { id: r.get(0)?, command: r.get(1)?, cwd: r.get(2)?, namespace: r.get(3)?, project: r.get(4)?, user: r.get(5)?, created_at: r.get(6)? }) })?;
    Ok(rows.filter_map(Result::ok).collect())
}

pub fn delete_history(conn: &Connection, cwd: &str, state: &SessionState) -> Result<()> {
    let mut q = String::from("DELETE FROM history WHERE cwd = ?");
    let mut p: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(cwd.to_string())];
    if let Some(ref n) = state.active_namespace { q.push_str(" AND namespace = ?"); p.push(Box::new(n.to_string())); } else { q.push_str(" AND namespace IS NULL"); }
    if let Some(ref pr) = state.active_project { q.push_str(" AND project = ?"); p.push(Box::new(pr.to_string())); } else { q.push_str(" AND project IS NULL"); }
    if let Some(ref u) = state.active_user { q.push_str(" AND user = ?"); p.push(Box::new(u.to_string())); } else { q.push_str(" AND user IS NULL"); }
    let p_refs: Vec<&dyn rusqlite::ToSql> = p.iter().map(|x| x.as_ref()).collect();
    conn.execute(&q, rusqlite::params_from_iter(p_refs))?;
    Ok(())
}

pub fn delete_all_history(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM history", [])?;
    Ok(())
}
