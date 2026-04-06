use csv::Reader;
use rusqlite::{Connection, Error, Result};

use std::path::{Path, PathBuf};

use crate::constants::cons::compiled_books;

const TABLES_SCHEMA: &str = include_str!("./schemas/tables.sql");

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn load_bible_structure() -> Result<()> {
    let conn = Connection::open(project_root().join("./database.sqlite3"))?;

    let table_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='biblia'",
        [],
        |row| row.get::<_, i64>(0),
    )? > 0;

    if !table_exists {
        match conn.execute_batch(TABLES_SCHEMA) {
            Ok(()) => {}
            Err(ex) => {
                println!("{}", ex);
                return Err(ex);
            }
        };
    }

    Ok(())
}

pub fn load_traduction<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let file_name = path
        .as_ref()
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unkwon");

    let mut traduccion_iter = file_name.split("-").into_iter();
    let traduccion_code: &str = traduccion_iter.next().unwrap();
    let traduccion_name: &str = traduccion_iter
        .next()
        .unwrap()
        .split(".")
        .into_iter()
        .next()
        .unwrap();

    let conn = Connection::open(project_root().join("./database.sqlite3")).unwrap();

    match conn.execute(
        "INSERT INTO traducciones(codigo, nombre) VALUES (?1, ?2);
        ",
        (traduccion_code, traduccion_name),
    ) {
        Ok(_) => {}
        Err(opss) => {
            println!("{}", opss);
            return Err(opss);
        }
    }

    let mut statement = conn.prepare("SELECT t.id FROM traducciones t WHERE t.codigo = ?1")?;

    let translation_id_iter = statement.query_map([traduccion_code], |row| row.get::<_, u16>(0))?;

    let mut translation_id: u16 = 0;
    for i in translation_id_iter {
        translation_id = i.unwrap();
    }

    let mut reader = Reader::from_path(path).expect("<CSV not found>");
    let compiled = compiled_books();

    for result in reader.records() {
        let record = result.expect("Fallo al cargar registro");

        let book = &record[0];
        let chapter = &record[1];
        let verse = &record[2];
        let text = &record[3];

        let matched_book = compiled.iter().find(|(_, pattern)| pattern.is_match(book));

        if let Some((cannonical_name, _)) = matched_book {
            println!("{} {}:{} -> {}", cannonical_name, chapter, verse, text);

            match conn.execute("INSERT INTO TEXTO_VERSO(libro, capitulo, verso, translation_id, texto) VALUES(?1, ?2, ?3, ?4, ?5);", [book, chapter, verse, &translation_id.to_string(), text]) {
            Ok(_) => {
                println!("{} {}:{} -> insertado con exito ", cannonical_name, chapter, verse)
            },

            Err(opss) => {
            println!("{} {}:{} -> {} ", cannonical_name, chapter, verse, opss);
        },
        }
        }
    }

    Ok(())
}
