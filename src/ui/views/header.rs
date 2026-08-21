use crate::ui::design::Scope;
use crate::ui::design::components::bar::{bar, spacer};
use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::theme::Theme;
use crate::ui::model::Frame;

/// Barra superior: nombre, traduccion activa y ayuda de teclas.
pub fn view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) {
    bar(c, metrics, theme, |c| {
        c.text(frame.title, metrics.text(Role::Title, theme.accent));
        spacer(c);
        c.text(
            frame.translation,
            metrics.text_no_wrap(Role::Label, theme.accent),
        );
        c.text(frame.hint, metrics.text_no_wrap(Role::Label, theme.muted));
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

    bar(c, metrics, theme, |c| {
        c.text(frame.status, metrics.text_no_wrap(Role::Label, color));
    });
}
