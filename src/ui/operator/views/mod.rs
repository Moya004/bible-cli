pub mod bars;
pub mod navigation;
pub mod queue;
pub mod verses;

use clay_layout::{
    grow, id::Id,
    layout::{LayoutDirection, Sizing},
};

use crate::ui::design::components::scroll_area::ScrollIds;
use crate::ui::design::metrics::Metrics;
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::design::{Decl, Scope};
use crate::ui::operator::action::Panel;
use crate::ui::operator::frame::Frame;

/// Etiquetas con las que se derivan los ids de cada elemento. Tienen que ser las
/// mismas al dibujar y al preguntar por el puntero.
pub const BOOKS_ITEM: &str = "books_item";
pub const CHAPTERS_ITEM: &str = "chapters_item";
pub const VERSES_ITEM: &str = "verses_item";
pub const QUEUE_ITEM: &str = "queue_item";

/// Que tiene el puntero encima.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Hit {
    pub panel: Option<Panel>,
    pub item: Option<usize>,
}

/// Todo lo que hay que medir despues de maquetar.
#[derive(Clone, Copy)]
pub struct Anchors {
    pub books: ScrollIds,
    pub chapters: ScrollIds,
    pub verses: ScrollIds,
    pub queue: ScrollIds,
    pub query_field: Id,
    /// Elemento elegido de cada panel, para correr la vista hasta el
    pub selected: [Id; 4],
}

/// El indice bajo el puntero dentro de un panel, o nada si el puntero esta en otro.
pub fn hovered(frame: &Frame<'_>, panel: Panel) -> Option<usize> {
    (frame.hit.panel == Some(panel))
        .then_some(frame.hit.item)
        .flatten()
}

/// Ventana de control, segun el boceto: libros y capitulos a la izquierda, los
/// versos al centro, la cola a la derecha, y abajo el estado y la consulta.
pub fn root<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) -> (Anchors, Hit) {
    let anchors = Anchors {
        books: ScrollIds {
            viewport: c.id("books_viewport"),
            content: c.id("books_content"),
        },
        chapters: ScrollIds {
            viewport: c.id("chapters_viewport"),
            content: c.id("chapters_content"),
        },
        verses: ScrollIds {
            viewport: c.id("verses_viewport"),
            content: c.id("verses_content"),
        },
        queue: ScrollIds {
            viewport: c.id("queue_viewport"),
            content: c.id("queue_content"),
        },
        query_field: c.id("query_field"),
        // El anclaje del elegido es el id del elemento que ocupa ese indice, no
        // un id aparte. Con un id propio, el elegido quedaba fuera del alcance
        // del golpeo — que busca `etiqueta_indice` — y pinchar justo sobre el
        // no registraba nada, que es precisamente lo que uno hace al dar doble
        // click para proyectarlo.
        selected: [
            c.id_index(BOOKS_ITEM, frame.book as u32),
            c.id_index(CHAPTERS_ITEM, frame.chapter as u32),
            c.id_index(VERSES_ITEM, frame.verse as u32),
            c.id_index(QUEUE_ITEM, frame.queue_index as u32),
        ],
    };

    let hit = hit_test(c, frame, &anchors);

    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
            .end()
            .background_color(theme.base.bg),
        |c| {
            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(grow!())
                    .direction(LayoutDirection::LeftToRight)
                    .padding(metrics.padding(Space::Xs, Space::Xs))
                    .child_gap(metrics.space(Space::Xs))
                    .end(),
                |c| {
                    // Columna de navegacion: elegir libro y despues capitulo.
                    c.with(
                        Decl::new()
                            .layout()
                            .width(Sizing::Percent(0.4))
                            .height(grow!())
                            .direction(LayoutDirection::TopToBottom)
                            .child_gap(metrics.space(Space::Xs))
                            .end(),
                        |c| {
                            navigation::books(c, metrics, theme, frame, anchors.books);
                            navigation::chapters(c, metrics, theme, frame, anchors.chapters);
                        },
                    );

                    verses::view(c, metrics, theme, frame, anchors.verses);
                    queue::view(c, metrics, theme, frame, anchors.queue);
                },
            );

            bars::status(c, metrics, theme, frame);
            bars::query(c, metrics, theme, frame, anchors.query_field);
        },
    );

    (anchors, hit)
}

/// Averigua sobre que esta el puntero, antes de armar el arbol nuevo.
///
/// Clay resuelve el puntero contra la maquetacion **anterior**, y los ids de
/// esta son los mismos porque se derivan de `(etiqueta, indice)`. Por eso se
/// puede consultar aqui y aplicar el resaltado en el mismo fotograma, sin
/// exportar listas de ids ni reservar memoria.
fn hit_test(c: &Scope<'_>, frame: &Frame<'_>, anchors: &Anchors) -> Hit {
    let panels = [
        (
            Panel::Books,
            anchors.books.viewport,
            BOOKS_ITEM,
            frame.books.len(),
        ),
        (
            Panel::Chapters,
            anchors.chapters.viewport,
            CHAPTERS_ITEM,
            frame.chapters.len(),
        ),
        (
            Panel::Verses,
            anchors.verses.viewport,
            VERSES_ITEM,
            frame.verses.len(),
        ),
        (
            Panel::Queue,
            anchors.queue.viewport,
            QUEUE_ITEM,
            frame.queue.len(),
        ),
    ];

    for (panel, viewport, label, count) in panels {
        if !c.pointer_over(viewport) {
            continue;
        }

        let item = (0..count).find(|index| c.pointer_over(c.id_index(label, *index as u32)));

        return Hit {
            panel: Some(panel),
            item,
        };
    }

    Hit::default()
}
