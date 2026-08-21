pub mod header;
pub mod input;
pub mod results;

use clay_layout::{
    ClayLayoutScope, Declaration, grow, id::Id, layout::LayoutDirection,
};

use crate::ui::{metrics::Metrics, model::Frame, theme::Theme};

/// Alias del ambito de maquetacion. No usamos imagenes ni elementos
/// personalizados, asi que ambos parametros son `()` y las vistas sirven igual
/// para la terminal y para la ventana.
pub type Scope<'render> = ClayLayoutScope<'render, 'render, (), ()>;
pub type Decl<'render> = Declaration<'render, (), ()>;

/// Identificadores de los elementos que el backend necesita medir despues de
/// maquetar.
///
/// `Id::new` es privado en `clay-layout`, asi que los ids solo se pueden obtener
/// desde el ambito; se crean durante la maquetacion y se devuelven aqui para
/// consultarlos luego con `Clay::bounding_box`.
#[derive(Clone, Copy)]
pub struct Anchors {
    /// Zona donde se dibuja el texto de la entrada, para situar el cursor
    pub input_field: Id,
    /// Contenido del panel de resultados, para conocer su alto total
    pub results_content: Id,
    /// Parte visible del panel, para saber cuanto se puede desplazar
    pub results_viewport: Id,
}

/// Construye la interfaz completa. Identica en ambos modos: lo unico que cambia
/// es la escala que trae [`Metrics`] y la paleta de [`Theme`].
pub fn root<'render>(
    c: &mut Scope<'render>,
    metrics: &Metrics,
    theme: &Theme,
    frame: &'render Frame<'render>,
) -> Anchors {
    let anchors = Anchors {
        input_field: c.id("input_field"),
        results_content: c.id("results_content"),
        results_viewport: c.id("results_viewport"),
    };

    c.with(
        &Decl::new()
            .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
            .end()
            .background_color(theme.base),
        |c| {
            header::view(c, metrics, theme, frame);
            results::view(c, metrics, theme, frame, anchors);
            header::status_bar(c, metrics, theme, frame);
            input::view(c, metrics, theme, frame, anchors);
        },
    );

    anchors
}
