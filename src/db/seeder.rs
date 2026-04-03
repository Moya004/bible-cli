use rusqlite::{Connection, Result};

use std::path::PathBuf;

const TABLES_SCHEMA: &str = include_str!("./schemas/tables.sql");

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn load_bible_structure() -> Result<()> {
    let conn = Connection::open(project_root().join("./database.sqlite3"))?;

    match conn.execute("SELECT DISTINCT b.book FROM bible b;", []) {
        Ok(_) => Ok(()),

        Err(_) => conn.execute_batch(TABLES_SCHEMA),
    }
}
