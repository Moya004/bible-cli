use crate::constants::types::Book;

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
    // let re = Regex::new(r"(\d+):(\d+)").unwrap();

    let range_re = r"(\d+)-(\d+)";
    let single_re = r"(\d+)";
    let list_elem_re = format!(r"{range_re}|{single_re}");
    let list_re = format!(r"{list_elem_re}(,\s*{list_elem_re})*");

    None
}
