use std::fmt::{self, Display};
use std::ops::RangeInclusive;

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
