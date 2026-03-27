use crate::logic::constants::BOOKS;
use regex::Regex;

pub fn get_verses(buffer: &String) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    for input in buffer.split(";") {
        for (book, pattern) in BOOKS {
            let re = Regex::new(pattern).unwrap();

            if re.is_match(input) {
                matches.push(book.to_string());
            }
        }
    }
    return matches;
}
