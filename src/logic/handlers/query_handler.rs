use std::fmt::{self, Display};

use crate::constants::types::{Book, IndexVariation};

#[derive(Debug, Clone)]
pub struct Query {
    pub book: Book,
    pub chapters: IndexVariation,
    pub verses: IndexVariation,
    pub translation: String,
}

impl Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Libro: {}\r\nCapitulo(s): {}\r\nVersiculo(s): {}\r\nTraduccion: {}\r\n",
            self.book, self.chapters, self.verses, self.translation
        )
    }
}
