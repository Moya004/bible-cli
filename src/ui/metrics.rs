use clay_layout::{
    color::Color,
    layout::Padding,
    text::{TextAlignment, TextConfig, TextElementConfig, TextElementConfigWrapMode},
};

/// Lados de una caja que llevan borde dibujado.
#[derive(Debug, Clone, Copy, Default)]
pub struct BorderSides {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl BorderSides {
    pub const ALL: Self = Self {
        left: true,
        right: true,
        top: true,
        bottom: true,
    };
    pub const NONE: Self = Self {
        left: false,
        right: false,
        top: false,
        bottom: false,
    };
    pub const TOP: Self = Self {
        top: true,
        ..Self::NONE
    };
    pub const LEFT: Self = Self {
        left: true,
        ..Self::NONE
    };
}

/// Papel semantico de un texto dentro de la interfaz. Cada backend lo traduce a
/// su propia nocion de tipografia: la terminal a atributos (negrita, tenue) y la
/// ventana a una fuente y un tamaño en pixeles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    /// Titulo de una tarjeta de pasaje, p.ej. "Juan 3:16"
    Title,
    /// Cuerpo del verso
    Body,
    /// Numero de verso
    VerseNumber,
    /// Texto secundario: traduccion activa, ayudas, estado
    Label,
}

/// Valores que Clay necesita para medir y colocar un texto.
///
/// `font_id` no es una fuente para Clay: es un `u16` libre que el pintor
/// interpreta. En la terminal selecciona atributos ANSI, en la ventana escoge
/// una de las fuentes cargadas.
#[derive(Debug, Clone, Copy)]
pub struct RoleStyle {
    pub font_id: u16,
    pub font_size: u16,
    pub line_height: u16,
}

/// Atributos de terminal codificados en `font_id`.
pub const ATTR_NORMAL: u16 = 0;
pub const ATTR_BOLD: u16 = 1;
pub const ATTR_DIM: u16 = 2;

/// Fuentes de la ventana codificadas en `font_id`.
pub const FONT_REGULAR: u16 = 0;
pub const FONT_BOLD: u16 = 1;

/// Medidas dependientes de la escala del backend.
///
/// En la terminal una unidad de Clay es una celda; en la ventana, un pixel. Las
/// vistas de `ui::views` son identicas en ambos modos y solo se parametrizan con
/// esta estructura.
pub struct Metrics {
    pub title: RoleStyle,
    pub body: RoleStyle,
    pub verse_number: RoleStyle,
    pub label: RoleStyle,
    /// Relleno horizontal de las barras y del panel de resultados
    pub pad_x: u16,
    /// Relleno vertical de las barras
    pub pad_y: u16,
    /// Relleno de una tarjeta de pasaje
    pub card_pad_x: u16,
    pub card_pad_y: u16,
    /// Separacion entre elementos de una misma fila o columna
    pub gap: u16,
    /// Separacion entre versos dentro de una tarjeta
    pub verse_gap: u16,
    /// Separacion entre tarjetas de pasaje
    pub card_gap: u16,
    /// Ancho de la columna del numero de verso
    pub number_column: f32,
    /// Grosor de los bordes
    pub border: u16,
    /// Cuanto avanza el scroll con una pulsacion o un paso de rueda
    pub scroll_step: f32,
}

impl Metrics {
    /// Una unidad = una celda de terminal.
    pub const TERMINAL: Self = Self {
        title: RoleStyle {
            font_id: ATTR_BOLD,
            font_size: 0,
            line_height: 1,
        },
        body: RoleStyle {
            font_id: ATTR_NORMAL,
            font_size: 0,
            line_height: 1,
        },
        verse_number: RoleStyle {
            font_id: ATTR_DIM,
            font_size: 0,
            line_height: 1,
        },
        label: RoleStyle {
            font_id: ATTR_DIM,
            font_size: 0,
            line_height: 1,
        },
        pad_x: 1,
        pad_y: 0,
        card_pad_x: 1,
        card_pad_y: 0,
        gap: 1,
        verse_gap: 0,
        card_gap: 1,
        number_column: 4.,
        border: 1,
        scroll_step: 3.,
    };

    /// Una unidad = un pixel.
    pub const WINDOW: Self = Self {
        title: RoleStyle {
            font_id: FONT_BOLD,
            font_size: 20,
            line_height: 28,
        },
        body: RoleStyle {
            font_id: FONT_REGULAR,
            font_size: 17,
            line_height: 24,
        },
        verse_number: RoleStyle {
            font_id: FONT_BOLD,
            font_size: 13,
            line_height: 24,
        },
        label: RoleStyle {
            font_id: FONT_REGULAR,
            font_size: 14,
            line_height: 20,
        },
        pad_x: 16,
        pad_y: 10,
        card_pad_x: 14,
        card_pad_y: 10,
        gap: 8,
        verse_gap: 4,
        card_gap: 12,
        number_column: 34.,
        border: 1,
        scroll_step: 48.,
    };

    /// Relleno de una caja con borde.
    ///
    /// Clay dibuja el borde *dentro* del recuadro y sin ocupar espacio de
    /// maquetacion, asi que el contenido empieza en el mismo punto haya borde o
    /// no. En pixeles un borde de 1 sobre un relleno de 16 no se nota; en la
    /// terminal se come la fila entera y borra la primera linea. Por eso el
    /// grosor del borde se suma al relleno de los lados que lo llevan.
    pub fn padding_in_border(&self, sides: BorderSides) -> Padding {
        Padding::new(
            self.pad_x + if sides.left { self.border } else { 0 },
            self.pad_x + if sides.right { self.border } else { 0 },
            self.pad_y + if sides.top { self.border } else { 0 },
            self.pad_y + if sides.bottom { self.border } else { 0 },
        )
    }

    pub fn style(&self, role: Role) -> RoleStyle {
        match role {
            Role::Title => self.title,
            Role::Body => self.body,
            Role::VerseNumber => self.verse_number,
            Role::Label => self.label,
        }
    }

    /// Construye la configuracion de texto de Clay para un papel y un color.
    ///
    /// Debe llamarse dentro del ambito de la maquetacion: `TextConfig::end()`
    /// guarda la configuracion en la arena de Clay.
    pub fn text(&self, role: Role, color: Color) -> TextElementConfig {
        let style = self.style(role);

        TextConfig::new()
            .color(color)
            .font_id(style.font_id)
            .font_size(style.font_size)
            .line_height(style.line_height)
            .wrap_mode(TextElementConfigWrapMode::Words)
            .alignment(TextAlignment::Left)
            .end()
    }

    /// Como [`Metrics::text`], pero sin ajuste de linea. Se usa en la barra de
    /// entrada, que desplaza horizontalmente en vez de partir el texto.
    pub fn text_no_wrap(&self, role: Role, color: Color) -> TextElementConfig {
        let style = self.style(role);

        TextConfig::new()
            .color(color)
            .font_id(style.font_id)
            .font_size(style.font_size)
            .line_height(style.line_height)
            .wrap_mode(TextElementConfigWrapMode::None)
            .alignment(TextAlignment::Left)
            .end()
    }
}
