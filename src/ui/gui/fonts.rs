use std::ffi::CString;

use raylib::consts::TextureFilter;
use raylib::ffi;
use raylib::math::Vector2;
use raylib::text::{RaylibFont, WeakFont};

use crate::ui::metrics::{FONT_BOLD, Metrics, Role};

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

/// Tipografias cargadas, indexadas por el par `(font_id, font_size)` que traen
/// las `TextConfig` de [`Metrics`].
///
/// Se guardan como `WeakFont` a proposito: no se descargan nunca. La funcion de
/// medida que Clay necesita tiene que ser `'static`, asi que las fuentes viven
/// lo que dura el programa.
pub struct Fonts {
    entries: Vec<(u16, u16, WeakFont)>,
}

impl Fonts {
    pub fn load(metrics: &Metrics) -> Result<Self, String> {
        let regular = pick("BIBLIA_FONT", REGULAR)?;
        let bold = pick("BIBLIA_FONT_BOLD", BOLD).unwrap_or_else(|_| regular.clone());

        let codepoints = codepoints();
        let mut entries: Vec<(u16, u16, WeakFont)> = Vec::new();

        for role in [Role::Title, Role::Body, Role::VerseNumber, Role::Label] {
            let style = metrics.style(role);

            if entries
                .iter()
                .any(|(id, size, _)| *id == style.font_id && *size == style.font_size)
            {
                continue;
            }

            let path = if style.font_id == FONT_BOLD {
                &bold
            } else {
                &regular
            };

            let font = load_font(path, style.font_size, &codepoints)
                .ok_or_else(|| format!("No se pudo cargar la tipografia {}", path))?;

            entries.push((style.font_id, style.font_size, font));
        }

        Ok(Self { entries })
    }

    pub fn get(&self, font_id: u16, font_size: u16) -> &WeakFont {
        self.entries
            .iter()
            .find(|(id, size, _)| *id == font_id && *size == font_size)
            .or_else(|| self.entries.first())
            .map(|(_, _, font)| font)
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
