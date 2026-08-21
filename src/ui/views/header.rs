use clay_layout::{
    fit, grow,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection, Padding},
    math::Vector2,
};

use crate::ui::{
    metrics::{Metrics, Role},
    model::Frame,
    theme::Theme,
    views::{Decl, Scope},
};

/// Barra superior: nombre, traduccion activa y ayuda de teclas.
pub fn view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) {
    c.with(&bar(metrics, theme), |c| {
        c.text(frame.title, metrics.text(Role::Title, theme.accent));

        // Empuja el resto hacia la derecha.
        c.with(
            &Decl::new().layout().width(grow!()).height(fit!()).end(),
            |_| {},
        );

        c.text(
            frame.translation,
            metrics.text_no_wrap(Role::Label, theme.accent),
        );
        c.text(
            frame.hint,
            metrics.text_no_wrap(Role::Label, theme.muted),
        );
    });
}

/// Barra de estado, justo encima de la entrada.
pub fn status_bar<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) {
    let color = if frame.status_is_error {
        theme.error
    } else {
        theme.muted
    };

    c.with(&bar(metrics, theme), |c| {
        c.text(frame.status, metrics.text_no_wrap(Role::Label, color));
    });
}

/// Fila de una sola linea, recortada para que un texto largo no se desborde
/// sobre el resto de la interfaz en ventanas angostas.
fn bar<'render>(metrics: &Metrics, theme: &Theme) -> Decl<'render> {
    let mut declaration = Decl::new();

    declaration
        .layout()
        .width(grow!())
        .height(fit!())
        .direction(LayoutDirection::LeftToRight)
        .padding(Padding::new(
            metrics.pad_x,
            metrics.pad_x,
            metrics.pad_y,
            metrics.pad_y,
        ))
        .child_gap(metrics.gap)
        .child_alignment(Alignment::new(
            LayoutAlignmentX::Left,
            LayoutAlignmentY::Center,
        ))
        .end()
        .background_color(theme.panel)
        .clip(true, false, Vector2::new(0., 0.));

    declaration
}
