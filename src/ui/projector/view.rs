use clay_layout::{
    fit, grow, id::Id,
    layout::{Alignment, LayoutAlignmentX, LayoutAlignmentY, LayoutDirection, Sizing},
    math::Vector2,
    text::{TextAlignment, TextConfig, TextElementConfigWrapMode},
};

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::design::{Decl, Scope};

/// Lo que hay en pantalla. Vive fuera del bucle de dibujo porque Clay guarda
/// punteros al texto y no copias.
pub struct Slide {
    pub cite: String,
    pub text: String,
    pub blank: bool,
}

impl Slide {
    pub fn empty() -> Self {
        Self {
            cite: String::new(),
            text: String::new(),
            blank: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }
}

/// Identificador para medir despues de maquetar: con el alto del verso se
/// decide que cuerpo usar en la pasada definitiva.
#[derive(Clone, Copy)]
pub struct Anchors {
    pub verse: Id,
}

/// Pantalla del proyector: el verso ocupando el centro y la cita al pie.
///
/// `area_height` es el hueco que le toca al verso. Se recibe hecho en vez de
/// dejar que el contenedor crezca solo: acotado, un verso demasiado largo se
/// recorta, y sin acotar empujaria la cita fuera de la pantalla.
pub fn root<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    slide: &'render Slide,
    area_height: f32,
) -> Anchors {
    let anchors = Anchors {
        verse: c.id("projector_verse"),
    };

    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
            .padding(metrics.padding(Space::Lg, Space::Md))
            .child_gap(metrics.space(Space::Md))
            .end()
            .background_color(theme.base.bg),
        |c| {
            // Con la pantalla oculta no se dibuja nada, pero el verso sigue
            // guardado: volver a mostrarlo es una tecla, no una busqueda.
            if slide.blank {
                return;
            }

            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(Sizing::Grow(0., area_height))
                    .child_alignment(Alignment::new(
                        LayoutAlignmentX::Center,
                        LayoutAlignmentY::Center,
                    ))
                    .end()
                    .clip(false, true, Vector2::new(0., 0.)),
                |c| {
                    c.with(
                        Decl::new()
                            .id(anchors.verse)
                            .layout()
                            .width(grow!())
                            .height(fit!())
                            .direction(LayoutDirection::TopToBottom)
                            // `TextAlignment::Center` solo centra las lineas
                            // *dentro* del elemento de texto, y ese elemento
                            // mide lo que su linea mas larga. Para un verso de
                            // una linea eso no centra nada: hay que centrar el
                            // elemento dentro de su contenedor.
                            .child_alignment(Alignment::new(
                                LayoutAlignmentX::Center,
                                LayoutAlignmentY::Top,
                            ))
                            .end(),
                        |c| {
                            c.text(&slide.text, centered(metrics, theme, Role::Display));
                        },
                    );
                },
            );

            c.with(
                Decl::new()
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .child_alignment(Alignment::new(
                        LayoutAlignmentX::Center,
                        LayoutAlignmentY::Center,
                    ))
                    .end(),
                |c| {
                    c.text(&slide.cite, centered(metrics, theme, Role::Label));
                },
            );
        },
    );

    anchors
}

/// El texto proyectado va centrado, que es lo que se lee de frente y a
/// distancia; alineado a la izquierda deja un borde irregular molesto.
fn centered(
    metrics: &Metrics,
    theme: &Theme,
    role: Role,
) -> clay_layout::text::TextElementConfig {
    let style = metrics.style(role);
    let color = if matches!(role, Role::Label) {
        theme.muted
    } else {
        theme.base.fg
    };

    TextConfig::new()
        .color(color)
        .font_id(style.font_id())
        .font_size(style.size)
        .line_height(style.line_height)
        .wrap_mode(TextElementConfigWrapMode::Words)
        .alignment(TextAlignment::Center)
        .end()
}
