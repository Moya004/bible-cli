use crate::business::repositories::{BufferRepository, VerseRepository};
use crate::constants::types::Passage;
use crate::logic::buffer::Buffer;
use crate::logic::parser::get_queries;
use crate::ui::action::{Action, Flow};

pub const DEFAULT_TRANSLATION: &str = "RVR1960";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusKind {
    Info,
    Error,
}

pub struct Status {
    pub text: String,
    pub kind: StatusKind,
}

impl Status {
    fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: StatusKind::Info,
        }
    }

    fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: StatusKind::Error,
        }
    }
}

/// Estado de la interfaz, compartido por los dos backends.
///
/// No sabe nada de celdas ni de pixeles: las medidas que guarda (`content_h`,
/// `viewport_h`, `h_scroll`) estan en unidades de Clay y las rellena el backend
/// despues de pintar cada fotograma.
pub struct AppState {
    /// Texto de la barra de entrada
    pub input: String,
    /// Posicion del cursor, en caracteres
    pub caret: usize,
    /// Desplazamiento horizontal de la entrada, en unidades de Clay
    pub h_scroll: f32,
    /// Desplazamiento vertical del panel de resultados
    pub scroll_y: f32,
    /// Alto del contenido del panel, medido en el fotograma anterior
    pub content_h: f32,
    /// Alto visible del panel, medido en el fotograma anterior
    pub viewport_h: f32,
    pub results: Vec<Passage>,
    pub status: Status,
    pub active_translation: String,
    /// El fotograma actual quedo obsoleto y hay que volver a maquetar
    pub dirty: bool,
    /// Error ocurrido al salir, para imprimirlo ya restaurada la terminal
    pub exit_error: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            caret: 0,
            h_scroll: 0.,
            scroll_y: 0.,
            content_h: 0.,
            viewport_h: 0.,
            results: Vec::new(),
            status: Status::info(String::new()),
            active_translation: String::from(DEFAULT_TRANSLATION),
            dirty: true,
            exit_error: None,
        }
    }

    pub fn dispatch<V: VerseRepository, B: BufferRepository>(
        &mut self,
        action: Action,
        buffer: &mut Buffer,
        verses: &V,
        history: &B,
    ) -> Flow {
        self.dirty = true;

        match action {
            Action::InsertChar(c) => {
                let at = self.caret_byte();
                self.input.insert(at, c);
                self.caret += 1;
            }
            Action::Backspace => {
                if self.caret > 0 {
                    self.caret -= 1;
                    let at = self.caret_byte();
                    self.input.remove(at);
                }
            }
            Action::Delete => {
                if self.caret < self.input_len() {
                    let at = self.caret_byte();
                    self.input.remove(at);
                }
            }
            Action::CaretLeft => self.caret = self.caret.saturating_sub(1),
            Action::CaretRight => self.caret = (self.caret + 1).min(self.input_len()),
            Action::CaretStart => self.caret = 0,
            Action::CaretEnd => self.caret = self.input_len(),
            Action::HistoryPrev => {
                buffer.load_prev_entry(&mut self.input);
                self.caret = self.input_len();
            }
            Action::HistoryNext => {
                buffer.load_next_entry(&mut self.input);
                self.caret = self.input_len();
            }
            Action::Scroll(delta) => {
                self.scroll_y += delta;
                self.clamp_scroll();
            }
            Action::Submit => self.submit(buffer, verses),
            Action::Redraw => {}
            Action::Quit => {
                if let Err(error) = buffer.save_history(history) {
                    // No se imprime aqui: seguimos dentro de la pantalla alterna
                    // y el mensaje se perderia. El backend lo muestra despues de
                    // restaurar la terminal.
                    self.exit_error = Some(format!("Error al guardar historico: {}", error));
                }
                return Flow::Quit;
            }
        }

        Flow::Continue
    }

    /// Ejecuta la linea escrita: la guarda en el historico, la interpreta y
    /// consulta cada cita. Reutiliza la misma cadena que usaba el bucle de
    /// `main`: [`Buffer::read_line`] → [`get_queries`] → [`VerseRepository::get_text`].
    fn submit<V: VerseRepository>(&mut self, buffer: &mut Buffer, verses: &V) {
        let line = buffer.read_line(std::mem::take(&mut self.input));

        self.caret = 0;
        self.h_scroll = 0.;
        self.scroll_y = 0.;

        if line.trim().is_empty() {
            self.results.clear();
            self.status = Status::info(String::new());
            return;
        }

        let queries = get_queries(&line);

        if queries.is_empty() {
            self.status = Status::error(format!("No se reconocio ninguna cita en «{}»", line.trim()));
            return;
        }

        // `parser::get_translation` aun devuelve una cadena vacia para el flag
        // `--CODIGO`, asi que solo actualizamos la cabecera si trae algo.
        if let Some(first) = queries.first().filter(|q| !q.translation.is_empty()) {
            self.active_translation = first.translation.clone();
        }

        let mut found = Vec::new();
        let mut empty = 0;
        let mut failed: Option<String> = None;

        for query in &queries {
            match verses.get_text(query) {
                // El repositorio devuelve `Ok` con un pasaje vacio cuando la
                // consulta no encontro versos; `Passage` no se puede mostrar en
                // ese estado, asi que se descarta aqui.
                Ok(passage) if passage.0.is_empty() => empty += 1,
                Ok(passage) => found.push(passage),
                Err(error) => failed = Some(error.to_string()),
            }
        }

        self.results = found;

        self.status = match (failed, self.results.len(), empty) {
            (Some(error), _, _) => Status::error(format!("Error consultando: {}", error)),
            (None, 0, _) => Status::error(format!("Sin resultados para «{}»", line.trim())),
            (None, n, 0) => Status::info(format!("{} pasaje(s)", n)),
            (None, n, e) => Status::info(format!("{} pasaje(s) · {} sin resultados", n, e)),
        };
    }

    /// Ajusta el desplazamiento horizontal para que el cursor quede visible.
    ///
    /// El ancho del texto que precede al cursor lo mide el backend, que es quien
    /// conoce la tipografia. Devuelve `true` si hubo que mover la vista.
    pub fn ensure_caret_visible(&mut self, prefix_width: f32, field_width: f32) -> bool {
        let previous = self.h_scroll;

        if prefix_width - self.h_scroll > field_width {
            self.h_scroll = prefix_width - field_width;
        }
        if prefix_width < self.h_scroll {
            self.h_scroll = prefix_width;
        }
        if self.h_scroll < 0. {
            self.h_scroll = 0.;
        }

        previous != self.h_scroll
    }

    /// Guarda las medidas del panel de resultados del fotograma recien pintado y
    /// vuelve a acotar el desplazamiento con ellas.
    pub fn set_scroll_extent(&mut self, content_height: f32, viewport_height: f32) {
        self.content_h = content_height;
        self.viewport_h = viewport_height;
        self.clamp_scroll();
    }

    fn clamp_scroll(&mut self) {
        let max = (self.content_h - self.viewport_h).max(0.);
        self.scroll_y = self.scroll_y.clamp(0., max);
    }

    /// Texto de la entrada que precede al cursor.
    pub fn caret_prefix(&self) -> &str {
        &self.input[..self.caret_byte()]
    }

    fn caret_byte(&self) -> usize {
        self.input
            .char_indices()
            .nth(self.caret)
            .map(|(index, _)| index)
            .unwrap_or(self.input.len())
    }

    fn input_len(&self) -> usize {
        self.input.chars().count()
    }
}
