use crate::business::domain::{Cite, Query};

pub trait VerseRepository {
    fn get_text(&self, query: Query) -> Cite;
}
