use simple_regex::RegexBuilder;
use std::io::{self, Write};

fn get_verses(buffer: &String) -> Vec<String> {
    let book = RegexBuilder::new()
        .group(
            RegexBuilder::new().alternative(
                RegexBuilder::new().alternative(
                    RegexBuilder::new()
                        .word_boundary()
                        .string("Genesis")
                        .word_boundary(),
                    RegexBuilder::new()
                        .word_boundary()
                        .string("Exodo")
                        .word_boundary(),
                ),
                RegexBuilder::new()
                    .word_boundary()
                    .character_class("Levitico")
                    .word_boundary(),
            ),
        )
        .to_regex_or_panic();

    for input in buffer.split(";") {
        println!("{}", book.is_match(input));
    }
    vec![]
}

fn main() {
    let mut buffer = String::new();
    loop {
        print!("📔>");

        io::stdout().flush().expect("Error vaciando buffer");

        io::stdin()
            .read_line(&mut buffer)
            .expect("Error leyendo versiculos.");

        let input_procesado: String = match buffer.trim().parse() {
            Ok(text) => text,
            Err(_) => continue,
        };

        get_verses(&input_procesado);

        buffer.clear();
    }
}
