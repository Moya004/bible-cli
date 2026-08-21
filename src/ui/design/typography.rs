/// Grosor del trazo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
    Regular,
    Bold,
}

/// Realce del texto, independiente del grosor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emphasis {
    Normal,
    /// Texto atenuado: en la terminal es un atributo ANSI, en la ventana lo
    /// lleva el color y no cambia la tipografia.
    Dim,
}

/// Como se ve un texto, sin decir todavia con que tipografia se dibuja.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStyle {
    pub weight: Weight,
    pub emphasis: Emphasis,
}

impl TextStyle {
    pub const REGULAR: Self = Self {
        weight: Weight::Regular,
        emphasis: Emphasis::Normal,
    };
    pub const BOLD: Self = Self {
        weight: Weight::Bold,
        emphasis: Emphasis::Normal,
    };
    pub const DIM: Self = Self {
        weight: Weight::Regular,
        emphasis: Emphasis::Dim,
    };
}

const BIT_BOLD: u16 = 1 << 0;
const BIT_DIM: u16 = 1 << 1;

/// Empaqueta el estilo en el `font_id` de Clay.
///
/// Para Clay ese `u16` es opaco: solo lo transporta hasta el pintor y lo mete
/// en el hash de la cache de medidas. Antes se usaba como atributo ANSI en la
/// terminal y como indice de tipografia en la ventana, dos cosas distintas en
/// el mismo campo, y por eso no se podia pedir negrita y atenuado a la vez.
/// Ahora viaja como banderas y cada pintor decide que hacer con ellas.
pub const fn encode(style: TextStyle) -> u16 {
    let mut id = 0;

    if matches!(style.weight, Weight::Bold) {
        id |= BIT_BOLD;
    }
    if matches!(style.emphasis, Emphasis::Dim) {
        id |= BIT_DIM;
    }

    id
}

pub const fn decode(font_id: u16) -> TextStyle {
    TextStyle {
        weight: if font_id & BIT_BOLD != 0 {
            Weight::Bold
        } else {
            Weight::Regular
        },
        emphasis: if font_id & BIT_DIM != 0 {
            Emphasis::Dim
        } else {
            Emphasis::Normal
        },
    }
}
