use clay_layout::color::Color;

use crate::business::domain::Section;

/// Color con alfa cero: le dice al pintor que no dibuje fondo y deje ver el de
/// la terminal o el de la ventana.
pub const TRANSPARENT: Color = Color::rgba(0., 0., 0., 0.);

/// Los tres colores de una superficie dibujable.
#[derive(Debug, Clone, Copy)]
pub struct Surface {
    pub bg: Color,
    pub fg: Color,
    pub border: Color,
}

/// Estado de un elemento con el que se puede interactuar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    /// El puntero esta encima. En la terminal no ocurre nunca.
    Hover,
    /// El panel que lo contiene tiene el foco del teclado.
    Focused,
    /// Es el elemento elegido dentro de su lista o rejilla.
    Selected,
}

/// Superficie que cambia segun el estado.
#[derive(Debug, Clone, Copy)]
pub struct Interactive {
    pub normal: Surface,
    pub hover: Surface,
    pub focused: Surface,
    pub selected: Surface,
}

impl Interactive {
    pub fn state(&self, state: State) -> Surface {
        match state {
            State::Normal => self.normal,
            State::Hover => self.hover,
            State::Focused => self.focused,
            State::Selected => self.selected,
        }
    }
}

/// Paleta de la interfaz.
///
/// Es dato de ejecucion y no una constante: asi se puede cambiar de tema, o
/// degradar a menos colores cuando la terminal no acepte 24 bits, sin tocar
/// ninguna vista.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// Fondo de la aplicacion
    pub base: Surface,
    /// Barras y paneles
    pub panel: Surface,
    /// Tarjetas y cajas de contenido
    pub card: Surface,
    /// Elementos de lista y de rejilla
    pub item: Interactive,
    /// Realces, titulos y el simbolo de la entrada
    pub accent: Color,
    /// Texto secundario
    pub muted: Color,
    /// Numero de verso
    pub verse_number: Color,
    /// Errores en la barra de estado
    pub error: Color,
}

/// Color de la franja de cada seccion del canon.
///
/// Nueve tonos que se distinguen entre si sobre fondo oscuro y que siguen el
/// orden del canon, para que la rejilla se lea como una escala y no como un
/// mosaico al azar. Van solo en la franja: el fondo del bloque se queda neutro
/// para que seleccionado y hover no tengan que competir con nueve colores.
pub fn section_color(section: Section) -> Color {
    match section {
        Section::Pentateuco => Color::u_rgb(0x5f, 0xa8, 0x6a),
        Section::Historicos => Color::u_rgb(0xd7, 0x9a, 0x4a),
        Section::Poeticos => Color::u_rgb(0xd7, 0x5f, 0x6b),
        Section::ProfetasMayores => Color::u_rgb(0xa8, 0x7f, 0xd8),
        Section::ProfetasMenores => Color::u_rgb(0x7f, 0x8f, 0xd8),
        Section::Evangelios => Color::u_rgb(0x4a, 0xb5, 0xc4),
        Section::CartasPaulinas => Color::u_rgb(0x5f, 0xb0, 0x8a),
        Section::OtrasCartas => Color::u_rgb(0xc4, 0xb0, 0x4a),
        Section::Apocalipsis => Color::u_rgb(0xd8, 0x7f, 0x4a),
    }
}

const INK: Color = Color::u_rgb(0xe8, 0xe8, 0xe8);
const MUTED: Color = Color::u_rgb(0x99, 0x99, 0xaa);
const ACCENT: Color = Color::u_rgb(0xd7, 0xaf, 0x5f);
const VERSE: Color = Color::u_rgb(0x7f, 0xa8, 0xd8);
const ERROR: Color = Color::u_rgb(0xd7, 0x5f, 0x5f);
const PANEL_BG: Color = Color::u_rgb(0x2a, 0x2a, 0x35);

impl Theme {
    /// Respeta el fondo del emulador y solo colorea texto y barras.
    pub fn terminal() -> Self {
        let line = Color::u_rgb(0x54, 0x54, 0x66);

        Self {
            base: Surface {
                bg: TRANSPARENT,
                fg: INK,
                border: line,
            },
            panel: Surface {
                bg: PANEL_BG,
                fg: INK,
                border: line,
            },
            card: Surface {
                bg: TRANSPARENT,
                fg: INK,
                border: ACCENT,
            },
            item: Interactive {
                normal: Surface {
                    bg: TRANSPARENT,
                    fg: INK,
                    border: TRANSPARENT,
                },
                hover: Surface {
                    bg: TRANSPARENT,
                    fg: INK,
                    border: TRANSPARENT,
                },
                focused: Surface {
                    bg: TRANSPARENT,
                    fg: ACCENT,
                    border: TRANSPARENT,
                },
                selected: Surface {
                    bg: Color::u_rgb(0x3a, 0x3a, 0x4a),
                    fg: ACCENT,
                    border: TRANSPARENT,
                },
            },
            accent: ACCENT,
            muted: MUTED,
            verse_number: VERSE,
            error: ERROR,
        }
    }

    /// En la ventana si se pintan todos los fondos.
    pub fn window() -> Self {
        let line = Color::u_rgb(0x3c, 0x3c, 0x4a);

        Self {
            base: Surface {
                bg: Color::u_rgb(0x1c, 0x1c, 0x24),
                fg: INK,
                border: line,
            },
            panel: Surface {
                bg: PANEL_BG,
                fg: INK,
                border: line,
            },
            card: Surface {
                bg: Color::u_rgb(0x24, 0x24, 0x2e),
                fg: INK,
                border: ACCENT,
            },
            item: Interactive {
                normal: Surface {
                    bg: TRANSPARENT,
                    fg: INK,
                    border: TRANSPARENT,
                },
                hover: Surface {
                    bg: Color::u_rgb(0x2f, 0x2f, 0x3c),
                    fg: INK,
                    border: TRANSPARENT,
                },
                focused: Surface {
                    bg: Color::u_rgb(0x33, 0x33, 0x42),
                    fg: INK,
                    border: ACCENT,
                },
                selected: Surface {
                    bg: Color::u_rgb(0x3d, 0x35, 0x22),
                    fg: ACCENT,
                    border: ACCENT,
                },
            },
            accent: ACCENT,
            muted: MUTED,
            verse_number: VERSE,
            error: ERROR,
        }
    }

    /// Alto contraste para proyectar: fondo negro y texto claro, que es lo que
    /// se lee a distancia y lo que menos molesta si el proyector queda encendido.
    pub fn projector() -> Self {
        let black = Color::u_rgb(0x00, 0x00, 0x00);

        Self {
            base: Surface {
                bg: black,
                fg: Color::u_rgb(0xff, 0xff, 0xff),
                border: black,
            },
            panel: Surface {
                bg: black,
                fg: Color::u_rgb(0xff, 0xff, 0xff),
                border: black,
            },
            card: Surface {
                bg: black,
                fg: Color::u_rgb(0xff, 0xff, 0xff),
                border: black,
            },
            item: Interactive {
                normal: Surface {
                    bg: black,
                    fg: Color::u_rgb(0xff, 0xff, 0xff),
                    border: black,
                },
                hover: Surface {
                    bg: black,
                    fg: Color::u_rgb(0xff, 0xff, 0xff),
                    border: black,
                },
                focused: Surface {
                    bg: black,
                    fg: Color::u_rgb(0xff, 0xff, 0xff),
                    border: black,
                },
                selected: Surface {
                    bg: black,
                    fg: Color::u_rgb(0xff, 0xff, 0xff),
                    border: black,
                },
            },
            accent: Color::u_rgb(0xe8, 0xc9, 0x8a),
            muted: Color::u_rgb(0xa0, 0xa0, 0xa0),
            verse_number: Color::u_rgb(0xe8, 0xc9, 0x8a),
            error: ERROR,
        }
    }
}
