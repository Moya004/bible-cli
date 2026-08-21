/// Paso de la escala de espaciado.
///
/// Antes cada sitio de uso tenia su propio campo en `Metrics` (`pad_x`,
/// `card_pad_x`, `gap`, `verse_gap`, `card_gap`...), asi que agregar un
/// componente obligaba a inventar campos nuevos. Con una escala, un componente
/// nuevo escoge un paso que ya existe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

/// Valores concretos de la escala para un backend.
///
/// En la terminal la unidad es una celda, asi que la escala es corta y casi
/// todos los pasos valen 0 o 1: no hay medias celdas. Eso obliga a algo poco
/// evidente: `Xs` es el unico paso que vale cero en terminal, asi que es el que
/// tienen que usar los rellenos verticales de las barras — cualquier otro las
/// convertiria en tres filas de alto.
#[derive(Debug, Clone, Copy)]
pub struct SpaceScale {
    steps: [u16; 6],
}

impl SpaceScale {
    pub const TERMINAL: Self = Self {
        steps: [0, 0, 1, 1, 2, 3],
    };

    pub const WINDOW: Self = Self {
        steps: [0, 8, 12, 16, 24, 32],
    };

    pub const PROJECTOR: Self = Self {
        steps: [0, 8, 16, 32, 48, 72],
    };

    pub const fn get(&self, space: Space) -> u16 {
        self.steps[space as usize]
    }
}
