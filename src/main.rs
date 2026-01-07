use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();
    loop {
        print!("📔>");

        io::stdout().flush().expect("Error vaciando buffer");

        io::stdin()
            .read_line(&mut buffer)
            .expect("Error leyendo versiculos.");

        let mut buffer: String = match buffer.trim().parse() {
            Ok(text) => text,
            Err(_) => continue,
        };

        println!("{}", buffer);

        buffer.clear();
    }
}
