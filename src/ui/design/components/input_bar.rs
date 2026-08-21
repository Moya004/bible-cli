use clay_layout::{
    fit, grow, id::Id,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection},
    math::Vector2,
};

use crate::ui::design::metrics::{BorderSides, Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::design::{Decl, Scope};

pub struct InputProps<'a> {
    /// Simbolo que precede al texto
    pub prompt: &'a str,
    pub text: &'a str,
    /// Desplazamiento horizontal del texto, en unidades de Clay
    pub h_scroll: f32,
    pub focused: bool,
    /// Identifica la zona donde se dibuja el texto. El backend consulta su
    /// recuadro para situar el cursor.
    pub field: Id,
}

/// Barra de entrada de una sola linea.
///
/// El texto no se parte en varias filas: se dibuja entero dentro de un
/// contenedor que recorta y lo corre en horizontal. Asi la posicion del cursor
/// es el ancho del texto que lo precede, en vez de la aritmetica de filas y
/// columnas que hacia falta cuando la entrada se ajustaba.
pub fn input_bar<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    props: InputProps<'render>,
) {
    let border = if props.focused {
        theme.accent
    } else {
        theme.panel.border
    };

    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(fit!())
            .direction(LayoutDirection::LeftToRight)
            .padding(metrics.padding_in_border(Space::Md, Space::Xs, BorderSides::TOP))
            .child_alignment(Alignment::new(
                LayoutAlignmentX::Left,
                LayoutAlignmentY::Center,
            ))
            .end()
            .background_color(theme.panel.bg)
            .border()
            .top(metrics.border)
            .color(border)
            .end(),
        |c| {
            c.text(
                props.prompt,
                metrics.text_no_wrap(Role::Body, theme.accent),
            );

            c.with(
                Decl::new()
                    .id(props.field)
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .end()
                    .clip(true, false, Vector2::new(-props.h_scroll, 0.)),
                |c| {
                    c.text(
                        props.text,
                        metrics.text_no_wrap(Role::Body, theme.panel.fg),
                    );
                },
            );
        },
    );
}

/// Corrige el desplazamiento horizontal para que el cursor quede a la vista.
///
/// El ancho del texto que precede al cursor lo mide el backend, que es quien
/// conoce la tipografia. Devuelve el desplazamiento nuevo.
pub fn keep_caret_visible(h_scroll: f32, prefix_width: f32, field_width: f32) -> f32 {
    let mut scroll = h_scroll;

    if prefix_width - scroll > field_width {
        scroll = prefix_width - field_width;
    }
    if prefix_width < scroll {
        scroll = prefix_width;
    }

    scroll.max(0.)
}
