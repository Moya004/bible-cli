use std::io::{BufRead, Result, Write};

use serde::{Deserialize, Serialize};

/// Ordenes que el operador le manda al proyector.
///
/// Es lo unico que comparten los dos procesos, asi que conviene tratarlo como
/// una frontera y no dejar que se llene de detalles de ninguno de los dos lados.
/// Va en JSON, una orden por linea: el texto biblico trae acentos, comillas y
/// signos de todo tipo, y un escapado propio funcionaria con los 61.947 versos
/// hasta toparse con el que no, en vivo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Command {
    /// Muestra un verso. `cite` va al pie, `text` ocupa la pantalla.
    Show { cite: String, text: String },
    /// Pantalla en negro sin perder lo que estaba puesto.
    Blank(bool),
    /// Deja la pantalla sin verso.
    Clear,
    /// Cierra el proyector.
    Quit,
}

/// Escribe una orden y la empuja de inmediato.
///
/// Sin el vaciado explicito la orden se queda en el buffer del operador y la
/// pantalla no cambia hasta la siguiente, que en vivo se ve como si la
/// aplicacion se hubiera colgado.
pub fn write(out: &mut impl Write, command: &Command) -> Result<()> {
    serde_json::to_writer(&mut *out, command)?;
    out.write_all(b"\n")?;
    out.flush()
}

/// Lee las ordenes de un flujo, descartando las lineas que no se entiendan.
///
/// Una linea corrupta no deberia tumbar la proyeccion: se ignora y se sigue.
pub fn read(input: impl BufRead) -> impl Iterator<Item = Command> {
    input
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| serde_json::from_str(&line).ok())
}
