use clay_layout::{
    fit, grow,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection},
    math::Vector2,
};

use crate::ui::design::metrics::Metrics;
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::design::{Decl, Scope};

/// Franja de una sola linea: cabecera, barra de estado, pie.
///
/// Recorta su contenido a proposito. En una ventana angosta un texto largo se
/// desbordaria sobre el resto de la interfaz, y en la terminal se pintaria
/// encima de la fila siguiente.
pub fn bar<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    children: impl FnOnce(&mut Scope<'render>),
) {
    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(fit!())
            .direction(LayoutDirection::LeftToRight)
            .padding(metrics.padding(Space::Md, Space::Xs))
            .child_gap(metrics.space(Space::Sm))
            .child_alignment(Alignment::new(
                LayoutAlignmentX::Left,
                LayoutAlignmentY::Center,
            ))
            .end()
            .background_color(theme.panel.bg)
            .clip(true, false, Vector2::new(0., 0.)),
        children,
    );
}

/// Separador que empuja lo que sigue hacia el borde derecho de una barra.
pub fn spacer<'render>(c: &mut Scope<'render>) {
    c.with(
        Decl::new().layout().width(grow!()).height(fit!()).end(),
        |_| {},
    );
}
