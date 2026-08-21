use crate::business::domain::Cite;
use crate::constants::types::Passage;
use crate::ui::state::{AppState, StatusKind};

/// Un verso listo para dibujar: la etiqueta ya formateada y una referencia al
/// texto que vive en [`AppState::results`].
pub struct VerseLine<'a> {
    pub number: String,
    pub text: &'a str,
}

/// Una tarjeta de pasaje.
pub struct Card<'a> {
    pub title: String,
    pub verses: Vec<VerseLine<'a>>,
}

/// Arena de cadenas de un fotograma.
///
/// Clay **no copia** el texto: `Clay__OpenTextElement` guarda el puntero y las
/// lineas ajustadas son rebanadas de ese mismo buffer. Por eso todo lo que se
/// dibuja se construye aqui, antes de `Clay::begin`, y vive hasta despues de
/// pintar los comandos. Las vistas de `ui::views` solo reciben `&str`.
pub struct Frame<'a> {
    pub title: &'static str,
    pub translation: &'a str,
    pub hint: &'static str,
    pub status: &'a str,
    pub status_is_error: bool,
    pub cards: Vec<Card<'a>>,
    pub prompt: &'static str,
    pub input: &'a str,
    pub empty_hint: &'static str,
    /// Desplazamiento vertical del panel de resultados, en unidades de Clay
    pub scroll_y: f32,
    /// Desplazamiento horizontal de la entrada, en unidades de Clay
    pub h_scroll: f32,
}

/// Rotulos que dependen de lo que el backend sabe dibujar.
///
/// La terminal muestra emoji sin problema; las fuentes TTF de escritorio no
/// traen `📔` (solo esta en fuentes de color, que raylib no rasteriza), asi que
/// la ventana usa texto plano en vez de un rombo de reemplazo.
pub struct Glyphs {
    pub app_title: &'static str,
    pub prompt: &'static str,
}

impl Glyphs {
    pub const TERMINAL: Self = Self {
        app_title: "📔 biblia-cli",
        prompt: "📔> ",
    };

    pub const WINDOW: Self = Self {
        app_title: "biblia-cli",
        prompt: "> ",
    };
}

impl<'a> Frame<'a> {
    pub fn build(state: &'a AppState, glyphs: &'static Glyphs) -> Self {
        Self {
            title: glyphs.app_title,
            translation: &state.active_translation,
            hint: "^C salir · ↑↓ historial · PgUp/PgDn desplazar",
            status: &state.status.text,
            status_is_error: state.status.kind == StatusKind::Error,
            cards: state.results.iter().map(card_from_passage).collect(),
            prompt: glyphs.prompt,
            input: &state.input,
            empty_hint: "Escribe una cita, por ejemplo:  juan 3:16 ; salmos 23:1-3",
            scroll_y: state.scroll_y,
            h_scroll: state.h_scroll,
        }
    }
}

/// Arma el titulo y las etiquetas de verso de un pasaje.
///
/// Cuando el pasaje abarca varios capitulos la etiqueta de cada verso pasa a ser
/// `capitulo:verso`, para que no quede ambigua.
fn card_from_passage(passage: &Passage) -> Card<'_> {
    let cites: &[Cite] = &passage.0;

    // `Frame` solo se construye con pasajes no vacios: `AppState` descarta los
    // que no devolvieron versos.
    let first = &cites[0];
    let last = &cites[cites.len() - 1];
    let spans_chapters = first.chapter != last.chapter;

    let chapters = if spans_chapters {
        format!("{}-{}", first.chapter, last.chapter)
    } else {
        first.chapter.to_string()
    };

    Card {
        title: format!(
            "{} {} · {}",
            first.book, chapters, first.translation.code
        ),
        verses: cites
            .iter()
            .map(|cite| VerseLine {
                number: if spans_chapters {
                    format!("{}:{}", cite.chapter, cite.verse)
                } else {
                    cite.verse.to_string()
                },
                text: cite.text.trim(),
            })
            .collect(),
    }
}
