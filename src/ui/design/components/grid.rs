use clay_layout::{
    fit, grow, id::Id,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection, Sizing},
};

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::{State, Theme};
use crate::ui::design::{Decl, Scope};

pub struct GridProps {
    /// Celdas por fila. **Clay no acomoda en varias filas por su cuenta**, asi
    /// que el llamador decide cuantas caben con el ancho que tiene.
    pub columns: usize,
    /// Ancho de cada celda. Fijo para numeros, que se ven mejor alineados; que
    /// crezca para nombres de largo desparejo.
    pub cell: Sizing,
    /// Indice elegido dentro de `labels`.
    pub selected: Option<usize>,
    /// El panel que contiene la rejilla tiene el foco del teclado.
    pub focused: bool,
    /// Id de la celda elegida, para poder correr la vista hasta ella.
    pub selected_id: Option<Id>,
}

/// Rejilla de celdas seleccionables.
///
/// Sirve igual para los 66 libros que para los capitulos de uno: en rejilla
/// caben todos a la vista y no hay que desplazar nada mientras alguien espera.
pub fn grid<'render, Label: AsRef<str>>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    props: GridProps,
    labels: &'render [Label],
) {
    let columns = props.columns.max(1);

    for (row, chunk) in labels.chunks(columns).enumerate() {
        c.with(
            Decl::new()
                .layout()
                .width(grow!())
                .height(fit!())
                .direction(LayoutDirection::LeftToRight)
                .child_gap(metrics.space(Space::Xs))
                .end(),
            |c| {
                for (column, label) in chunk.iter().enumerate() {
                    let index = row * columns + column;
                    cell(c, metrics, theme, &props, index, label.as_ref());
                }
            },
        );
    }
}

fn cell<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    props: &GridProps,
    index: usize,
    label: &'render str,
) {
    let state = match props.selected {
        Some(selected) if selected == index => {
            if props.focused {
                State::Selected
            } else {
                State::Focused
            }
        }
        _ => State::Normal,
    };

    let surface = theme.item.state(state);
    let mut declaration = Decl::new();

    if state != State::Normal && let Some(id) = props.selected_id {
        declaration.id(id);
    }

    c.with(
        declaration
            .layout()
            .width(props.cell)
            .height(fit!())
            .padding(metrics.padding(Space::Sm, Space::None))
            .child_alignment(Alignment::new(
                LayoutAlignmentX::Center,
                LayoutAlignmentY::Center,
            ))
            .end()
            .background_color(surface.bg),
        |c| {
            c.text(label, metrics.text_no_wrap(Role::Body, surface.fg));
        },
    );
}
