use array_deque::StackArrayDeque as SDeque;
use simple_regex::RegexBuilder;
use std::fmt::Write as WriteFmt;
use std::io::{Write, stdin, stdout};
use std::process::exit;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, terminal_size};

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
                    .string("Levitico")
                    .word_boundary(),
            ),
        )
        .to_regex_or_panic();

    for input in buffer.split(";") {
        println!("{}", input);
        println!("{}", book.is_match(input));
    }
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

    fn load_prev_entry(&mut self) {
        if self.history_pointer > 0 {
            self.history_pointer -= 1;
            self.content = self.history[self.history_pointer].clone();
        }
    }

    fn load_next_entry(&mut self) {
        if self.history_pointer < self.history.len() - 1 {
            self.history_pointer += 1;
            self.content = self.history[self.history_pointer].clone();
        }
    }
    fn read_line(&mut self) -> String {
        let input = stdin();
        print!("\x1B[2J\x1B[1;1H");
        let mut output = stdout().into_raw_mode().unwrap();
        output.flush().expect("Error vaciando el buffer");
        match write!(output, "📔>") {
            Ok(text) => text,
            Err(_) => (),
        }

        let (_, cursor_y) = output.cursor_pos().unwrap();
        for key in input.keys() {
            match key.as_ref().unwrap() {
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
                Key::Right => {
                    let curr_coords = output.cursor_pos().unwrap();
                    let dimensions = terminal_size().unwrap();
                    let (dx, dy) = (
                        ((self.content.chars().count() + 4) % (dimensions.0 as usize)) as u16,
                        ((self.content.chars().count() + 4) / (dimensions.0 as usize)) as u16,
                    );
                    if curr_coords.0 < dx {
                        write!(output, "{}", cursor::Right(1)).unwrap();
                    } else if curr_coords.1 <= dy {
                        if curr_coords.0 == dimensions.0 {
                            write!(output, "{}", cursor::Goto(1, curr_coords.1 + 1)).unwrap();
                        } else {
                            write!(output, "{}", cursor::Right(1)).unwrap();
                        }
                    }
                }
                Key::Up => {
                    self.load_prev_entry();
                    write!(output, "{}📔>{}", cursor::Goto(1, 1), self.content).unwrap();
                }
                Key::Down => {
                    self.load_next_entry();
                    write!(output, "{}📔>{}", cursor::Goto(1, 1), self.content).unwrap();
                }
                Key::End => {
                    let dimensions: (u16, u16) = terminal_size().unwrap();
                    let (dx, dy) = (
                        ((self.content.chars().count() + 4) % (dimensions.0 as usize)) as u16,
                        ((self.content.chars().count() + 4) / (dimensions.0 as usize)) as u16,
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
                    write!(output, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
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

        write!(output, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        output.flush().unwrap();
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
