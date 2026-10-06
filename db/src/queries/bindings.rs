use rusqlite::{params, Connection, Result};

pub fn insert_binding(conn: &Connection, target: &str, bind_type: &str, value: &str) -> Result<()> {
    // جلوگیری از خطای Foreign Key با ساخت اتوماتیک رفرنس‌ها
    if bind_type == "project" {
        let _ = conn.execute("INSERT OR IGNORE INTO namespaces (name) VALUES ('default')", []);
        let _ = conn.execute("INSERT OR IGNORE INTO projects (name, namespace_name) VALUES (?1, 'default')", params![value]);
    } else if bind_type == "user" {
        let _ = conn.execute("INSERT OR IGNORE INTO users (username, password) VALUES (?1, 'local')", params![value]);
    }
    
    conn.execute("INSERT OR REPLACE INTO bindings (target, bind_type, value) VALUES (?1, ?2, ?3)", params![target, bind_type, value])?;
    Ok(())
}

pub fn delete_binding(conn: &Connection, target: &str) -> Result<()> {
    conn.execute("DELETE FROM bindings WHERE target = ?1", params![target])?;
    Ok(())
}

pub fn get_binding(conn: &Connection, target: &str) -> Option<(String, String)> {
    conn.query_row("SELECT bind_type, value FROM bindings WHERE target = ?1", params![target], |r| Ok((r.get(0)?, r.get(1)?))).ok()
}
