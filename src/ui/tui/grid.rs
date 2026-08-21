use std::io::{Result, Write};

use clay_layout::color::Color;
use termion::{clear, cursor, style};
use unicode_width::UnicodeWidthChar;

use crate::ui::metrics::{ATTR_BOLD, ATTR_DIM};

/// Rectangulo en celdas. Los extremos `x1`/`y1` son exclusivos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl Rect {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    /// Convierte un rectangulo de Clay a celdas enteras.
    pub fn from_bounding_box(bbox: &clay_layout::math::BoundingBox) -> Self {
        Self::new(
            bbox.x.round() as i32,
            bbox.y.round() as i32,
            (bbox.x + bbox.width).round() as i32,
            (bbox.y + bbox.height).round() as i32,
        )
    }

    pub fn intersect(self, other: Self) -> Self {
        Self::new(
            self.x0.max(other.x0),
            self.y0.max(other.y0),
            self.x1.min(other.x1),
            self.y1.min(other.y1),
        )
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x0 && x < self.x1 && y >= self.y0 && y < self.y1
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub attrs: u16,
}

/// Color de primer plano por defecto de la terminal.
pub const DEFAULT_FG: Color = Color::rgba(0., 0., 0., 0.);
/// Fondo por defecto: alfa cero, o sea "no pintar y dejar el de la terminal".
pub const DEFAULT_BG: Color = Color::rgba(0., 0., 0., 0.);

impl Style {
    pub const DEFAULT: Self = Self {
        fg: DEFAULT_FG,
        bg: DEFAULT_BG,
        attrs: 0,
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub style: Style,
    /// Segunda mitad de un caracter de doble ancho: no se dibuja por si sola.
    pub continuation: bool,
}

impl Cell {
    const BLANK: Self = Self {
        ch: ' ',
        style: Style::DEFAULT,
        continuation: false,
    };
}

/// Lienzo de celdas.
///
/// Se mantienen dos: se dibuja sobre el de atras y al volcar solo se emiten las
/// celdas que cambiaron respecto al de adelante, que es lo que evita el parpadeo.
pub struct Grid {
    pub width: u16,
    pub height: u16,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::BLANK; width as usize * height as usize],
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.cells
            .resize(width as usize * height as usize, Cell::BLANK);
        self.clear();
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::BLANK);
    }

    pub fn bounds(&self) -> Rect {
        Rect::new(0, 0, self.width as i32, self.height as i32)
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        Some(y as usize * self.width as usize + x as usize)
    }

    /// Pinta el fondo de un rectangulo, componiendo si el color es traslucido.
    pub fn fill_rect(&mut self, rect: Rect, color: Color, clip: Rect) {
        if color.a <= 0. {
            return;
        }

        let area = rect.intersect(clip).intersect(self.bounds());

        for y in area.y0..area.y1 {
            for x in area.x0..area.x1 {
                let Some(index) = self.index(x, y) else {
                    continue;
                };
                let cell = &mut self.cells[index];
                cell.style.bg = blend(cell.style.bg, color);
                // Un fondo nuevo tapa lo que hubiera escrito debajo.
                cell.ch = ' ';
                cell.continuation = false;
            }
        }
    }

    pub fn set_char(&mut self, x: i32, y: i32, ch: char, style: Style, clip: Rect) {
        if !clip.contains(x, y) {
            return;
        }
        let Some(index) = self.index(x, y) else {
            return;
        };

        let background = blend(self.cells[index].style.bg, style.bg);

        self.cells[index] = Cell {
            ch,
            style: Style {
                bg: background,
                ..style
            },
            continuation: false,
        };
    }

    /// Escribe un texto a partir de `(x, y)`, respetando el recorte.
    ///
    /// Los caracteres de doble ancho ocupan dos celdas: la segunda queda marcada
    /// como continuacion para que el volcado no la dibuje por separado.
    pub fn put_str(&mut self, x: i32, y: i32, text: &str, style: Style, clip: Rect) {
        let mut cursor_x = x;

        for ch in text.chars() {
            let width = ch.width().unwrap_or(0) as i32;
            if width == 0 {
                continue;
            }

            self.set_char(cursor_x, y, ch, style, clip);

            if width > 1 {
                if let Some(index) = self.index(cursor_x + 1, y).filter(|_| {
                    clip.contains(cursor_x + 1, y) && clip.contains(cursor_x, y)
                }) {
                    self.cells[index] = Cell {
                        ch: ' ',
                        style,
                        continuation: true,
                    };
                }
            }

            cursor_x += width;

            if cursor_x >= clip.x1 {
                break;
            }
        }
    }

    /// Vuelca las diferencias contra `previous`. Con `previous` en `None` se
    /// redibuja todo, que es lo que hace falta tras un cambio de tamaño.
    pub fn flush(&self, previous: Option<&Grid>, out: &mut impl Write) -> Result<()> {
        let same_size = previous.is_some_and(|p| p.width == self.width && p.height == self.height);
        let previous = if same_size { previous } else { None };

        write!(out, "{}", cursor::Hide)?;

        if previous.is_none() {
            write!(out, "{}{}", style::Reset, clear::All)?;
        }

        let mut applied: Option<Style> = None;
        let mut pen: Option<(u16, u16)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y as usize * self.width as usize + x as usize;
                let cell = self.cells[index];

                if cell.continuation {
                    continue;
                }
                if previous.is_some_and(|p| p.cells[index] == cell) {
                    continue;
                }

                if pen != Some((x, y)) {
                    write!(out, "{}", cursor::Goto(x + 1, y + 1))?;
                }

                if applied != Some(cell.style) {
                    write_style(out, cell.style)?;
                    applied = Some(cell.style);
                }

                write!(out, "{}", cell.ch)?;

                let advance = cell.ch.width().unwrap_or(1).max(1) as u16;
                pen = Some((x + advance, y));
            }
        }

        write!(out, "{}", style::Reset)?;
        Ok(())
    }
}

fn write_style(out: &mut impl Write, style: Style) -> Result<()> {
    write!(out, "{}", termion::style::Reset)?;

    if style.attrs == ATTR_BOLD {
        write!(out, "{}", termion::style::Bold)?;
    } else if style.attrs == ATTR_DIM {
        write!(out, "{}", termion::style::Faint)?;
    }

    if style.fg.a > 0. {
        write!(
            out,
            "{}",
            termion::color::Fg(termion::color::Rgb(
                style.fg.r as u8,
                style.fg.g as u8,
                style.fg.b as u8
            ))
        )?;
    }

    if style.bg.a > 0. {
        write!(
            out,
            "{}",
            termion::color::Bg(termion::color::Rgb(
                style.bg.r as u8,
                style.bg.g as u8,
                style.bg.b as u8
            ))
        )?;
    }

    Ok(())
}

/// Compone `src` sobre `dst` usando el alfa de `src`.
pub fn blend(dst: Color, src: Color) -> Color {
    if src.a <= 0. {
        return dst;
    }
    if src.a >= 255. || dst.a <= 0. {
        return src;
    }

    let alpha = src.a / 255.;

    Color::rgba(
        dst.r + (src.r - dst.r) * alpha,
        dst.g + (src.g - dst.g) * alpha,
        dst.b + (src.b - dst.b) * alpha,
        dst.a.max(src.a),
    )
}
