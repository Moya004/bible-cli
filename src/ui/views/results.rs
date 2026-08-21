use clay_layout::{
    fit, grow,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection, Padding},
    math::Vector2,
};

use crate::ui::{
    metrics::{BorderSides, Metrics, Role},
    model::{Card, Frame},
    theme::Theme,
    views::{Anchors, Decl, Scope},
};

/// Panel de resultados: una tarjeta por pasaje, con desplazamiento vertical.
///
/// El desplazamiento es manual (`clip` con `child_offset`) porque los
/// contenedores con scroll propio de Clay necesitan estado de puntero, que en la
/// terminal no existe.
pub fn view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    anchors: Anchors,
) {
    c.with(
        Decl::new()
            .id(anchors.results_viewport)
            .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
            .padding(metrics.padding_in_border(BorderSides::ALL))
            .end()
            .border()
            .all_directions(metrics.border)
            .color(theme.border)
            .end()
            .clip(false, true, Vector2::new(0., -frame.scroll_y)),
        |c| {
            c.with(
                Decl::new()
                    .id(anchors.results_content)
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .direction(LayoutDirection::TopToBottom)
                    .child_gap(metrics.card_gap)
                    .end(),
                |c| {
                    if frame.cards.is_empty() {
                        c.text(frame.empty_hint, metrics.text(Role::Label, theme.muted));
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
            .padding(Padding::new(
                metrics.card_pad_x + metrics.border,
                metrics.card_pad_x,
                metrics.card_pad_y,
                metrics.card_pad_y,
            ))
            .child_gap(metrics.gap)
            .end()
            .background_color(theme.card)
            .border()
            .left(metrics.border)
            .color(theme.accent)
            .end(),
        |c| {
            c.text(&card.title, metrics.text(Role::Title, theme.accent));

            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .direction(LayoutDirection::TopToBottom)
                    .child_gap(metrics.verse_gap)
                    .end(),
                |c| {
                    for verse in &card.verses {
                        verse_row(c, metrics, theme, &verse.number, verse.text);
                    }
                },
            );
        },
    );
}

fn verse_row<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    number: &'render str,
    text: &'render str,
) {
    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(fit!())
            .direction(LayoutDirection::LeftToRight)
            .child_gap(metrics.gap)
            .end(),
        |c| {
            // Columna del numero, alineada a la derecha para que los versos
            // queden con el margen parejo.
            c.with(
                Decl::new()
                    .layout()
                    .width(clay_layout::layout::Sizing::Fixed(metrics.number_column))
                    .height(fit!())
                    .child_alignment(Alignment::new(
                        LayoutAlignmentX::Right,
                        LayoutAlignmentY::Top,
                    ))
                    .end(),
                |c| {
                    c.text(
                        number,
                        metrics.text_no_wrap(Role::VerseNumber, theme.verse_number),
                    );
                },
            );

            // El texto va en su propio contenedor que crece: asi Clay conoce el
            // ancho disponible y hace el ajuste de linea.
            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .direction(LayoutDirection::TopToBottom)
                    .end(),
                |c| {
                    c.text(text, metrics.text(Role::Body, theme.text));
                },
            );
        },
    );
}
