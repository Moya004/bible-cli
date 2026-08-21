use clay_layout::{fit, grow, id::Id, layout::LayoutDirection, math::Vector2};

use crate::ui::design::metrics::Metrics;
use crate::ui::design::{Decl, Scope};

/// Identificadores de las dos piezas de un area desplazable.
///
/// Hacen falta despues de maquetar: el alto del contenido y el de la parte
/// visible son lo que permite acotar el desplazamiento, y solo se conocen
/// consultando `Clay::bounding_box` con estos ids.
#[derive(Clone, Copy)]
pub struct ScrollIds {
    pub viewport: Id,
    pub content: Id,
}

/// Recorta a su recuadro y corre el contenido hacia arriba.
///
/// El desplazamiento es manual porque los contenedores con scroll propio de
/// Clay necesitan estado de puntero, que en la terminal no existe: el llamador
/// guarda el desplazamiento y lo pasa aca.
pub fn scroll_area<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    ids: ScrollIds,
    offset: f32,
    gap: u16,
    children: impl FnOnce(&mut Scope<'render>),
) {
    let _ = metrics;

    c.with(
        Decl::new()
            .id(ids.viewport)
            .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
            .end()
            .clip(false, true, Vector2::new(0., -offset)),
        |c| {
            c.with(
                Decl::new()
                    .id(ids.content)
                    .layout()
                    .width(grow!())
                    .height(fit!())
                    .direction(LayoutDirection::TopToBottom)
                    .child_gap(gap)
                    .end(),
                children,
            );
        },
    );
}

/// Acota un desplazamiento a lo que realmente sobra de contenido.
pub fn clamp(offset: f32, content_height: f32, viewport_height: f32) -> f32 {
    offset.clamp(0., (content_height - viewport_height).max(0.))
}
