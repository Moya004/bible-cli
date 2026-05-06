use std::fmt::{self, Display};
use std::ops::RangeInclusive;

use crate::business::domain::Cite;

#[derive(Debug, Clone)]
pub enum IndexVariation {
    Single(u8),
    List(RangeInclusive<u8>),
}

impl Display for IndexVariation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndexVariation::List(r) => write!(f, "{}-{}", r.start(), r.end()),
            IndexVariation::Single(s) => write!(f, "{}", s),
        }
    }
}

pub struct Passage(pub Vec<Cite>);

impl Display for Passage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut verse_output = String::new();
        let mut prev_chap = self.0[0].chapter;

        verse_output += &format!("Capitulo {}:\r\n\n", self.0[0].chapter);

        for c in &self.0 {
            if c.chapter != prev_chap {
                prev_chap = c.chapter;
                verse_output +=
                    &format!("\r\n\nCapitulo {}:\r\n\n{} {}", c.chapter, c.verse, c.text);
            } else {
                verse_output += &format!("{} {}", c.verse, c.text);
            }
        }

        write!(f, "{}\r\n\n{}\r\n", self.0[0].book, verse_output)
    }
}
