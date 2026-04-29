mod constants;
mod db;
mod logic;

use db::seeder::load_bible_structure;
use logic::{buffer::Buffer, parser::get_cites};
use std::fmt::Write as WriteFmt;
use std::io::{Write, stdin, stdout};
use std::path::PathBuf;
use std::process::exit;
use std::thread;
use std::time::Duration;

use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, terminal_size};

use crate::db::seeder::load_traduction;

fn main() {
    let mut buffer = Buffer::new();
    let _ = load_bible_structure();
    let _ = load_traduction(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("./traducciones/RVR1960-Reina_Valera_1960.csv"),
    );
    let _ = load_traduction(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./traducciones/DHH-Dios_Habla_Hoy.csv"),
    );
    loop {
        let input = stdin();
        let mut content = String::new();
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
                        ((content.chars().count() + 4) % (dimensions.0 as usize)) as u16,
                        ((content.chars().count() + 4) / (dimensions.0 as usize)) as u16,
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
                    buffer.load_prev_entry(&mut content);
                    write!(output, "{}", clear::All).unwrap();
                    write!(output, "{}📔>{}", cursor::Goto(1, 1), content).unwrap();
                }
                Key::Down => {
                    buffer.load_next_entry(&mut content);
                    write!(output, "{}", clear::All).unwrap();
                    write!(output, "{}📔>{}", cursor::Goto(1, 1), content).unwrap();
                }
                Key::End => {
                    let dimensions: (u16, u16) = terminal_size().unwrap();
                    let (dx, dy) = (
                        ((content.chars().count() + 4) % (dimensions.0 as usize)) as u16,
                        ((content.chars().count() + 4) / (dimensions.0 as usize)) as u16,
                    );
                    write!(output, "{}", cursor::Goto(dx, cursor_y + dy)).unwrap();
                }
                Key::Backspace => {
                    if output.cursor_pos().unwrap().0 > 4 {
                        write!(output, "{}", cursor::Left(1)).unwrap();
                        write!(output, " ").unwrap();
                        write!(output, "{}", cursor::Left(1)).unwrap();
                        content.pop();
                    }
                }
                Key::Ctrl('c') => {
                    output.suspend_raw_mode().unwrap();
                    write!(output, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                    exit(0);
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    write!(&mut content, "{}", c).unwrap();
                    write!(output, "{}", c).unwrap();
                }
                _ => continue,
            }
            output.flush().unwrap();
        }

        write!(output, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        output.flush().unwrap();

        let input_to_process = buffer.read_line(content);

        let processed_input: Vec<String> = get_cites(&input_to_process)
            .iter()
            .map(|cite| cite.as_string())
            .collect();
        for a in processed_input {
            println!("{}", a);
        }

        thread::sleep(Duration::from_secs(10));
    }
}
