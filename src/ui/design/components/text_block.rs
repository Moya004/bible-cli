use clay_layout::{color::Color, fit, grow, layout::LayoutDirection};

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::{Decl, Scope};

/// Parrafo ajustado al ancho disponible.
///
/// Va envuelto en un contenedor que crece a proposito: un texto suelto entre
/// hermanos se encoge hasta su palabra mas larga, mientras que dentro de un
/// contenedor que crece Clay conoce el ancho y ajusta las lineas ahi.
pub fn text_block<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    role: Role,
    color: Color,
    text: &'render str,
) {
    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(fit!())
            .direction(LayoutDirection::TopToBottom)
            .end(),
        |c| {
            c.text(text, metrics.text(role, color));
        },
    );
}
