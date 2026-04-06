use crate::constants::cons::compiled_books;

pub fn get_verses(buffer: &String) -> Vec<String> {
    let patterns = compiled_books();
    let mut matches: Vec<String> = Vec::new();
    for input in buffer.split(";") {
        for (book, pattern) in patterns.iter() {
            if pattern.is_match(input) {
                matches.push(book.to_string());
            }
        }
    }
    return matches;
}

// pub fn get_book(input: &str) -> Option<Book> {
//     for (book, pattern) in BOOKS {
//         let re = Regex::new(pattern).unwrap();
//         if re.is_match(input) {
//             return Some(book);
//         }
//     }
//     None
// }

pub fn get_chapter_and_verse(input: &str) -> Option<(Vec<u16>, Vec<u16>)> {
    // let re = Regex::new(r"(\d+):(\d+)").unwrap();

    let range_re = r"(\d+)-(\d+)";
    let single_re = r"(\d+)";
    let list_elem_re = format!(r"{range_re}|{single_re}");
    let list_re = format!(r"{list_elem_re}(,\s*{list_elem_re})*");

    None
}
