mod business;
mod constants;
mod db;
mod logic;
mod repositories;
mod ui;

use db::seeder::load_bible_structure;
use logic::{buffer::Buffer, parser::get_queries};

use std::path::PathBuf;

use crate::business::repositories::VerseRepository;
use crate::db::seeder::load_traduction;
use crate::repositories::verse_text_sqlite_repository::VerseTextSqliteRepository;
use crate::ui::main_display::{continue_controls, main_controls};
fn main() {
    let mut buffer = Buffer::new();
    let _ = load_bible_structure();
    let _ = load_traduction(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("./traducciones/RVR1960-Reina_Valera_1960.csv"),
    );
    let _ = load_traduction(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./traducciones/DHH-Dios_Habla_Hoy.csv"),
    );
    loop {
        let input_to_process = main_controls(&mut buffer).unwrap();

        let processed_input = get_queries(&input_to_process);
        // for a in processed_input {
        //     println!("{}", a);
        // }

        // processed_input.iter().for_each(|q| {
        //     println!("{q}");
        // });

        let verse_repo = VerseTextSqliteRepository::new().unwrap();

        processed_input
            .iter()
            .for_each(|q| match verse_repo.get_text(q) {
                Ok(v) => {
                    println!("{v}");
                }
                Err(_) => {
                    return;
                }
            });

        let _ = continue_controls();
    }
}
