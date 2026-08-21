use clay_layout::{fit, grow, layout::LayoutDirection};

use crate::ui::design::components::list::verse_item;
use crate::ui::design::components::scroll_area::scroll_area;
use crate::ui::design::components::text_block::text_block;
use crate::ui::design::metrics::{BorderSides, Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::{State, Theme};
use crate::ui::design::{Decl, Scope};
use crate::ui::model::{Card, Frame};
use crate::ui::views::Anchors;

/// Panel de resultados: una tarjeta por pasaje, con desplazamiento vertical.
pub fn view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    anchors: Anchors,
) {
    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
            .padding(metrics.padding_in_border(Space::Md, Space::Xs, BorderSides::ALL))
            .end()
            .border()
            .all_directions(metrics.border)
            .color(theme.base.border)
            .end(),
        |c| {
            scroll_area(
                c,
                metrics,
                anchors.results,
                frame.scroll_y,
                metrics.space(Space::Sm),
                |c| {
                    if frame.cards.is_empty() {
                        text_block(c, metrics, Role::Label, theme.muted, frame.empty_hint);
                        return;
                    }

                    for card in &frame.cards {
                        card_view(c, metrics, theme, card);
                    }
                },
            );
        },
    );
}

/// Tarjeta de un pasaje: titulo y versos, con una barra de acento a la
/// izquierda que separa un pasaje del siguiente sin gastar una fila.
fn card_view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    card: &'render Card<'render>,
) {
    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(fit!())
            .direction(LayoutDirection::TopToBottom)
            .padding(metrics.padding_in_border(Space::Sm, Space::None, BorderSides::LEFT))
            .child_gap(metrics.space(Space::Sm))
            .end()
            .background_color(theme.card.bg)
            .border()
            .left(metrics.border)
            .color(theme.card.border)
            .end(),
        |c| {
            c.text(&card.title, metrics.text(Role::Title, theme.accent));

            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .direction(LayoutDirection::TopToBottom)
                    .child_gap(metrics.space(Space::None))
                    .end(),
                |c| {
                    for verse in &card.verses {
                        verse_item(
                            c,
                            metrics,
                            theme,
                            State::Normal,
                            None,
                            &verse.number,
                            verse.text,
                        );
                    }
                },
            );
        },
    );
}
