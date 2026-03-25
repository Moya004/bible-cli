use array_deque::StackArrayDeque as SDeque;
use regex::Regex;
use std::fmt::Write as WriteFmt;
use std::io::{Read, Write, stdin, stdout};
use std::process::exit;
use std::thread;
use std::time::Duration;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, terminal_size};

static BOOKS: &[(&str, &str)] = &[
    // Pentateuco
    ("Genesis", r"(?i)\b(Genesis|Gen|Gn)\b"),
    ("Exodo", r"(?i)\b(Exodo|Exod|Ex|Exo)\b"),
    ("Levitico", r"(?i)\b(Levitico|Lev|Lv)\b"),
    ("Numeros", r"(?i)\b(Numeros|Num|Nm)\b"),
    ("Deuteronomio", r"(?i)\b(Deuteronomio|Deut|Dt)\b"),
    // Históricos
    ("Josue", r"(?i)\b(Josue|Jos|Js)\b"),
    ("Jueces", r"(?i)\b(Jueces|Jue|Juec|Jz)\b"),
    ("Rut", r"(?i)\b(Rut|Rt)\b"),
    ("1 Samuel", r"(?i)\b(1\s?Samuel|1\s?Sam|1\s?Sm)\b"),
    ("2 Samuel", r"(?i)\b(2\s?Samuel|2\s?Sam|2\s?Sm)\b"),
    ("1 Reyes", r"(?i)\b(1\s?Reyes|1\s?Rey|1\s?R)\b"),
    ("2 Reyes", r"(?i)\b(2\s?Reyes|2\s?Rey|2\s?R)\b"),
    ("1 Cronicas", r"(?i)\b(1\s?Cronicas|1\s?Cro|1\s?Chr)\b"),
    ("2 Cronicas", r"(?i)\b(2\s?Cronicas|2\s?Cro|2\s?Chr)\b"),
    ("Esdras", r"(?i)\b(Esdras|Esd|Ezr)\b"),
    ("Nehemias", r"(?i)\b(Nehemias|Neh|Ne)\b"),
    ("Ester", r"(?i)\b(Ester|Est)\b"),
    // Poéticos
    ("Job", r"(?i)\b(Job|Jb)\b"),
    ("Salmos", r"(?i)\b(Salmos|Sal|Ps)\b"),
    ("Proverbios", r"(?i)\b(Proverbios|Prov|Pr)\b"),
    ("Eclesiastes", r"(?i)\b(Eclesiastes|Ecl|Qo|Ec)\b"),
    ("Cantares", r"(?i)\b(Cantares|Cant|Ct|Can)\b"),
    // Profetas Mayores
    ("Isaias", r"(?i)\b(Isaias|Isa|Is)\b"),
    ("Jeremias", r"(?i)\b(Jeremias|Jer|Jr)\b"),
    ("Lamentaciones", r"(?i)\b(Lamentaciones|Lam|Lm)\b"),
    ("Ezequiel", r"(?i)\b(Ezequiel|Eze|Ez)\b"),
    ("Daniel", r"(?i)\b(Daniel|Dan|Dn)\b"),
    // Profetas Menores
    ("Oseas", r"(?i)\b(Oseas|Ose|Os)\b"),
    ("Joel", r"(?i)\b(Joel|Jl)\b"),
    ("Amos", r"(?i)\b(Amos|Am)\b"),
    ("Abdias", r"(?i)\b(Abdias|Abd|Ab)\b"),
    ("Jonas", r"(?i)\b(Jonas|Jon)\b"),
    ("Miqueas", r"(?i)\b(Miqueas|Miq|Mi)\b"),
    ("Nahum", r"(?i)\b(Nahum|Nah|Na)\b"),
    ("Habacuc", r"(?i)\b(Habacuc|Hab|Ha)\b"),
    ("Sofonias", r"(?i)\b(Sofonias|Sof|Sf)\b"),
    ("Hageo", r"(?i)\b(Hageo|Hag|Hg)\b"),
    ("Zacarias", r"(?i)\b(Zacarias|Zac|Zc)\b"),
    ("Malaquias", r"(?i)\b(Malaquias|Mal|Ml)\b"),
    // Nuevo Testamento - Evangelios y Hechos
    ("Mateo", r"(?i)\b(Mateo|Mat|Mt)\b"),
    ("Marcos", r"(?i)\b(Marcos|Mar|Mc|Mr)\b"),
    ("Lucas", r"(?i)\b(Lucas|Luc|Lc)\b"),
    ("Juan", r"(?i)\b(Juan|Jn)\b"),
    ("Hechos", r"(?i)\b(Hechos|Hch|Ac)\b"),
    // Cartas Paulinas
    ("Romanos", r"(?i)\b(Romanos|Rom|Ro)\b"),
    ("1 Corintios", r"(?i)\b(1\s?Corintios|1\s?Cor|1\s?Co)\b"),
    ("2 Corintios", r"(?i)\b(2\s?Corintios|2\s?Cor|2\s?Co)\b"),
    ("Galatas", r"(?i)\b(Galatas|Gal|Gl)\b"),
    ("Efesios", r"(?i)\b(Efesios|Efe|Ef)\b"),
    ("Filipenses", r"(?i)\b(Filipenses|Fil|Flp)\b"),
    ("Colosenses", r"(?i)\b(Colosenses|Col|Cl)\b"),
    (
        "1 Tesalonicenses",
        r"(?i)\b(1\s?Tesalonicenses|1\s?Tes|1\s?Ts)\b",
    ),
    (
        "2 Tesalonicenses",
        r"(?i)\b(2\s?Tesalonicenses|2\s?Tes|2\s?Ts)\b",
    ),
    ("1 Timoteo", r"(?i)\b(1\s?Timoteo|1\s?Tim|1\s?Tm)\b"),
    ("2 Timoteo", r"(?i)\b(2\s?Timoteo|2\s?Tim|2\s?Tm)\b"),
    ("Tito", r"(?i)\b(Tito|Tit|Tt)\b"),
    ("Filemon", r"(?i)\b(Filemon|Filem|Flm)\b"),
    // Otras Cartas y Apocalipsis
    ("Hebreos", r"(?i)\b(Hebreos|Heb|Hb)\b"),
    ("Santiago", r"(?i)\b(Santiago|Sant|Stgo|Jas)\b"),
    ("1 Pedro", r"(?i)\b(1\s?Pedro|1\s?Pe|1\s?P)\b"),
    ("2 Pedro", r"(?i)\b(2\s?Pedro|2\s?Pe|2\s?P)\b"),
    ("1 Juan", r"(?i)\b(1\s?Juan|1\s?Jn)\b"),
    ("2 Juan", r"(?i)\b(2\s?Juan|2\s?Jn)\b"),
    ("3 Juan", r"(?i)\b(3\s?Juan|3\s?Jn)\b"),
    ("Judas", r"(?i)\b(Judas|Jud|Jd)\b"),
    ("Apocalipsis", r"(?i)\b(Apocalipsis|Apoc|Ap|Rev)\b"),
];

fn get_verses(buffer: &String) -> Vec<String> {
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

        println!("{:?}", get_verses(&input));

        thread::sleep(Duration::from_secs(5));
    }
}
