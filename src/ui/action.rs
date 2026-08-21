/// Intenciones de la interfaz, independientes del backend que las origina.
///
/// Tanto el backend de terminal (teclas de `termion`) como el de ventana
/// (`raylib`) traducen sus eventos nativos a estas acciones, de modo que ambas
/// interfaces comparten el mismo comportamiento.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    /// Inserta un caracter en la posicion del cursor
    InsertChar(char),
    /// Borra el caracter anterior al cursor
    Backspace,
    /// Borra el caracter bajo el cursor
    Delete,
    /// Mueve el cursor un caracter a la izquierda
    CaretLeft,
    /// Mueve el cursor un caracter a la derecha
    CaretRight,
    /// Mueve el cursor al inicio de la entrada
    CaretStart,
    /// Mueve el cursor al final de la entrada
    CaretEnd,
    /// Carga la entrada anterior del historico
    HistoryPrev,
    /// Carga la entrada siguiente del historico
    HistoryNext,
    /// Desplaza el panel de resultados. Positivo baja, negativo sube.
    Scroll(f32),
    /// Ejecuta la consulta escrita en la entrada
    Submit,
    /// Fuerza un repintado completo
    Redraw,
    /// Guarda el historico y termina
    Quit,
}

/// Indica al bucle de eventos si debe continuar o terminar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flow {
    Continue,
    Quit,
}
