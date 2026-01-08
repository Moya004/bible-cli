use array_deque::StackArrayDeque as SDeque;
use simple_regex::RegexBuilder;
use std::io::{self, Write};

fn get_verses(buffer: &String) -> Vec<String> {
    vec![]
}

struct Buffer {
    content: String,
    history: SDeque<String, 10_000>,
    history_pointer: usize,
}

impl Buffer {
    fn new() -> Self {
        Self {
            content: String::new(),
            history: SDeque::new(),
            history_pointer: 0,
        }
    }

    fn save_on_history(&mut self) {
        self.history.push_back(self.content.clone());

        if self.history_pointer <= self.history.len() {
            self.history_pointer = self.history.len();
        }
    }

    fn read_line(&mut self) -> String {
        print!("📔>");

        io::stdout().flush().expect("Error vaciando buffer");

        io::stdin()
            .read_line(&mut self.content)
            .expect("Error leyendo linea");

        self.save_on_history();

        let input_to_return: String = match self.content.trim().parse() {
            Ok(text) => text,
            Err(_) => return self.content.clone(),
        };

        self.content.clear();

        input_to_return
    }
}

fn main() {
    let mut buffer = Buffer::new();
    loop {
        let input = buffer.read_line();

        get_verses(&input);
    }
}
