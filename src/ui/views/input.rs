use clay_layout::{
    fit, grow,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection},
    math::Vector2,
};

use crate::ui::{
    metrics::{BorderSides, Metrics, Role},
    model::Frame,
    theme::Theme,
    views::{Anchors, Decl, Scope},
};

/// Barra de entrada.
///
/// El texto no se ajusta a varias lineas: se dibuja completo dentro de un
/// contenedor que recorta y lo desplaza horizontalmente. Asi la posicion del
/// cursor es una simple medida del texto que lo precede, en vez de la aritmetica
/// de filas y columnas que hacia falta antes.
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
            .height(fit!())
            .direction(LayoutDirection::LeftToRight)
            .padding(metrics.padding_in_border(BorderSides::TOP))
            .child_alignment(Alignment::new(
                LayoutAlignmentX::Left,
                LayoutAlignmentY::Center,
            ))
            .end()
            .background_color(theme.panel)
            .border()
            .top(metrics.border)
            .color(theme.border)
            .end(),
        |c| {
            c.text(
                frame.prompt,
                metrics.text_no_wrap(Role::Body, theme.accent),
            );

            c.with(
                Decl::new()
                    .id(anchors.input_field)
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .end()
                    .clip(true, false, Vector2::new(-frame.h_scroll, 0.)),
                |c| {
                    c.text(frame.input, metrics.text_no_wrap(Role::Body, theme.text));
                },
            );
        },
    );
}
