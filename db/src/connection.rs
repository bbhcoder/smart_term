use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use crate::queries::schema;

pub fn get_connection() -> Result<Connection> {
    let base_dir = env::var("SMART_BASE_DIR").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
        format!("{}/.smart_dev", home)
    });
    let _ = fs::create_dir_all(&base_dir);
    let db_path = format!("{}/smart_term.db", base_dir);
    let conn = Connection::open(db_path)?;
    schema::init_tables(&conn)?;
    Ok(conn)
}
