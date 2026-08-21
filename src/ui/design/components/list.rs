use clay_layout::{fit, grow, layout::LayoutDirection};

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::{State, Theme};
use crate::ui::design::{Decl, Scope};

/// Fila de una lista, con su estado.
///
/// El fondo y el color de texto salen de `theme.item`, que es lo que hace que
/// «seleccionado» se vea igual en toda la aplicacion.
pub fn list_item<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    state: State,
    children: impl FnOnce(&mut Scope<'render>),
) {
    let surface = theme.item.state(state);

    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(fit!())
            .direction(LayoutDirection::LeftToRight)
            .padding(metrics.padding(Space::Sm, Space::None))
            .child_gap(metrics.space(Space::Sm))
            .end()
            .background_color(surface.bg),
        children,
    );
}

/// Fila de texto simple: el caso de la Cola y del panel de Libros en lista.
pub fn text_item<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    state: State,
    label: &'render str,
) {
    let surface = theme.item.state(state);

    list_item(c, metrics, theme, state, |c| {
        c.text(label, metrics.text_no_wrap(Role::Body, surface.fg));
    });
}

/// Fila de verso: numero a la derecha en una columna fija y el texto ajustado
/// al ancho restante, para que todos los versos queden con el margen parejo.
pub fn verse_item<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    state: State,
    number: &'render str,
    text: &'render str,
) {
    use clay_layout::layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, Sizing};

    let surface = theme.item.state(state);

    list_item(c, metrics, theme, state, |c| {
        c.with(
            Decl::new()
                .layout()
                .width(Sizing::Fixed(metrics.number_column))
                .height(fit!())
                .child_alignment(Alignment::new(
                    LayoutAlignmentX::Right,
                    LayoutAlignmentY::Top,
                ))
                .end(),
            |c| {
                c.text(
                    number,
                    metrics.text_no_wrap(Role::VerseNumber, theme.verse_number),
                );
            },
        );

        // El texto va en su propio contenedor que crece: asi Clay conoce el
        // ancho disponible y puede ajustar las lineas.
        c.with(
            Decl::new()
                .layout()
                .width(grow!())
                .height(fit!())
                .direction(LayoutDirection::TopToBottom)
                .end(),
            |c| {
                c.text(text, metrics.text(Role::Body, surface.fg));
            },
        );
    });
}
