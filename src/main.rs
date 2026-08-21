mod business;
mod constants;
mod db;
mod logic;
mod repositories;
mod ui;

use db::seeder::load_bible_structure;
use logic::buffer::Buffer;

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::db::seeder::load_traduction;
use crate::repositories::buffer_sqlite_repository::BufferSqliteRepository;
use crate::repositories::verse_text_sqlite_repository::VerseTextSqliteRepository;

const WINDOW_FLAG: &str = "--window";
const PROJECTOR_FLAG: &str = "--projector";

fn main() {
    // El proyector se atiende antes que nada: es un proceso hijo que solo
    // dibuja lo que le mandan por la tuberia. No abre la base ni siembra nada,
    // y su stdout pertenece al protocolo.
    if has_flag(PROJECTOR_FLAG) {
        if let Err(error) = run_projector() {
            eprintln!("Error en el proyector: {}", error);
        }
        return;
    }

    // La siembra imprime por consola, asi que ocurre antes de que cualquiera de
    // las dos interfaces tome la pantalla.
    let _ = load_bible_structure();
    let _ = load_traduction(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("./traducciones/RVR1960-Reina_Valera_1960.csv"),
    );
    let _ = load_traduction(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./traducciones/DHH-Dios_Habla_Hoy.csv"),
    );

    let Some(buffer_repo) = BufferSqliteRepository::new() else {
        eprintln!("No se pudo abrir la base de datos para el historico");
        return;
    };

    let Some(verse_repo) = VerseTextSqliteRepository::new() else {
        eprintln!("No se pudo abrir la base de datos de versos");
        return;
    };

    let mut buffer = Buffer::new();

    if let Err(error) = buffer.load_history(&buffer_repo) {
        println!(
            "No se pudo cargar el historico: {}\r\nContinuando...",
            error
        );
        thread::sleep(Duration::from_secs(5));
    }

    let window_mode = has_flag(WINDOW_FLAG);

    let result = if window_mode {
        run_window(&mut buffer, &verse_repo, &buffer_repo)
    } else {
        ui::tui::run(&mut buffer, &verse_repo, &buffer_repo)
    };

    if let Err(error) = result {
        eprintln!("Error en la interfaz: {}", error);
    }
}

#[cfg(feature = "gui")]
fn run_window(
    buffer: &mut Buffer,
    verses: &VerseTextSqliteRepository,
    history: &BufferSqliteRepository,
) -> std::io::Result<()> {
    ui::gui::run(buffer, verses, history)
}

#[cfg(not(feature = "gui"))]
fn run_window(
    _buffer: &mut Buffer,
    _verses: &VerseTextSqliteRepository,
    _history: &BufferSqliteRepository,
) -> std::io::Result<()> {
    Err(sin_ventana(WINDOW_FLAG))
}

#[cfg(feature = "gui")]
fn run_projector() -> std::io::Result<()> {
    ui::projector::run()
}

#[cfg(not(feature = "gui"))]
fn run_projector() -> std::io::Result<()> {
    Err(sin_ventana(PROJECTOR_FLAG))
}

#[cfg(not(feature = "gui"))]
fn sin_ventana(flag: &str) -> std::io::Error {
    std::io::Error::other(format!(
        "esta copia se compilo sin soporte de ventana; recompila con: \
         cargo run --features gui -- {}",
        flag
    ))
}

/// Los argumentos se filtran aca y no llegan nunca al analizador de citas: el
/// patron de traduccion es `--\w+` y se tragaria `--window` como si fuera un
/// codigo de version.
fn has_flag(flag: &str) -> bool {
    std::env::args().skip(1).any(|argument| argument == flag)
}
