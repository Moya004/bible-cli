use clay_layout::Clay;

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::projector::view::{self, Slide};

/// Cuerpos entre los que se elige el del verso.
///
/// Cada uno necesita su propio atlas rasterizado, asi que la escalera es corta
/// a proposito: siete pasos cubren desde un salmo largo hasta un versiculo de
/// media linea sin gastar memoria de video de mas.
pub const LADDER: [u16; 7] = [28, 36, 44, 56, 68, 84, 104];

/// Proporcion entre el cuerpo y el alto de linea.
const LINE_HEIGHT: f32 = 1.3;

pub fn line_height(size: u16) -> u16 {
    (size as f32 * LINE_HEIGHT) as u16
}

/// Alto que le queda al verso una vez descontados margenes y cita.
///
/// Se calcula en vez de medirlo. El contenedor del verso crece, y en Clay un
/// elemento que crece nunca queda por debajo del tamaño de su contenido: si se
/// le preguntara cuanto mide, contestaria lo mismo que el verso y la comparacion
/// siempre daria que si.
pub fn available_height(metrics: &Metrics, window_height: f32) -> f32 {
    let margins = 2 * metrics.space(Space::Md);
    let gap = metrics.space(Space::Md);
    let cite = metrics.style(Role::Label).line_height;

    (window_height - (margins + gap + cite) as f32).max(0.)
}

/// Escoge el cuerpo mas grande con el que el verso todavia entra.
///
/// No se reimplementa el ajuste de linea: se maqueta de verdad con cada
/// candidato y se le pregunta a Clay cuanto ocupo. Clay es lo bastante barato
/// como para permitirse tres pasadas por fotograma, y asi el calculo no se puede
/// desincronizar de como se dibuja despues.
pub fn choose(
    clay: &mut Clay,
    metrics: &Metrics,
    theme: &Theme,
    slide: &Slide,
    available: f32,
) -> u16 {
    if slide.is_empty() {
        return LADDER[0];
    }

    let (mut low, mut high) = (0usize, LADDER.len() - 1);
    let mut best = 0usize;

    while low <= high {
        let middle = (low + high) / 2;

        if fits(clay, metrics, theme, slide, LADDER[middle], available) {
            best = middle;
            low = middle + 1;
        } else if middle == 0 {
            break;
        } else {
            high = middle - 1;
        }
    }

    LADDER[best]
}

/// Maqueta con un cuerpo y responde si el verso cabe en el hueco que le toca.
fn fits(
    clay: &mut Clay,
    metrics: &Metrics,
    theme: &Theme,
    slide: &Slide,
    size: u16,
    available: f32,
) -> bool {
    let metrics = metrics.with_size(Role::Display, size, line_height(size));

    let anchors = {
        let mut scope = clay.begin::<(), ()>();
        let anchors = view::root(&mut scope, &metrics, theme, slide, available);
        // Los comandos se descartan: de esta pasada solo interesa la medida.
        let _ = scope.end().count();
        anchors
    };

    clay.bounding_box(anchors.verse)
        .is_none_or(|verse| verse.height <= available)
}
