use array_deque::StackArrayDeque as SDeque;
use std::fmt::{Write as WriteFmt, write};
use std::io::{Write, stdin, stdout};
use std::process::exit;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;

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
        let input = stdin();
        let mut output = stdout().into_raw_mode().unwrap();
        match write!(output, "\r\n📔>") {
            Ok(text) => text,
            Err(_) => (),
        }
        output.flush().expect("Error vaciando el buffer");

        let (_, cursor_y) = output.cursor_pos().unwrap();
        for key in input.keys() {
            write!(output, "").unwrap();
            match key.as_ref().unwrap() {
                Key::Char(';') => {
                    let coords = output.cursor_pos().unwrap();
                    write!(output, "{:?}", coords).unwrap()
                }
                Key::Left => {
                    let curr_coords = output.cursor_pos().unwrap();
                    if curr_coords.1 == cursor_y {
                        if curr_coords.0 > 4 {
                            write!(output, "{}", cursor::Left(1)).unwrap()
                        }
                    } else {
                        if curr_coords.1 > cursor_y {
                            if curr_coords.0 > 1 {
                                write!(output, "{}", cursor::Left(1)).unwrap()
                            } else {
                                write!(
                                    output,
                                    "{}",
                                    cursor::Goto(terminal_size().unwrap().0, curr_coords.1 - 1)
                                )
                                .unwrap();
                            }
                        }
                    }
                }
                Key::Right => write!(output, "{}", cursor::Right(1)).unwrap(),
                Key::Up => write!(output, "YOU PRESSED 'UP'").unwrap(),
                Key::Down => write!(output, "YOU PRESSD DOWN").unwrap(),
                Key::End => {
                    let dimensions: (u16, u16) = terminal_size().unwrap();
                    let (dx, dy) = (
                        ((self.content.len() + 4) % (dimensions.0 as usize)) as u16,
                        ((self.content.len() + 4) / (dimensions.0 as usize)) as u16,
                    );
                    write!(output, "{}", cursor::Goto(dx, cursor_y + dy)).unwrap();
                }
                Key::Backspace => {
                    if output.cursor_pos().unwrap().0 > 4 {
                        write!(output, "{}", cursor::Left(1)).unwrap();
                        write!(output, " ").unwrap();
                        write!(output, "{}", cursor::Left(1)).unwrap();
                        self.content.pop();
                    }
                }
                Key::Ctrl('c') => {
                    output.suspend_raw_mode().unwrap();
                    exit(0);
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    write!(&mut self.content, "{}", c).unwrap();
                    write!(output, "{}", c).unwrap();
                }
                _ => continue,
            }
            output.flush().unwrap();
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
