use clay_layout::color::Color;

/// Color con alfa cero: le indica al pintor que no dibuje fondo y deje ver el
/// fondo propio de la terminal o de la ventana.
pub const TRANSPARENT: Color = Color::rgba(0., 0., 0., 0.);

pub struct Theme {
    /// Fondo de la aplicacion
    pub base: Color,
    /// Fondo de las barras superior e inferior
    pub panel: Color,
    /// Fondo de cada tarjeta de pasaje
    pub card: Color,
    /// Bordes de paneles y tarjetas
    pub border: Color,
    /// Texto normal
    pub text: Color,
    /// Texto secundario (traduccion, ayudas)
    pub muted: Color,
    /// Titulos de tarjeta y realces
    pub accent: Color,
    /// Numero de verso
    pub verse_number: Color,
    /// Mensajes de error en la barra de estado
    pub error: Color,
}

impl Theme {
    /// Paleta para la terminal: respeta el fondo del emulador y solo colorea el
    /// texto y las barras.
    pub const TERMINAL: Self = Self {
        base: TRANSPARENT,
        panel: Color::u_rgb(0x2a, 0x2a, 0x35),
        card: TRANSPARENT,
        border: Color::u_rgb(0x54, 0x54, 0x66),
        text: Color::u_rgb(0xe8, 0xe8, 0xe8),
        muted: Color::u_rgb(0x99, 0x99, 0xaa),
        accent: Color::u_rgb(0xd7, 0xaf, 0x5f),
        verse_number: Color::u_rgb(0x7f, 0xa8, 0xd8),
        error: Color::u_rgb(0xd7, 0x5f, 0x5f),
    };

    /// Paleta para la ventana: aqui si pintamos todos los fondos.
    pub const WINDOW: Self = Self {
        base: Color::u_rgb(0x1c, 0x1c, 0x24),
        panel: Color::u_rgb(0x2a, 0x2a, 0x35),
        card: Color::u_rgb(0x24, 0x24, 0x2e),
        border: Color::u_rgb(0x3c, 0x3c, 0x4a),
        text: Color::u_rgb(0xe8, 0xe8, 0xe8),
        muted: Color::u_rgb(0x99, 0x99, 0xaa),
        accent: Color::u_rgb(0xd7, 0xaf, 0x5f),
        verse_number: Color::u_rgb(0x7f, 0xa8, 0xd8),
        error: Color::u_rgb(0xd7, 0x5f, 0x5f),
    };
}
