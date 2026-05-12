use rusqlite::Error;

use crate::{business::domain::Query, constants::types::Passage};
use array_deque::ArrayDeque as Deque;

pub trait VerseRepository {
    fn get_text(&self, query: &Query) -> Result<Passage, Error>;
}

pub trait BufferRepository {
    fn load_history(&self, limit: u16) -> Result<Vec<String>, Error>;
    fn save_history(&self, history: Deque<String>) -> Result<(), Error>;
}
