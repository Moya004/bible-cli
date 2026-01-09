use array_deque::StackArrayDeque as SDeque;
use std::fmt::Write as WriteFmt;
use std::io::{Write, stdin, stdout};
use std::process::exit;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn get_verses(_buffer: &String) -> Vec<String> {
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
        let input_controller = stdin();
        let mut output_controller = stdout().into_raw_mode().unwrap();
        match write!(output_controller, "\r\n📔>") {
            Ok(text) => text,
            Err(_) => (),
        }
        output_controller.flush().expect("Error vaciando el buffer");

        for key in input_controller.keys() {
            write!(output_controller, "").unwrap();
            match key.as_ref().unwrap() {
                Key::Left => write!(output_controller, "{}", cursor::Left(1)).unwrap(),
                Key::Right => write!(output_controller, "{}", cursor::Right(1)).unwrap(),
                Key::Up => write!(output_controller, "YOU PRESSED 'UP'").unwrap(),
                Key::Down => write!(output_controller, "YOU PRESSD DOWN").unwrap(),
                Key::Ctrl('c') => {
                    output_controller.suspend_raw_mode().unwrap();
                    exit(0);
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    write!(&mut self.content, "{}", c).unwrap();
                    print!("{}", c);
                }
                _ => continue,
            }
            output_controller.flush().unwrap();
        }

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
