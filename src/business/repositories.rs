use rusqlite::Error;

use crate::{
    business::domain::{Book, Cite, Query},
    constants::types::{BufferEntryVariation, Passage},
};
use array_deque::ArrayDeque as Deque;

pub trait VerseRepository {
    fn get_text(&self, query: &Query) -> Result<Passage, Error>;

    /// Capitulos de un libro, del indice canonico. No depende de la traduccion.
    fn chapters(&self, book: Book) -> Result<Vec<u8>, Error>;

    /// Versos de un capitulo, para recorrerlos en pantalla.
    fn verses(&self, book: Book, chapter: u8, translation: &str) -> Result<Vec<Cite>, Error>;
}

pub trait BufferRepository {
    fn load_history(&self, limit: u16) -> Result<Vec<BufferEntryVariation>, Error>;
    fn save_history(&self, history: &Deque<BufferEntryVariation>) -> Result<(), Error>;
}
