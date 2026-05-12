use std::io::{Error, Stdout};

use crate::business::repositories::BufferRepository;
use crate::logic::buffer::Buffer;
use std::fmt::Write as WriteFmt;
use std::io::{Write, stdin, stdout};
use std::process::exit;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, terminal_size};

fn get_cursor(output: &mut RawTerminal<Stdout>) -> (u16, u16) {
    output.cursor_pos().unwrap_or((4, 1))
}

pub fn main_controls<T: BufferRepository>(
    buffer: &mut Buffer,
    bufferRepo: &T,
) -> Result<String, Error> {
    let input = stdin();
    let mut content = String::new();
    print!("\x1B[2J\x1B[1;1H");
    let mut output = stdout().into_raw_mode().unwrap();
    output.flush().expect("Error vaciando el buffer");
    match write!(output, "📔>") {
        Ok(text) => text,
        Err(_) => (),
    }

    let (_, cursor_y) = get_cursor(&mut output);
    for key in input.keys() {
        match key.as_ref().unwrap() {
            Key::Left => {
                let curr_coords = get_cursor(&mut output);
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
                let curr_coords = get_cursor(&mut output);
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
                if get_cursor(&mut output).0 > 4 {
                    write!(output, "{}", cursor::Left(1)).unwrap();
                    write!(output, " ").unwrap();
                    write!(output, "{}", cursor::Left(1)).unwrap();
                    content.pop();
                }
            }
            Key::Ctrl('c') => {
                output.suspend_raw_mode().unwrap();
                write!(output, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
                match buffer.save_history(bufferRepo) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("Error al guardar historico: {}", error);
                    }
                };
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

    let input_to_process = buffer.read_line(String::from(content));
    Ok(input_to_process)
}

pub fn continue_controls() -> Result<(), Error> {
    let input = stdin();
    print!("\r\n\nPresione 'Enter' para continuar...");
    let mut output = stdout().into_raw_mode().unwrap();
    output.flush().expect("Error vaciando el buffer");
    for key in input.keys() {
        match key.as_ref().unwrap() {
            Key::Char('\n') => break,
            _ => continue,
        }
    }
    Ok(())
}
