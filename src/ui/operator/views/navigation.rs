use clay_layout::grow;

use crate::ui::design::Scope;
use crate::ui::design::components::grid::{Block, GridProps, grid};
use crate::ui::design::components::panel::{PanelProps, panel};
use crate::ui::design::components::scroll_area::{ScrollIds, scroll_area};
use crate::ui::design::metrics::Metrics;
use crate::ui::design::theme::Theme;
use crate::ui::operator::action::Panel;
use crate::ui::operator::frame::Frame;
use crate::ui::operator::views::{BOOKS_ITEM, CHAPTERS_ITEM, hovered};

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
            // El vector es local: Clay guarda punteros al texto, no a los
            // `Block`, y las cadenas viven en el fotograma.
            let blocks: Vec<Block> = frame
                .books
                .iter()
                .map(|entry| Block {
                    label: entry.abbreviation,
                    sublabel: Some(entry.name),
                    accent: Some(entry.accent),
                })
                .collect();

            scroll_area(c, metrics, ids, frame.scroll.books, 0, |c| {
                grid(
                    c,
                    metrics,
                    theme,
                    &GridProps {
                        columns: frame.book_columns,
                        selected: Some(frame.book),
                        hovered: hovered(frame, Panel::Books),
                        focused,
                        id_label: BOOKS_ITEM,
                    },
                    &blocks,
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
            let blocks: Vec<Block> = frame
                .chapters
                .iter()
                .map(|number| Block::new(number))
                .collect();

            scroll_area(c, metrics, ids, frame.scroll.chapters, 0, |c| {
                grid(
                    c,
                    metrics,
                    theme,
                    &GridProps {
                        columns: frame.chapter_columns,
                        selected: Some(frame.chapter),
                        hovered: hovered(frame, Panel::Chapters),
                        focused,
                        id_label: CHAPTERS_ITEM,
                    },
                    &blocks,
                );
            });
        },
    );
}
