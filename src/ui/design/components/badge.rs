use clay_layout::{color::Color, fit};

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::design::{Decl, Scope};

/// Etiqueta corta y destacada.
///
/// Existe sobre todo para el aviso de pantalla en negro: sin algo imposible de
/// pasar por alto, el operador olvida que la salida esta oculta y pierde medio
/// minuto preguntandose por que no aparece nada.
pub fn badge<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    label: &'render str,
    tone: Color,
) {
    c.with(
        Decl::new()
            .layout()
            .width(fit!())
            .height(fit!())
            .padding(metrics.padding(Space::Sm, Space::None))
            .end()
            .background_color(tone),
        |c| {
            c.text(label, metrics.text_no_wrap(Role::Heading, theme.panel.bg));
        },
    );
}
