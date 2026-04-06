use csv::Reader;
use rusqlite::{Connection, Result};

use std::path::{Path, PathBuf};

use crate::constants::cons::compiled_books;

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

pub fn load_traduction<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut reader = Reader::from_path(path).expect("<CSV not found>");
    let compiled = compiled_books();

    for result in reader.records() {
        let record = result.expect("Fallo al cargar registro");

        let book = &record[0];
        let chapter: u16 = record[1].parse().unwrap();
        let verse: u16 = record[2].parse().unwrap();
        let text = &record[3];

        let matched_book = compiled.iter().find(|(_, pattern)| pattern.is_match(book));

        if let Some((cannonical_name, _)) = matched_book {
            println!("{} {}:{} -> {}", cannonical_name, chapter, verse, text);
        }
    }

    Ok(())
}
