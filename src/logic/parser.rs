use crate::{
    business::domain::{Book, Query},
    constants::{
        cons::{collection_regex, list_regex, single_regex, translation_flag_regex},
        types::IndexVariation,
    },
};

#[derive(Debug)]
pub struct ChaptersAndVerses {
    chapters: Vec<Vec<IndexVariation>>,
    verses: Vec<Vec<IndexVariation>>,
}

pub fn get_queries(buffer: &String) -> Vec<Query> {
    let mut matches: Vec<Query> = Vec::new();
    for input in buffer.split(";") {
        if let Some(matched_book) = Book::from_string(input)
            && let Some(body_to_parse) = collection_regex().find(input)
        {
            let indices = get_chapter_and_verse(&input[body_to_parse.range()]);

            let translation = match translation_flag_regex().find(input) {
                Some(flag) => get_translation(&input[flag.range()]),
                None => String::from("RVR1960"),
            };

            let mut queries = build_cites(matched_book, indices, translation);

            matches.append(&mut queries);
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
                    chapters: chapters.clone(),
                    verses: verses.clone(),
                    translation: translation.clone(),
                })
            }
        }
    }
    to_return
}

//TODO: refactorizar como se descargan y comprueban las traducciones
pub fn get_translation(input: &str) -> String {
    String::new()
}

pub fn get_chapter_and_verse(input: &str) -> ChaptersAndVerses {
    let mut to_return: ChaptersAndVerses = ChaptersAndVerses {
        chapters: Vec::new(),
        verses: Vec::new(),
    };

    for complete_item in input.split(".") {
        let to_search: Vec<&str> = complete_item.split(":").collect();

        to_return
            .chapters
            .push(merge_indicies(get_index_variations(to_search[0])));
        to_return
            .verses
            .push(merge_indicies(get_index_variations(to_search[1])));
    }

    to_return
}

fn get_index_variations(input: &str) -> Vec<IndexVariation> {
    let mut to_return: Vec<IndexVariation> = Vec::new();

    for group in input.split(",") {
        if list_regex().is_match(group) {
            let low_high: Vec<u8> = group
                .split("-")
                .into_iter()
                .map(|num| {
                    let final_num = num.trim();
                    final_num
                        .parse::<u8>()
                        .expect(&format!("No valid u8 number: {final_num}"))
                })
                .collect();
            to_return.push(IndexVariation::List(low_high[0]..=low_high[1]));
        } else if single_regex().is_match(group) {
            let final_num = group.trim();
            to_return.push(IndexVariation::Single(
                final_num
                    .parse::<u8>()
                    .expect(&format!("No valid u8 number: {final_num}")),
            ));
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
