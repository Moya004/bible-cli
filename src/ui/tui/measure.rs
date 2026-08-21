use clay_layout::{math::Dimensions, text::TextConfig};
use unicode_width::UnicodeWidthStr;

/// Mide un texto en celdas de terminal.
///
/// Clay llama a esto por palabra para decidir donde cortar las lineas, asi que
/// aqui se decide todo el ajuste de linea. Se usa el ancho de presentacion de
/// Unicode y no el numero de caracteres: las vocales acentuadas ocupan una celda
/// aunque sean varios bytes, y un emoji como `📔` ocupa dos.
pub fn measure(text: &str, _config: &TextConfig) -> Dimensions {
    Dimensions::new(UnicodeWidthStr::width(text) as f32, 1.)
}

/// Mismo criterio, para las medidas que el backend necesita fuera de Clay
/// (la posicion del cursor en la barra de entrada).
pub fn width(text: &str) -> f32 {
    UnicodeWidthStr::width(text) as f32
}
