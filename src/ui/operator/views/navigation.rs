use clay_layout::{grow, id::Id, layout::Sizing};

use crate::ui::design::Scope;
use crate::ui::design::components::grid::{GridProps, grid};
use crate::ui::design::components::panel::{PanelProps, panel};
use crate::ui::design::components::scroll_area::{ScrollIds, scroll_area};
use crate::ui::design::metrics::Metrics;
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::operator::action::Panel;
use crate::ui::operator::frame::Frame;
use crate::ui::operator::state::{BOOK_COLUMNS, CHAPTER_COLUMNS};

/// Panel de libros: los 66 en rejilla.
///
/// En rejilla caben todos a la vista y no hay que desplazar nada mientras
/// alguien espera; en lista habria que recorrerlos de a uno.
pub fn books<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    ids: ScrollIds,
    selected: Id,
) {
    let focused = frame.focus == Panel::Books;

    panel(
        c,
        metrics,
        theme,
        PanelProps {
            title: Some("Libros"),
            focused,
            width: grow!(),
            height: grow!(),
        },
        |c| {
            scroll_area(c, metrics, ids, frame.scroll.books, 0, |c| {
                grid(
                    c,
                    metrics,
                    theme,
                    GridProps {
                        columns: BOOK_COLUMNS,
                        cell: grow!(),
                        selected: Some(frame.book),
                        focused,
                        selected_id: Some(selected),
                    },
                    frame.books,
                );
            });
        },
    );
}

/// Panel de capitulos del libro elegido.
pub fn chapters<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    ids: ScrollIds,
    selected: Id,
) {
    let focused = frame.focus == Panel::Chapters;

    panel(
        c,
        metrics,
        theme,
        PanelProps {
            title: Some("Capitulos"),
            focused,
            width: grow!(),
            height: grow!(),
        },
        |c| {
            scroll_area(c, metrics, ids, frame.scroll.chapters, 0, |c| {
                grid(
                    c,
                    metrics,
                    theme,
                    GridProps {
                        columns: CHAPTER_COLUMNS,
                        // Ancho fijo: los numeros quedan en columnas parejas y
                        // no bailan al pasar de un libro de 4 capitulos a uno
                        // de 150.
                        cell: Sizing::Fixed(metrics.space(Space::Xl) as f32),
                        selected: Some(frame.chapter),
                        focused,
                        selected_id: Some(selected),
                    },
                    &frame.chapters,
                );
            });
        },
    );
}
