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

const TUI_FLAG: &str = "--tui";
const PROJECTOR_FLAG: &str = "--projector";

/// Lo unico que el programa entiende. Sin argumentos arranca el operador, que es
/// el modo normal de uso.
const FLAGS: [&str; 2] = [TUI_FLAG, PROJECTOR_FLAG];

fn main() {
    // Antes que nada, porque `has_flag` compara exacto y un argumento que no
    // reconoce se perderia en silencio: `cargo run -- --features gui` arrancaba
    // el modo equivocado sin decir nada, con la feature apagada de propina.
    if let Some(unknown) = unknown_argument() {
        eprintln!("Argumento desconocido: {}", unknown);
        eprintln!("Uso: biblia-cli [{}]", FLAGS.join(" | "));
        eprintln!("  (sin argumentos)  ventana de operador; lanza el proyector");
        eprintln!("  {}             interfaz de terminal", TUI_FLAG);
        eprintln!("  {}       proceso de proyeccion; lo lanza el operador", PROJECTOR_FLAG);
        return;
    }

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

    let result = if has_flag(TUI_FLAG) {
        ui::tui::run(&mut buffer, &verse_repo, &buffer_repo)
    } else {
        run_operator(&mut buffer, &verse_repo, &buffer_repo)
    };

    if let Err(error) = result {
        eprintln!("Error en la interfaz: {}", error);
    }
}

#[cfg(feature = "gui")]
fn run_operator(
    buffer: &mut Buffer,
    verses: &VerseTextSqliteRepository,
    history: &BufferSqliteRepository,
) -> std::io::Result<()> {
    ui::operator::run(verses, buffer, history)
}

/// Sin ventana no hay operador, pero el modo por omision tiene que llevar a
/// alguna parte: una copia compilada con `--no-default-features` cae a la
/// terminal en vez de quedarse sin interfaz.
#[cfg(not(feature = "gui"))]
fn run_operator(
    buffer: &mut Buffer,
    verses: &VerseTextSqliteRepository,
    history: &BufferSqliteRepository,
) -> std::io::Result<()> {
    eprintln!("Esta copia se compilo sin soporte de ventana; abriendo la terminal.");
    ui::tui::run(buffer, verses, history)
}

#[cfg(feature = "gui")]
fn run_projector() -> std::io::Result<()> {
    ui::projector::run()
}

/// El proyector no tiene equivalente en terminal: es una ventana o no es nada.
#[cfg(not(feature = "gui"))]
fn run_projector() -> std::io::Result<()> {
    Err(std::io::Error::other(
        "esta copia se compilo sin soporte de ventana; recompila con `cargo build`, \
         que ya activa la feature `gui`",
    ))
}

/// Los argumentos se filtran aca y no llegan nunca al analizador de citas: el
/// patron de traduccion es `--\w+` y se tragaria `--tui` como si fuera un codigo
/// de version.
fn has_flag(flag: &str) -> bool {
    std::env::args().skip(1).any(|argument| argument == flag)
}

/// El primer argumento con pinta de opcion que no esta en `FLAGS`. Solo se
/// miran los que empiezan por `-`: el resto no significa nada todavia, pero
/// tampoco es una equivocacion evidente.
fn unknown_argument() -> Option<String> {
    std::env::args()
        .skip(1)
        .find(|argument| argument.starts_with('-') && !FLAGS.contains(&argument.as_str()))
}
