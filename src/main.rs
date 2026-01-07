use simple_regex::RegexBuilder;
use std::io::{self, Write};

fn get_verses(buffer: &String) -> Vec<String> {
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
