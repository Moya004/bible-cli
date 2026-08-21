use clay_layout::{
    color::Color,
    fit, grow,
    id::Id,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection, Sizing},
};

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::{State, Theme, TRANSPARENT};
use crate::ui::design::{Decl, Scope};

/// Contenido de un bloque de la rejilla.
///
/// Un solo tipo sirve para las dos rejillas: los capitulos son un numero suelto,
/// los libros llevan ademas el nombre completo debajo y una franja del color de
/// su seccion arriba.
#[derive(Clone, Copy)]
pub struct Block<'a> {
    /// Lo que va grande: la abreviatura del libro o el numero de capitulo
    pub label: &'a str,
    /// Nombre completo, en pequeño bajo la abreviatura
    pub sublabel: Option<&'a str>,
    /// Franja superior que agrupa por seccion
    pub accent: Option<Color>,
}

impl<'a> Block<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            sublabel: None,
            accent: None,
        }
    }
}

pub struct GridProps {
    /// Bloques por fila. **Clay no acomoda en varias filas por su cuenta**, asi
    /// que el llamador decide cuantos caben con el ancho que tiene.
    pub columns: usize,
    /// Indice elegido
    pub selected: Option<usize>,
    /// Indice bajo el puntero
    pub hovered: Option<usize>,
    /// El panel que contiene la rejilla tiene el foco del teclado
    pub focused: bool,
    /// Etiqueta con la que se derivan los ids de cada bloque, para poder
    /// preguntar despues por cual esta el puntero
    pub id_label: &'static str,
}

/// Rejilla de bloques seleccionables.
///
/// Los bloques crecen hasta llenar el ancho del panel. La ultima fila se rellena
/// con huecos: sin eso, dos capitulos sueltos al final creceran hasta ocupar
/// media fila cada uno y los bloques dejan de ser parejos.
///
/// El prestamo de `blocks` es independiente de `'render` a proposito. Clay guarda
/// punteros al texto, no a los `Block`, asi que la vista puede armar el vector
/// localmente apuntando a las cadenas del fotograma; atarlo a `'render` obligaria
/// a guardarlo en la arena y quedaria autorreferencial.
pub fn grid<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    props: &GridProps,
    blocks: &[Block<'render>],
) {
    let columns = props.columns.max(1);

    for (row, chunk) in blocks.chunks(columns).enumerate() {
        c.with(
            Decl::new()
                .layout()
                .width(grow!())
                .height(fit!())
                .direction(LayoutDirection::LeftToRight)
                .child_gap(metrics.space(Space::Xs))
                .end(),
            |c| {
                for (column, block) in chunk.iter().enumerate() {
                    cell(c, metrics, theme, props, row * columns + column, block);
                }

                // Huecos hasta completar la fila, para que los bloques de la
                // ultima mantengan el ancho de todos los demas.
                for _ in chunk.len()..columns {
                    c.with(
                        Decl::new().layout().width(grow!()).height(fit!()).end(),
                        |_| {},
                    );
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
    block: &Block<'render>,
) {
    let selected = props.selected == Some(index);
    let state = match (selected, props.focused, props.hovered == Some(index)) {
        (true, true, _) => State::Selected,
        (true, false, _) => State::Focused,
        (false, _, true) => State::Hover,
        _ => State::Normal,
    };

    let surface = theme.item.state(state);

    c.with(
        Decl::new()
            .id(c.id_index(props.id_label, index as u32))
            .layout()
            .width(grow!())
            .height(fit!())
            .direction(LayoutDirection::TopToBottom)
            .child_alignment(Alignment::new(
                LayoutAlignmentX::Center,
                LayoutAlignmentY::Center,
            ))
            .end()
            .background_color(surface.bg)
            .border()
            .all_directions(metrics.border)
            .color(if selected { theme.accent } else { TRANSPARENT })
            .end(),
        |c| {
            // Franja de seccion: agrupa sin robarle el fondo al resaltado.
            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(Sizing::Fixed(metrics.space(Space::Xs) as f32 / 2.))
                    .end()
                    .background_color(block.accent.unwrap_or(TRANSPARENT)),
                |_| {},
            );

            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .direction(LayoutDirection::TopToBottom)
                    .padding(metrics.padding(Space::Xs, Space::Xs))
                    .child_alignment(Alignment::new(
                        LayoutAlignmentX::Center,
                        LayoutAlignmentY::Center,
                    ))
                    .end(),
                |c| {
                    c.text(block.label, metrics.text_no_wrap(Role::Title, surface.fg));

                    if let Some(sublabel) = block.sublabel {
                        c.text(
                            sublabel,
                            metrics.text_no_wrap(Role::Caption, theme.muted),
                        );
                    }
                },
            );
        },
    );
}
