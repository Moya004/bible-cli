use std::ffi::CString;

use raylib::consts::TextureFilter;
use raylib::ffi;
use raylib::math::Vector2;
use raylib::text::{RaylibFont, WeakFont};

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::typography::{self, Weight};

/// Rutas donde buscar una tipografia, en orden. Se puede saltar el descarte con
/// las variables `BIBLIA_FONT` y `BIBLIA_FONT_BOLD`.
const REGULAR: &[&str] = &[
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/noto/NotoSans-Regular.ttf",
    "/usr/share/fonts/TTF/NotoSans-Regular.ttf",
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
];

const BOLD: &[&str] = &[
    "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/noto/NotoSans-Bold.ttf",
    "/usr/share/fonts/TTF/NotoSans-Bold.ttf",
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/liberation/LiberationSans-Bold.ttf",
];

/// Espaciado entre letras que se le pasa a raylib al medir y al dibujar. Tiene
/// que ser el mismo en ambos sitios o el texto no cuadra con la maquetacion.
pub const SPACING: f32 = 0.5;

/// Tipografias cargadas, indexadas por grosor y cuerpo.
///
/// Cada cuerpo necesita su propio atlas rasterizado, asi que la clave es el par
/// `(Weight, size)`. El atenuado no entra aqui: en la ventana lo lleva el color,
/// no una tipografia distinta.
///
/// Se guardan como `WeakFont` a proposito: no se descargan nunca. La funcion de
/// medida que Clay necesita tiene que ser `'static`, asi que las fuentes viven
/// lo que dura el programa.
pub struct Fonts {
    entries: Vec<(Weight, u16, WeakFont)>,
}

impl Fonts {
    /// Carga lo que piden los papeles de `metrics`, mas los cuerpos de `extra`
    /// en ambos grosores. `extra` es para el proyector, que elige el cuerpo del
    /// verso en cada fotograma y necesita una escalera donde escoger.
    pub fn load(metrics: &Metrics, extra: &[u16]) -> Result<Self, String> {
        let regular = pick("BIBLIA_FONT", REGULAR)?;
        let bold = pick("BIBLIA_FONT_BOLD", BOLD).unwrap_or_else(|_| regular.clone());

        let codepoints = codepoints();
        let mut fonts = Self {
            entries: Vec::new(),
        };

        let roles = [
            Role::Display,
            Role::Title,
            Role::Heading,
            Role::Body,
            Role::VerseNumber,
            Role::Label,
        ];

        for role in roles {
            let style = metrics.style(role);
            fonts.ensure(style.style.weight, style.size, &regular, &bold, &codepoints)?;
        }

        for size in extra {
            for weight in [Weight::Regular, Weight::Bold] {
                fonts.ensure(weight, *size, &regular, &bold, &codepoints)?;
            }
        }

        Ok(fonts)
    }

    fn ensure(
        &mut self,
        weight: Weight,
        size: u16,
        regular: &str,
        bold: &str,
        codepoints: &[i32],
    ) -> Result<(), String> {
        if size == 0 || self.find(weight, size).is_some() {
            return Ok(());
        }

        let path = if matches!(weight, Weight::Bold) {
            bold
        } else {
            regular
        };

        let font = load_font(path, size, codepoints)
            .ok_or_else(|| format!("No se pudo cargar la tipografia {}", path))?;

        self.entries.push((weight, size, font));
        Ok(())
    }

    fn find(&self, weight: Weight, size: u16) -> Option<&WeakFont> {
        self.entries
            .iter()
            .find(|(w, s, _)| *w == weight && *s == size)
            .map(|(_, _, font)| font)
    }

    /// Resuelve el `font_id` que trajo Clay a una tipografia concreta.
    pub fn get(&self, font_id: u16, font_size: u16) -> &WeakFont {
        let weight = typography::decode(font_id).weight;

        self.find(weight, font_size)
            .or_else(|| self.find(Weight::Regular, font_size))
            .or_else(|| self.entries.first().map(|(_, _, font)| font))
            .expect("siempre se carga al menos una tipografia")
    }

    pub fn measure(&self, text: &str, font_id: u16, font_size: u16) -> Vector2 {
        if text.is_empty() {
            return Vector2::new(0., font_size as f32);
        }

        self.get(font_id, font_size)
            .measure_text(text, font_size as f32, SPACING)
    }
}

fn pick(variable: &str, candidates: &[&str]) -> Result<String, String> {
    if let Ok(path) = std::env::var(variable)
        && std::path::Path::new(&path).is_file()
    {
        return Ok(path);
    }

    candidates
        .iter()
        .find(|path| std::path::Path::new(path).is_file())
        .map(|path| path.to_string())
        .ok_or_else(|| {
            format!(
                "No se encontro ninguna tipografia. Indica una con {}=/ruta/a/fuente.ttf",
                variable
            )
        })
}

/// Carga una fuente para un tamaño concreto.
///
/// No se usa `RaylibHandle::load_font_ex`: esa funcion le pasa a raylib el
/// *largo en bytes* de la cadena de caracteres como si fuera el numero de
/// codepoints, y con acentos de por medio ambos numeros no coinciden. Aqui se
/// arma el arreglo de codepoints a mano.
fn load_font(path: &str, size: u16, codepoints: &[i32]) -> Option<WeakFont> {
    let c_path = CString::new(path).ok()?;
    let mut codepoints = codepoints.to_vec();

    let raw = unsafe {
        ffi::LoadFontEx(
            c_path.as_ptr(),
            size as i32,
            codepoints.as_mut_ptr(),
            codepoints.len() as i32,
        )
    };

    if raw.glyphs.is_null() || raw.texture.id == 0 {
        return None;
    }

    // Sin filtro bilineal los bordes de las letras quedan dentados.
    unsafe {
        ffi::SetTextureFilter(
            raw.texture,
            TextureFilter::TEXTURE_FILTER_BILINEAR as i32,
        );
    }

    Some(unsafe { WeakFont::from_raw(raw) })
}

/// Juego de caracteres a rasterizar: ASCII imprimible, el suplemento Latin-1
/// (donde viven los acentos y la eñe del texto biblico) y la puntuacion que usa
/// la interfaz.
fn codepoints() -> Vec<i32> {
    let mut points: Vec<i32> = (0x20..0x7F).collect();
    points.extend(0xA0..=0xFF);
    points.extend("·«»—–…‹›“”‘’↑↓".chars().map(|c| c as i32));
    points
}
