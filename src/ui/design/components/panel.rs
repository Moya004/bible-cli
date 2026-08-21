use clay_layout::{
    fit, grow,
    layout::{LayoutDirection, Sizing},
};

use crate::ui::design::metrics::{BorderSides, Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::design::{Decl, Scope};

pub struct PanelProps<'a> {
    /// Encabezado. Sin titulo, el panel es solo una caja con borde.
    pub title: Option<&'a str>,
    /// Marca el panel que tiene el foco del teclado tiñendo su borde.
    pub focused: bool,
    pub width: Sizing,
    pub height: Sizing,
}

impl Default for PanelProps<'_> {
    fn default() -> Self {
        Self {
            title: None,
            focused: false,
            width: grow!(),
            height: grow!(),
        }
    }
}

/// Caja con borde y encabezado opcional.
///
/// Es el contenedor de Libros, Capitulos, Versos y Cola. El unico indicio de
/// donde esta el foco del teclado es el color del borde, asi que se dibuja
/// siempre, incluso cuando el panel no lo tiene.
pub fn panel<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    props: PanelProps<'render>,
    children: impl FnOnce(&mut Scope<'render>),
) {
    let border = if props.focused {
        theme.accent
    } else {
        theme.panel.border
    };

    c.with(
        Decl::new()
            .layout()
            .width(props.width)
            .height(props.height)
            .direction(LayoutDirection::TopToBottom)
            .padding(metrics.padding_in_border(Space::Md, Space::Xs, BorderSides::ALL))
            .child_gap(metrics.space(Space::Xs))
            .end()
            .background_color(theme.panel.bg)
            .border()
            .all_directions(metrics.border)
            .color(border)
            .end(),
        |c| {
            if let Some(title) = props.title {
                c.text(title, metrics.text_no_wrap(Role::Heading, border));
            }

            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(if props.title.is_some() {
                        grow!()
                    } else {
                        fit!()
                    })
                    .direction(LayoutDirection::TopToBottom)
                    .end(),
                children,
            );
        },
    );
}
