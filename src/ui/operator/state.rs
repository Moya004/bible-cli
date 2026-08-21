use crate::business::domain::{Book, Cite};
use crate::business::repositories::VerseRepository;
use crate::constants::cons::BOOKS;
use crate::constants::types::IndexVariation;
use crate::logic::parser::get_queries;
use crate::ui::operator::action::{Action, Panel};
use crate::ui::projector::protocol::Command;

/// Celdas por fila de cada rejilla. Las usan el estado, para saber que hace una
/// flecha arriba, y la vista, para dibujar las mismas filas.
pub const BOOK_COLUMNS: usize = 3;
pub const CHAPTER_COLUMNS: usize = 6;

pub const DEFAULT_TRANSLATION: &str = "RVR1960";

/// Un verso listo para proyectar.
#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    pub cite: String,
    pub text: String,
}

impl Selection {
    fn from_cite(cite: &Cite) -> Self {
        Self {
            cite: format!(
                "{} {}:{} · {}",
                cite.book, cite.chapter, cite.verse, cite.translation.code
            ),
            text: cite.text.trim().to_string(),
        }
    }
}

/// Desplazamiento de cada panel, en unidades de Clay.
#[derive(Debug, Clone, Copy, Default)]
pub struct Scrolls {
    pub books: f32,
    pub chapters: f32,
    pub verses: f32,
    pub queue: f32,
}

pub struct OperatorState {
    pub focus: Panel,
    /// La barra de consulta tiene el teclado
    pub in_query: bool,

    pub books: Vec<&'static str>,
    pub book: usize,
    pub chapters: Vec<u8>,
    pub chapter: usize,
    pub verses: Vec<Cite>,
    pub verse: usize,

    pub queue: Vec<Selection>,
    pub queue_index: usize,
    /// Lo que esta en la pantalla ahora mismo
    pub live: Option<Selection>,
    pub blank: bool,

    pub translation: String,
    pub status: String,
    pub query: String,
    pub query_caret: usize,
    pub query_h_scroll: f32,
    pub scroll: Scrolls,
}

impl OperatorState {
    pub fn new<V: VerseRepository>(verses: &V) -> Self {
        let mut state = Self {
            focus: Panel::Books,
            in_query: false,
            books: BOOKS.iter().map(|(name, _, _)| *name).collect(),
            book: 0,
            chapters: Vec::new(),
            chapter: 0,
            verses: Vec::new(),
            verse: 0,
            queue: Vec::new(),
            queue_index: 0,
            live: None,
            blank: false,
            translation: String::from(DEFAULT_TRANSLATION),
            status: String::new(),
            query: String::new(),
            query_caret: 0,
            query_h_scroll: 0.,
            scroll: Scrolls::default(),
        };

        state.load_chapters(verses);
        state
    }

    /// Aplica una intencion y devuelve lo que haya que mandarle al proyector.
    pub fn dispatch<V: VerseRepository>(&mut self, action: Action, verses: &V) -> Vec<Command> {
        match action {
            Action::FocusNext => self.focus = self.focus.next(),
            Action::FocusPrevious => self.focus = self.focus.previous(),

            Action::Up => self.move_selection(verses, -(self.columns() as isize)),
            Action::Down => self.move_selection(verses, self.columns() as isize),
            Action::Left => self.move_selection(verses, -1),
            Action::Right => self.move_selection(verses, 1),

            Action::Send => return self.send_selection(),
            Action::Enqueue => self.enqueue(),

            Action::QueueNext => return self.step_queue(1),
            Action::QueuePrevious => return self.step_queue(-1),

            Action::ToggleBlank => {
                self.blank = !self.blank;
                return vec![Command::Blank(self.blank)];
            }

            Action::OpenQuery => {
                self.in_query = true;
                self.status.clear();
            }
            Action::CloseQuery => self.in_query = false,

            Action::InsertChar(character) => {
                let at = self.caret_byte();
                self.query.insert(at, character);
                self.query_caret += 1;
            }
            Action::Backspace => {
                if self.query_caret > 0 {
                    self.query_caret -= 1;
                    let at = self.caret_byte();
                    self.query.remove(at);
                }
            }
            Action::SubmitQuery => self.jump_to_query(verses),

            Action::Quit => {}
        }

        Vec::new()
    }

    /// Cuantas celdas por fila tiene el panel con foco. En una lista, una: asi
    /// arriba y abajo se mueven de a un elemento sin casos especiales.
    fn columns(&self) -> usize {
        match self.focus {
            Panel::Books => BOOK_COLUMNS,
            Panel::Chapters => CHAPTER_COLUMNS,
            Panel::Verses | Panel::Queue => 1,
        }
    }

    fn move_selection<V: VerseRepository>(&mut self, verses: &V, delta: isize) {
        let (index, length) = match self.focus {
            Panel::Books => (&mut self.book, self.books.len()),
            Panel::Chapters => (&mut self.chapter, self.chapters.len()),
            Panel::Verses => (&mut self.verse, self.verses.len()),
            Panel::Queue => (&mut self.queue_index, self.queue.len()),
        };

        if length == 0 {
            return;
        }

        let target = (*index as isize + delta).clamp(0, length as isize - 1) as usize;
        if target == *index {
            return;
        }
        *index = target;

        // Cambiar de libro o de capitulo arrastra lo que hay debajo.
        match self.focus {
            Panel::Books => self.load_chapters(verses),
            Panel::Chapters => self.load_verses(verses),
            _ => {}
        }
    }

    fn load_chapters<V: VerseRepository>(&mut self, verses: &V) {
        let Some(book) = self.current_book() else {
            return;
        };

        self.chapters = verses.chapters(book).unwrap_or_default();
        self.chapter = 0;
        self.scroll.chapters = 0.;
        self.load_verses(verses);
    }

    fn load_verses<V: VerseRepository>(&mut self, repository: &V) {
        let (Some(book), Some(chapter)) = (self.current_book(), self.current_chapter()) else {
            self.verses.clear();
            return;
        };

        self.verses = repository
            .verses(book, chapter, &self.translation)
            .unwrap_or_default();
        self.verse = 0;
        self.scroll.verses = 0.;

        if self.verses.is_empty() {
            self.status = format!("Sin texto para {} {}", book, chapter);
        }
    }

    /// Lo que esta seleccionado ahora, sea en el panel de versos o en la cola.
    pub fn selection(&self) -> Option<Selection> {
        match self.focus {
            Panel::Queue => self.queue.get(self.queue_index).cloned(),
            _ => self.verses.get(self.verse).map(Selection::from_cite),
        }
    }

    fn send_selection(&mut self) -> Vec<Command> {
        let Some(selection) = self.selection() else {
            self.status = String::from("No hay nada seleccionado");
            return Vec::new();
        };

        self.show(selection)
    }

    fn show(&mut self, selection: Selection) -> Vec<Command> {
        let mut commands = vec![Command::Show {
            cite: selection.cite.clone(),
            text: selection.text.clone(),
        }];

        // Enviar con la pantalla oculta no mostraria nada, y el operador
        // quedaria mirando por que no aparece: se destapa sola.
        if self.blank {
            self.blank = false;
            commands.push(Command::Blank(false));
        }

        self.status = format!("En pantalla: {}", selection.cite);
        self.live = Some(selection);
        commands
    }

    fn enqueue(&mut self) {
        let Some(selection) = self.selection() else {
            return;
        };

        self.status = format!("Encolado: {}", selection.cite);
        self.queue.push(selection);
    }

    fn step_queue(&mut self, delta: isize) -> Vec<Command> {
        if self.queue.is_empty() {
            self.status = String::from("La cola esta vacia");
            return Vec::new();
        }

        let target = (self.queue_index as isize + delta).clamp(0, self.queue.len() as isize - 1);
        self.queue_index = target as usize;

        let Some(selection) = self.queue.get(self.queue_index).cloned() else {
            return Vec::new();
        };

        self.show(selection)
    }

    /// Lleva la seleccion a la cita escrita en la barra de consulta.
    ///
    /// Reutiliza el mismo analizador que la interfaz de terminal, asi que acepta
    /// las mismas abreviaturas: `jn 3:16`, `salmos 23:1`.
    fn jump_to_query<V: VerseRepository>(&mut self, verses: &V) {
        let queries = get_queries(&self.query);

        let Some(query) = queries.first() else {
            self.status = format!("No se reconocio la cita «{}»", self.query.trim());
            return;
        };

        let chapter = first_index(&query.chapters);
        let verse = first_index(&query.verses);

        if let Some(position) = self.books.iter().position(|name| *name == query.book.as_string())
        {
            self.book = position;
            self.load_chapters(verses);
        }

        if let Some(position) = self.chapters.iter().position(|c| *c == chapter) {
            self.chapter = position;
            self.load_verses(verses);
        }

        if let Some(position) = self.verses.iter().position(|c| c.verse == verse) {
            self.verse = position;
        }

        self.in_query = false;
        self.focus = Panel::Verses;
        self.query.clear();
        self.query_caret = 0;
        self.query_h_scroll = 0.;
        self.status = format!("{} {}:{}", query.book, chapter, verse);
    }

    pub fn current_book(&self) -> Option<Book> {
        Book::from_string(self.books.get(self.book)?)
    }

    pub fn current_chapter(&self) -> Option<u8> {
        self.chapters.get(self.chapter).copied()
    }

    pub fn caret_prefix(&self) -> &str {
        &self.query[..self.caret_byte()]
    }

    fn caret_byte(&self) -> usize {
        self.query
            .char_indices()
            .nth(self.query_caret)
            .map(|(index, _)| index)
            .unwrap_or(self.query.len())
    }
}

fn first_index(variation: &IndexVariation) -> u8 {
    match variation {
        IndexVariation::Single(value) => *value,
        IndexVariation::List(range) => *range.start(),
    }
}
