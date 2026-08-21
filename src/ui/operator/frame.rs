use crate::ui::operator::action::Panel;
use crate::ui::operator::state::{OperatorState, Scrolls};

pub struct VerseRow<'a> {
    pub number: String,
    pub text: &'a str,
}

/// Arena de cadenas de un fotograma del operador.
///
/// Igual que en el modo consulta: Clay guarda punteros al texto y no copias,
/// asi que todo lo que se dibuja se arma antes de maquetar. El texto de los
/// versos y las citas de la cola se prestan del estado en vez de copiarse.
pub struct Frame<'a> {
    pub books: &'a [&'static str],
    pub book: usize,
    pub chapters: Vec<String>,
    pub chapter: usize,
    pub verses: Vec<VerseRow<'a>>,
    pub verse: usize,
    pub queue: Vec<&'a str>,
    pub queue_index: usize,

    pub focus: Panel,
    pub in_query: bool,
    pub blank: bool,
    pub live: &'a str,
    pub status: &'a str,
    pub query: &'a str,
    pub query_h_scroll: f32,
    pub scroll: Scrolls,
}

impl<'a> Frame<'a> {
    pub fn build(state: &'a OperatorState) -> Self {
        Self {
            books: &state.books,
            book: state.book,
            chapters: state.chapters.iter().map(u8::to_string).collect(),
            chapter: state.chapter,
            verses: state
                .verses
                .iter()
                .map(|cite| VerseRow {
                    number: cite.verse.to_string(),
                    text: cite.text.trim(),
                })
                .collect(),
            verse: state.verse,
            queue: state
                .queue
                .iter()
                .map(|selection| selection.cite.as_str())
                .collect(),
            queue_index: state.queue_index,

            focus: state.focus,
            in_query: state.in_query,
            blank: state.blank,
            live: state
                .live
                .as_ref()
                .map(|selection| selection.cite.as_str())
                .unwrap_or("nada en pantalla"),
            status: &state.status,
            query: &state.query,
            query_h_scroll: state.query_h_scroll,
            scroll: state.scroll,
        }
    }
}
