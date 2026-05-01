use crate::constants::types::{Book, IndexVariation, Query};
use regex::Regex;

#[derive(Debug)]
pub struct ChaptersAndVerses {
    chapters: Vec<Vec<IndexVariation>>,
    verses: Vec<Vec<IndexVariation>>,
}

pub fn get_cites(buffer: &String) -> Vec<Query> {
    let mut matches: Vec<Query> = Vec::new();
    for input in buffer.split(";") {
        let splited_input: Vec<&str> = input.split(" ").collect();
        let book_to_parse = Book::from_string(splited_input[0]);
        let body_to_parse = splited_input[1..splited_input.len() - 1].join("");
        let translation_to_parse = splited_input[splited_input.len() - 1];

        if let Some(matched_book) = book_to_parse {
            let indices = get_chapter_and_verse(&body_to_parse);
            let translation = get_translation(translation_to_parse);

            if indices.chapters.len() > 0 && indices.verses.len() > 0 {
                let mut cites = build_cites(matched_book, indices, translation);
                matches.append(&mut cites);
            }
        }
    }
    matches
}

pub fn build_cites(book: Book, indices: ChaptersAndVerses, translation: String) -> Vec<Query> {
    let mut to_return: Vec<Query> = Vec::new();
    for (chapters_group, verses_group) in indices.chapters.iter().zip(indices.verses.iter()) {
        for chapters in chapters_group {
            for verses in verses_group {
                to_return.push(Query {
                    book: book,
                    chapter: chapters.clone(),
                    verse: verses.clone(),
                    translation: translation.clone(),
                })
            }
        }
    }
    to_return
}

pub fn get_translation(input: &str) -> String {
    String::new()
}

pub fn get_chapter_and_verse(input: &str) -> ChaptersAndVerses {
    let single = r"\d+";
    let list = format!(r"{single}-{single}");
    let atom = format!(r"(?:{single}|{list})");
    let group = format!(r"{atom}(?:,{atom})*");
    let complete = format!(r"(?:{group}:{group})");
    let collection = format!(r"{complete}(?:\.{complete})*");

    let re_coll = Regex::new(&collection).unwrap();
    let re_com = Regex::new(&complete).unwrap();

    let mut to_return: ChaptersAndVerses = ChaptersAndVerses {
        chapters: Vec::new(),
        verses: Vec::new(),
    };

    if re_coll.is_match(input) {
        for complete_item in input.split(".") {
            if re_com.is_match(complete_item) {
                let to_search: Vec<&str> = complete_item.split(":").collect();

                to_return
                    .chapters
                    .push(merge_indicies(get_index_variations(to_search[0])));
                to_return
                    .verses
                    .push(merge_indicies(get_index_variations(to_search[1])));
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

    let re_group = Regex::new(&group).unwrap();
    let re_list = Regex::new(&list).unwrap();
    let re_single = Regex::new(&single).unwrap();

    let mut to_return: Vec<IndexVariation> = Vec::new();

    if re_group.is_match(input) {
        for group in input.split(",") {
            if re_list.is_match(group) {
                let low_high: Vec<u8> = group
                    .split("-")
                    .into_iter()
                    .map(|num| num.parse::<u8>().expect("No valid u8 number"))
                    .collect();
                to_return.push(IndexVariation::List(low_high[0]..=low_high[1]));
            } else if re_single.is_match(group) {
                to_return.push(IndexVariation::Single(
                    group.parse::<u8>().expect("No valid u8 number"),
                ));
            }
        }
    }

    to_return
}

pub fn merge_indicies(indices: Vec<IndexVariation>) -> Vec<IndexVariation> {
    // Normalizar todo a (start, end) y ordenar
    let mut ranges: Vec<(u8, u8)> = indices
        .iter()
        .map(|i| match i {
            IndexVariation::Single(n) => (*n, *n),
            IndexVariation::List(r) => (*r.start(), *r.end()),
        })
        .collect();

    ranges.sort_by_key(|r| (r.0, r.1));

    // Fusionar
    let mut merged: Vec<(u8, u8)> = Vec::new();

    for (start, end) in ranges {
        match merged.last_mut() {
            None => merged.push((start, end)),
            Some(last) => {
                if start <= last.1 + 1 {
                    // Consecutivo, solapado o duplicado → extender si corresponde
                    if end > last.1 {
                        last.1 = end;
                    }
                } else {
                    merged.push((start, end));
                }
            }
        }
    }

    // Convertir de vuelta a IndexVariation
    merged
        .into_iter()
        .map(|(start, end)| {
            if start == end {
                IndexVariation::Single(start)
            } else {
                IndexVariation::List(start..=end)
            }
        })
        .collect()
}
