use std::io::{Result, Stdout, Write, stdout};
use std::sync::Once;

use termion::cursor;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

/// Secuencias minimas para devolver la terminal a un estado usable: salir de la
/// pantalla alterna, mostrar el cursor y limpiar atributos.
const RESTORE: &str = "\x1b[?1049l\x1b[?25h\x1b[0m";

static PANIC_HOOK: Once = Once::new();

/// Toma la terminal (modo directo, pantalla alterna, cursor oculto) y la
/// devuelve intacta al soltarse, tanto en una salida normal como en un panico.
pub struct TerminalGuard {
    output: AlternateScreen<RawTerminal<Stdout>>,
}

impl TerminalGuard {
    pub fn new() -> Result<Self> {
        install_panic_hook();

        let mut output = stdout().into_raw_mode()?.into_alternate_screen()?;
        write!(output, "{}", cursor::Hide)?;
        output.flush()?;

        Ok(Self { output })
    }

    pub fn output(&mut self) -> &mut impl Write {
        &mut self.output
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = write!(self.output, "{}", cursor::Show);
        let _ = self.output.flush();
        // Al soltarse, `AlternateScreen` vuelve a la pantalla normal y
        // `RawTerminal` restaura el modo de linea.
    }
}

/// Deja la pantalla alterna *antes* de que el gestor de panicos por defecto
/// imprima: si no, el mensaje se escribe en el lienzo alterno y desaparece con
/// el, dejando al usuario sin ninguna pista de lo ocurrido.
fn install_panic_hook() {
    PANIC_HOOK.call_once(|| {
        let previous = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |info| {
            let mut out = stdout();
            let _ = out.write_all(RESTORE.as_bytes());
            let _ = out.flush();
            previous(info);
        }));
    });
}
