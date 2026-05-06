use rusqlite::Error;

use crate::{business::domain::Query, constants::types::Passage};

pub trait VerseRepository {
    fn get_text(&self, query: &Query) -> Result<Passage, Error>;
}
