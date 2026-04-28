use crate::constants::types::{Book, IndexVariation};
use regex::Regex;

pub fn get_verses(buffer: &String) -> Vec<Book> {
    let mut matches: Vec<Book> = Vec::new();
    for input in buffer.split(";") {
        let parsed = Book::from_string(input);
        if let Some(matched_book) = parsed {
            matches.push(matched_book);
        }
    }
    return matches;
}

pub fn get_chapter_and_verse(input: &str) -> Option<(Vec<u16>, Vec<u16>)> {
    let single = r"\d+";
    let list = format!(r"{single}-{single}");
    let atom = format!(r"(?:{single}|{list})");
    let group = format!(r"{atom}(?:,{atom})*");
    let complete = format!(r"(?:{group}:{group})");
    let collection = format!(r"{complete}(?:\.{complete})*");

    let reColl = Regex::new(&collection).unwrap();
    let reCom = Regex::new(&complete).unwrap();

    if reColl.is_match(input) {
        for complete_item in input.split(".") {
            if reCom.is_match(complete_item) {
                let to_search: Vec<&str> = complete_item.split(":").collect();
                println!(
                    "{:?}:{:?}",
                    get_index_variations(to_search[0]),
                    get_index_variations(to_search[1])
                );
            }
        }
    }

    None
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
