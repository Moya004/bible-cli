use crate::constants::types::{Book, Cite, IndexVariation};
use regex::Regex;

pub struct ChaptersAndVerses {
    chapters: Vec<IndexVariation>,
    verses: Vec<IndexVariation>,
}

pub fn get_cites(buffer: &String) -> Option<Vec<Cite>> {
    let mut matches: Vec<Cite> = Vec::new();
    for input in buffer.split(";") {
        let book_to_parse = Book::from_string(input.split(" ").collect::<Vec<&str>>()[0]);
        let body_to_pars = input.split(" ").collect::<Vec<&str>>()[1..].join("");

        if let Some(matched_book) = book_to_parse {
            let indices = get_chapter_and_verse(&body_to_pars);

            if indices.chapters.len() > 0 && indices.verses.len() > 0 {
                if let Some(cite) = build_cites(matched_book, indices) {
                    matches.push(cite);
                }
            }
        }
    }
    None
}

pub fn build_cites(book: Book, indices: ChaptersAndVerses) -> Option<Cite> {
    None
}

pub fn get_chapter_and_verse(input: &str) -> ChaptersAndVerses {
    let single = r"\d+";
    let list = format!(r"{single}-{single}");
    let atom = format!(r"(?:{single}|{list})");
    let group = format!(r"{atom}(?:,{atom})*");
    let complete = format!(r"(?:{group}:{group})");
    let collection = format!(r"{complete}(?:\.{complete})*");

    let reColl = Regex::new(&collection).unwrap();
    let reCom = Regex::new(&complete).unwrap();

    let mut to_return: ChaptersAndVerses = ChaptersAndVerses {
        chapters: Vec::new(),
        verses: Vec::new(),
    };

    if reColl.is_match(input) {
        for complete_item in input.split(".") {
            if reCom.is_match(complete_item) {
                let to_search: Vec<&str> = complete_item.split(":").collect();

                to_return.chapters = get_index_variations(to_search[0]);
                to_return.verses = get_index_variations(to_search[1]);
            }
        }
    }

    to_return
}

fn get_index_variations(input: &str) -> Vec<IndexVariation> {
    let single = r"\d+";
    let list = format!(r"{single}-{single}");
    let atom = format!(r"(?:{single}|{list})");
    let group = format!(r"{atom}(?:,{atom})*");

    let reGroup = Regex::new(&group).unwrap();
    let reList = Regex::new(&list).unwrap();
    let reSingle = Regex::new(&single).unwrap();

    let mut to_return: Vec<IndexVariation> = Vec::new();

    if reGroup.is_match(input) {
        for group in input.split(",") {
            if reList.is_match(group) {
                let low_high: Vec<u8> = group
                    .split("-")
                    .into_iter()
                    .map(|num| num.parse::<u8>().expect("No valid u8 number"))
                    .collect();
                to_return.push(IndexVariation::List(low_high[0]..=low_high[1]));
            } else if reSingle.is_match(group) {
                to_return.push(IndexVariation::Single(
                    group.parse::<u8>().expect("No valid u8 number"),
                ));
            }
        }
    }

    to_return
}
