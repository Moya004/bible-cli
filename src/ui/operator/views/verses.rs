use clay_layout::grow;

use crate::ui::design::Scope;
use crate::ui::design::components::list::verse_item;
use crate::ui::design::components::panel::{PanelProps, panel};
use crate::ui::design::components::scroll_area::{ScrollIds, scroll_area};
use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::theme::{State, Theme};
use crate::ui::operator::action::Panel;
use crate::ui::operator::frame::Frame;

/// Panel de versos del capitulo, con su texto.
///
/// Se muestra el texto completo y no solo el numero porque el operador tiene
/// que leerlo para decidir cual manda: elegir a ciegas por numero obligaria a
/// proyectar para ver que decia.
pub fn view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    ids: ScrollIds,
) {
    let focused = frame.focus == Panel::Verses;

    panel(
        c,
        metrics,
        theme,
        PanelProps {
            title: Some("Versos"),
            focused,
            width: grow!(),
            height: grow!(),
        },
        |c| {
            if frame.verses.is_empty() {
                c.text(
                    "Sin versos",
                    metrics.text_no_wrap(Role::Label, theme.muted),
                );
                return;
            }

            scroll_area(c, metrics, ids, frame.scroll.verses, 0, |c| {
                for (index, verse) in frame.verses.iter().enumerate() {
                    verse_item(
                        c,
                        metrics,
                        theme,
                        state_of(index == frame.verse, focused),
                        &verse.number,
                        verse.text,
                    );
                }
            });
        },
    );
}

/// Un panel sin foco sigue mostrando cual es su elemento elegido, pero mas
/// apagado: en vivo hay que poder ver de un vistazo donde quedo cada panel.
pub fn state_of(selected: bool, focused: bool) -> State {
    match (selected, focused) {
        (true, true) => State::Selected,
        (true, false) => State::Focused,
        _ => State::Normal,
    }
}
