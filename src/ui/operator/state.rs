use clay_layout::color::Color;

use crate::business::domain::{Book, Cite};
use crate::business::repositories::VerseRepository;
use crate::constants::cons::BOOKS;
use crate::logic::buffer::Buffer;
use crate::logic::parser::get_queries;
use crate::ui::design::theme::section_color;
use crate::ui::operator::action::{Action, Panel};
use crate::ui::projector::protocol::Command;

/// Datos fijos de un libro para la rejilla.
///
/// Se arman una sola vez: no cambian nunca, y rehacerlos por fotograma seria
/// gastar por gusto.
pub struct BookEntry {
    pub name: &'static str,
    pub abbreviation: &'static str,
    pub accent: Color,
}

/// Celdas por fila de cada rejilla, mientras no se hayan medido los paneles.
pub const BOOK_COLUMNS: usize = 6;
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

    pub books: Vec<BookEntry>,
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

    /// Columnas de cada rejilla, recalculadas con el ancho real del panel.
    /// Las necesita tambien `columns()`: una flecha arriba tiene que saltar
    /// exactamente una fila de las que se dibujan.
    pub book_columns: usize,
    pub chapter_columns: usize,

    /// Ultima seleccion a la que se corrio la vista de cada panel.
    ///
    /// Solo se persigue la seleccion cuando esta *cambia*. Haciendolo en cada
    /// fotograma, la vista volveria sola al elemento elegido y no se podria
    /// mirar el resto de la lista con la rueda.
    pub followed: [Option<usize>; 4],
}

impl OperatorState {
    pub fn new<V: VerseRepository>(verses: &V) -> Self {
        let mut state = Self {
            focus: Panel::Books,
            in_query: false,
            books: BOOKS
                .iter()
                .filter_map(|(name, abbreviation, _)| {
                    let book = Book::from_string(name)?;
                    Some(BookEntry {
                        name,
                        abbreviation,
                        accent: section_color(book.section()),
                    })
                })
                .collect(),
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
            book_columns: BOOK_COLUMNS,
            chapter_columns: CHAPTER_COLUMNS,
            followed: [None; 4],
        };

        state.load_chapters(verses);
        state
    }

    /// Aplica una intencion y devuelve lo que haya que mandarle al proyector.
    pub fn dispatch<V: VerseRepository>(
        &mut self,
        action: Action,
        verses: &V,
        buffer: &mut Buffer,
    ) -> Vec<Command> {
        match action {
            Action::FocusNext => self.focus = self.focus.next(),
            Action::FocusPrevious => self.focus = self.focus.previous(),

            Action::Up => self.move_selection(verses, -(self.columns() as isize)),
            Action::Down => self.move_selection(verses, self.columns() as isize),
            Action::Left => self.move_selection(verses, -1),
            Action::Right => self.move_selection(verses, 1),

            Action::Send => return self.send_selection(),
            Action::Enqueue => self.enqueue_selection(),

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
            Action::Delete => {
                if self.query_caret < self.query_len() {
                    let at = self.caret_byte();
                    self.query.remove(at);
                }
            }
            Action::CaretLeft => self.query_caret = self.query_caret.saturating_sub(1),
            Action::CaretRight => {
                self.query_caret = (self.query_caret + 1).min(self.query_len())
            }
            Action::CaretStart => self.query_caret = 0,
            Action::CaretEnd => self.query_caret = self.query_len(),

            Action::SubmitQuery => self.submit_query(verses, buffer),
            Action::HistoryPrevious => self.recall(buffer, true),
            Action::HistoryNext => self.recall(buffer, false),

            Action::Click { panel, index } => {
                self.focus = panel;
                self.in_query = false;

                if let Some(index) = index {
                    self.select(verses, panel, index);
                }
            }

            Action::Activate { panel, index } => {
                self.focus = panel;
                self.select(verses, panel, index);

                if matches!(panel, Panel::Verses | Panel::Queue) {
                    return self.send_selection();
                }
            }

            Action::Scroll { panel, delta } => {
                *self.scroll_of(panel) += delta;
            }

            Action::Quit => {}
        }

        Vec::new()
    }

    /// Cuantas celdas por fila tiene el panel con foco. En una lista, una: asi
    /// arriba y abajo se mueven de a un elemento sin casos especiales.
    fn columns(&self) -> usize {
        match self.focus {
            Panel::Books => self.book_columns,
            Panel::Chapters => self.chapter_columns,
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

    /// Elige un elemento de un panel, arrastrando lo que dependa de el.
    ///
    /// Es lo mismo que hace una flecha al moverse, para que raton y teclado no
    /// puedan dejar el estado de formas distintas.
    fn select<V: VerseRepository>(&mut self, verses: &V, panel: Panel, index: usize) {
        match panel {
            Panel::Books if index < self.books.len() => {
                self.book = index;
                self.load_chapters(verses);
            }
            Panel::Chapters if index < self.chapters.len() => {
                self.chapter = index;
                self.load_verses(verses);
            }
            Panel::Verses if index < self.verses.len() => self.verse = index,
            Panel::Queue if index < self.queue.len() => self.queue_index = index,
            _ => {}
        }
    }

    fn scroll_of(&mut self, panel: Panel) -> &mut f32 {
        match panel {
            Panel::Books => &mut self.scroll.books,
            Panel::Chapters => &mut self.scroll.chapters,
            Panel::Verses => &mut self.scroll.verses,
            Panel::Queue => &mut self.scroll.queue,
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

    fn enqueue_selection(&mut self) {
        let Some(selection) = self.selection() else {
            return;
        };

        self.status = format!("Encolado: {}", selection.cite);
        self.queue.push(selection);
    }

    /// Encola un verso concreto, sin tocar el estado ni el aviso.
    ///
    /// Lo usa el envio de una consulta, que encola de golpe todo lo que la cita
    /// abarque y resume al final en vez de avisar verso por verso.
    fn enqueue(&mut self, cite: &Cite) {
        self.queue.push(Selection::from_cite(cite));
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

    /// Encola todo lo que abarque la cita escrita en la barra de consulta.
    ///
    /// Reutiliza el mismo analizador que la interfaz de terminal, asi que acepta
    /// las mismas abreviaturas y varias citas de una vez: `jn 3:16; salmos 23:1-3`.
    ///
    /// Al terminar, la seleccion queda en el ultimo verso encontrado: es el que
    /// uno acaba de pedir, y deja los paneles abiertos por donde se estaba
    /// trabajando en vez de donde estaban antes de escribir.
    fn submit_query<V: VerseRepository>(&mut self, verses: &V, buffer: &mut Buffer) {
        // La linea entra al historico antes de interpretarse, para poder
        // recuperarla con las flechas aunque no se reconociera ninguna cita —
        // que es justo cuando uno quiere recuperarla y corregirla.
        let line = buffer.read_line(std::mem::take(&mut self.query));

        self.in_query = false;
        self.query_caret = 0;
        self.query_h_scroll = 0.;

        let queries = get_queries(&line);

        if queries.is_empty() {
            self.status = format!("No se reconocio ninguna cita en «{}»", line.trim());
            return;
        }

        let mut enqueued = 0;
        let mut last: Option<Cite> = None;
        let mut failed: Option<String> = None;

        for query in &queries {
            match verses.get_text(query) {
                Ok(passage) => {
                    for cite in passage.0 {
                        self.enqueue(&cite);
                        enqueued += 1;
                        last = Some(cite);
                    }
                }
                Err(error) => failed = Some(error.to_string()),
            }
        }

        if let Some(cite) = last {
            self.select_cite(verses, &cite);
            self.focus = Panel::Verses;
        }

        self.status = match (failed, enqueued) {
            (Some(error), _) => format!("Error consultando: {}", error),
            (None, 0) => format!("Sin resultados para «{}»", line.trim()),
            (None, count) => format!("{} verso(s) encolado(s)", count),
        };
    }

    /// Lleva los paneles hasta un verso concreto.
    ///
    /// El orden importa: elegir libro recarga capitulos, y elegir capitulo
    /// recarga versos poniendo el indice en cero. Hay que ir de arriba abajo o
    /// el ultimo paso se pierde.
    fn select_cite<V: VerseRepository>(&mut self, verses: &V, cite: &Cite) {
        if let Some(position) = self
            .books
            .iter()
            .position(|entry| entry.name == cite.book.as_string())
        {
            self.book = position;
            self.load_chapters(verses);
        }

        if let Some(position) = self.chapters.iter().position(|c| *c == cite.chapter) {
            self.chapter = position;
            self.load_verses(verses);
        }

        if let Some(position) = self.verses.iter().position(|c| c.verse == cite.verse) {
            self.verse = position;
        }
    }

    /// Recorre el historico de consultas con las flechas, mientras se escribe.
    fn recall(&mut self, buffer: &mut Buffer, backwards: bool) {
        if backwards {
            buffer.load_prev_entry(&mut self.query);
        } else {
            buffer.load_next_entry(&mut self.query);
        }

        self.query_caret = self.query.chars().count();
    }

    pub fn current_book(&self) -> Option<Book> {
        Book::from_string(self.books.get(self.book)?.name)
    }

    pub fn current_chapter(&self) -> Option<u8> {
        self.chapters.get(self.chapter).copied()
    }

    pub fn caret_prefix(&self) -> &str {
        &self.query[..self.caret_byte()]
    }

    fn query_len(&self) -> usize {
        self.query.chars().count()
    }

    fn caret_byte(&self) -> usize {
        self.query
            .char_indices()
            .nth(self.query_caret)
            .map(|(index, _)| index)
            .unwrap_or(self.query.len())
    }
}
