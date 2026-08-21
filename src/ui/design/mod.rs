pub mod metrics;
pub mod space;
pub mod theme;
pub mod typography;

use clay_layout::{ClayLayoutScope, Declaration};

/// Ambito de maquetacion.
///
/// No se usan imagenes ni elementos personalizados, asi que ambos parametros de
/// Clay son `()` y las vistas sirven igual para la terminal, la ventana y el
/// proyector. Las dos vidas se unifican porque `ClayLayoutScope` exige que el
/// prestamo de `Clay` dure al menos lo que el texto del fotograma.
pub type Scope<'render> = ClayLayoutScope<'render, 'render, (), ()>;
pub type Decl<'render> = Declaration<'render, (), ()>;
