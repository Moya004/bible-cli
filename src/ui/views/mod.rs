pub mod header;
pub mod input;
pub mod results;

use clay_layout::{grow, id::Id, layout::LayoutDirection};

use crate::ui::design::components::scroll_area::ScrollIds;
use crate::ui::design::metrics::Metrics;
use crate::ui::design::theme::Theme;
use crate::ui::design::{Decl, Scope};
use crate::ui::model::Frame;

/// Identificadores de los elementos que el backend necesita medir despues de
/// maquetar.
///
/// `Id::new` es privado en `clay-layout`, asi que los ids solo se pueden obtener
/// desde el ambito: se crean durante la maquetacion y se devuelven aqui para
/// consultarlos luego con `Clay::bounding_box`.
#[derive(Clone, Copy)]
pub struct Anchors {
    /// Zona donde se dibuja el texto de la entrada, para situar el cursor
    pub input_field: Id,
    /// Panel de resultados, para acotar el desplazamiento
    pub results: ScrollIds,
}

/// Interfaz del modo consulta, identica en terminal y en ventana: lo unico que
/// cambia es la escala que trae [`Metrics`] y la paleta de [`Theme`].
pub fn root<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) -> Anchors {
    let anchors = Anchors {
        input_field: c.id("input_field"),
        results: ScrollIds {
            viewport: c.id("results_viewport"),
            content: c.id("results_content"),
        },
    };

    c.with(
        Decl::new()
            .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
            .end()
            .background_color(theme.base.bg),
        |c| {
            header::view(c, metrics, theme, frame);
            results::view(c, metrics, theme, frame, anchors);
            header::status_bar(c, metrics, theme, frame);
            input::view(c, metrics, theme, frame, anchors);
        },
    );

    anchors
}
