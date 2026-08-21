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
use crate::ui::operator::frame::Frame;

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

/// Ventana de control, segun el boceto: libros y capitulos a la izquierda, los
/// versos al centro, la cola a la derecha, y abajo el estado y la consulta.
pub fn root<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) -> Anchors {
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
        selected: [
            c.id("books_selected"),
            c.id("chapters_selected"),
            c.id("verses_selected"),
            c.id("queue_selected"),
        ],
    };

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
                            navigation::books(c, metrics, theme, frame, anchors.books, anchors.selected[0]);
                            navigation::chapters(c, metrics, theme, frame, anchors.chapters, anchors.selected[1]);
                        },
                    );

                    verses::view(c, metrics, theme, frame, anchors.verses, anchors.selected[2]);
                    queue::view(c, metrics, theme, frame, anchors.queue, anchors.selected[3]);
                },
            );

            bars::status(c, metrics, theme, frame);
            bars::query(c, metrics, theme, frame, anchors.query_field);
        },
    );

    anchors
}
