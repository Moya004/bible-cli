use clay_layout::id::Id;

use crate::ui::design::Scope;
use crate::ui::design::components::badge::badge;
use crate::ui::design::components::bar::{bar, spacer};
use crate::ui::design::components::input_bar::{InputProps, input_bar};
use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::theme::Theme;
use crate::ui::operator::frame::Frame;

/// Barra de estado: que hay en pantalla y si la salida esta oculta.
pub fn status<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) {
    bar(c, metrics, theme, |c| {
        c.text("En pantalla:", metrics.text_no_wrap(Role::Label, theme.muted));
        c.text(frame.live, metrics.text_no_wrap(Role::Heading, theme.accent));

        spacer(c);

        if !frame.status.is_empty() {
            c.text(frame.status, metrics.text_no_wrap(Role::Label, theme.muted));
        }

        // Sin este aviso el operador olvida que la salida esta tapada y pierde
        // medio minuto preguntandose por que no aparece nada.
        if frame.blank {
            badge(c, metrics, theme, "NEGRO · F1", theme.error);
        }
    });
}

/// Barra de consulta: acepta las mismas citas escritas que el modo terminal.
pub fn query<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    field: Id,
) {
    input_bar(
        c,
        metrics,
        theme,
        InputProps {
            prompt: if frame.in_query { "cita>" } else { "/ buscar" },
            text: frame.query,
            h_scroll: frame.query_h_scroll,
            focused: frame.in_query,
            field,
        },
    );
}
