use clay_layout::{
    color::Color,
    layout::Padding,
    text::{TextAlignment, TextConfig, TextElementConfig, TextElementConfigWrapMode},
};

use crate::ui::design::space::{Space, SpaceScale};
use crate::ui::design::typography::{self, Emphasis, TextStyle, Weight};

/// Papel semantico de un texto. Cada backend lo traduce a lo suyo: la terminal
/// a atributos, la ventana a una tipografia y un cuerpo en pixeles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Texto gigante del proyector
    Display,
    /// Titulo de tarjeta o de pasaje
    Title,
    /// Encabezado de panel
    Heading,
    /// Cuerpo del verso
    Body,
    /// Numero de verso
    VerseNumber,
    /// Texto secundario: traduccion, ayudas, estado
    Label,
    /// Letra menuda: el nombre completo bajo la abreviatura de un libro
    Caption,
}

const ROLE_COUNT: usize = 7;

#[derive(Debug, Clone, Copy)]
pub struct RoleStyle {
    pub style: TextStyle,
    pub size: u16,
    pub line_height: u16,
}

impl RoleStyle {
    const fn new(weight: Weight, emphasis: Emphasis, size: u16, line_height: u16) -> Self {
        Self {
            style: TextStyle { weight, emphasis },
            size,
            line_height,
        }
    }

    /// El `u16` que viaja por Clay hasta el pintor.
    pub const fn font_id(&self) -> u16 {
        typography::encode(self.style)
    }
}

/// Lados de una caja que llevan borde dibujado.
#[derive(Debug, Clone, Copy, Default)]
pub struct BorderSides {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl BorderSides {
    pub const NONE: Self = Self {
        left: false,
        right: false,
        top: false,
        bottom: false,
    };
    pub const ALL: Self = Self {
        left: true,
        right: true,
        top: true,
        bottom: true,
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

/// Medidas dependientes de la escala del backend.
///
/// En la terminal una unidad de Clay es una celda; en la ventana y en el
/// proyector, un pixel. Las vistas son las mismas y solo se parametrizan con
/// esta estructura.
#[derive(Debug, Clone, Copy)]
pub struct Metrics {
    roles: [RoleStyle; ROLE_COUNT],
    space: SpaceScale,
    /// Grosor de los bordes
    pub border: u16,
    /// Ancho de la columna del numero de verso
    pub number_column: f32,
    /// Cuanto avanza el desplazamiento con una pulsacion o un paso de rueda
    pub scroll_step: f32,
}

impl Metrics {
    /// Una unidad = una celda de terminal. Los cuerpos no significan nada aqui
    /// y van en cero; el alto de linea es una fila.
    pub const TERMINAL: Self = Self {
        roles: [
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 0, 1), // Display
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 0, 1), // Title
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 0, 1), // Heading
            RoleStyle::new(Weight::Regular, Emphasis::Normal, 0, 1), // Body
            RoleStyle::new(Weight::Regular, Emphasis::Dim, 0, 1), // VerseNumber
            RoleStyle::new(Weight::Regular, Emphasis::Dim, 0, 1), // Label
            RoleStyle::new(Weight::Regular, Emphasis::Dim, 0, 1), // Caption
        ],
        space: SpaceScale::TERMINAL,
        border: 1,
        number_column: 4.,
        scroll_step: 3.,
    };

    /// Una unidad = un pixel.
    pub const WINDOW: Self = Self {
        roles: [
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 32, 40), // Display
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 20, 28), // Title
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 15, 22), // Heading
            RoleStyle::new(Weight::Regular, Emphasis::Normal, 19, 27), // Body
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 14, 27), // VerseNumber
            RoleStyle::new(Weight::Regular, Emphasis::Normal, 14, 20), // Label
            RoleStyle::new(Weight::Regular, Emphasis::Normal, 10, 13), // Caption
        ],
        space: SpaceScale::WINDOW,
        border: 1,
        number_column: 34.,
        scroll_step: 48.,
    };

    /// Ventana de proyeccion. El cuerpo de `Display` es solo el punto de
    /// partida: se ajusta al alto disponible en cada fotograma.
    pub const PROJECTOR: Self = Self {
        roles: [
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 64, 80), // Display
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 32, 44), // Title
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 28, 38), // Heading
            RoleStyle::new(Weight::Regular, Emphasis::Normal, 48, 62), // Body
            RoleStyle::new(Weight::Bold, Emphasis::Normal, 32, 44), // VerseNumber
            RoleStyle::new(Weight::Regular, Emphasis::Normal, 28, 38), // Label
            RoleStyle::new(Weight::Regular, Emphasis::Normal, 22, 30), // Caption
        ],
        space: SpaceScale::PROJECTOR,
        border: 0,
        number_column: 64.,
        scroll_step: 96.,
    };

    pub const fn space(&self, space: Space) -> u16 {
        self.space.get(space)
    }

    pub const fn style(&self, role: Role) -> RoleStyle {
        self.roles[role as usize]
    }

    /// Copia con otro cuerpo para un papel. La usa el proyector para ajustar el
    /// verso al alto sin duplicar toda la tabla de medidas.
    pub fn with_size(mut self, role: Role, size: u16, line_height: u16) -> Self {
        self.roles[role as usize].size = size;
        self.roles[role as usize].line_height = line_height;
        self
    }

    pub fn padding(&self, x: Space, y: Space) -> Padding {
        let (x, y) = (self.space(x), self.space(y));
        Padding::new(x, x, y, y)
    }

    /// Relleno de una caja con borde.
    ///
    /// Clay dibuja el borde *dentro* del recuadro y sin ocupar espacio de
    /// maquetacion, asi que el contenido empieza en el mismo punto haya borde o
    /// no. Sobre un relleno de 16 pixeles un borde de 1 no se nota; en la
    /// terminal se come la fila entera y borra la primera linea de contenido.
    /// Por eso el grosor se suma al relleno de los lados que lo llevan.
    pub fn padding_in_border(&self, x: Space, y: Space, sides: BorderSides) -> Padding {
        let (x, y) = (self.space(x), self.space(y));

        Padding::new(
            x + if sides.left { self.border } else { 0 },
            x + if sides.right { self.border } else { 0 },
            y + if sides.top { self.border } else { 0 },
            y + if sides.bottom { self.border } else { 0 },
        )
    }

    /// Configuracion de texto de Clay para un papel y un color.
    ///
    /// Debe llamarse dentro del ambito de maquetacion: `TextConfig::end()`
    /// guarda la configuracion en la arena de Clay.
    pub fn text(&self, role: Role, color: Color) -> TextElementConfig {
        self.config(role, color, TextElementConfigWrapMode::Words)
    }

    /// Como [`Metrics::text`] pero sin ajuste de linea. La usan la barra de
    /// entrada y las etiquetas de una sola linea.
    pub fn text_no_wrap(&self, role: Role, color: Color) -> TextElementConfig {
        self.config(role, color, TextElementConfigWrapMode::None)
    }

    fn config(
        &self,
        role: Role,
        color: Color,
        wrap: TextElementConfigWrapMode,
    ) -> TextElementConfig {
        let style = self.style(role);

        TextConfig::new()
            .color(color)
            .font_id(style.font_id())
            .font_size(style.size)
            .line_height(style.line_height)
            .wrap_mode(wrap)
            .alignment(TextAlignment::Left)
            .end()
    }
}
