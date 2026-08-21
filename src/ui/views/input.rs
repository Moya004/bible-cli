use crate::ui::design::Scope;
use crate::ui::design::components::input_bar::{InputProps, input_bar};
use crate::ui::design::metrics::Metrics;
use crate::ui::design::theme::Theme;
use crate::ui::model::Frame;
use crate::ui::views::Anchors;

/// Barra de entrada de la consulta.
pub fn view<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
    anchors: Anchors,
) {
    input_bar(
        c,
        metrics,
        theme,
        InputProps {
            prompt: frame.prompt,
            text: frame.input,
            h_scroll: frame.h_scroll,
            focused: true,
            field: anchors.input_field,
        },
    );
}
