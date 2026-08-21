use raylib::RaylibHandle;
use raylib::consts::KeyboardKey;

/// Panel que tiene el foco del teclado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Books,
    Chapters,
    Verses,
    Queue,
}

impl Panel {
    const ORDER: [Panel; 4] = [Panel::Books, Panel::Chapters, Panel::Verses, Panel::Queue];

    pub fn next(self) -> Self {
        let position = Self::ORDER.iter().position(|p| *p == self).unwrap_or(0);
        Self::ORDER[(position + 1) % Self::ORDER.len()]
    }

    pub fn previous(self) -> Self {
        let position = Self::ORDER.iter().position(|p| *p == self).unwrap_or(0);
        Self::ORDER[(position + Self::ORDER.len() - 1) % Self::ORDER.len()]
    }
}

/// Intenciones del operador.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    FocusNext,
    FocusPrevious,
    Up,
    Down,
    Left,
    Right,
    /// Manda a la pantalla lo que este seleccionado
    Send,
    /// Agrega la seleccion al final de la cola
    Enqueue,
    /// Avanza y retrocede en la cola, ya en vivo
    QueueNext,
    QueuePrevious,
    /// Alterna la pantalla en negro
    ToggleBlank,
    /// Entra y sale de la barra de consulta
    OpenQuery,
    CloseQuery,
    InsertChar(char),
    Backspace,
    SubmitQuery,
    Quit,
}

/// Traduce lo ocurrido en el fotograma a intenciones.
///
/// Con la consulta abierta el teclado cambia de sentido: las letras se escriben
/// en vez de navegar, asi que las teclas de panel quedan fuera mientras dure.
pub fn actions(rl: &mut RaylibHandle, in_query: bool) -> Vec<Action> {
    let mut actions = Vec::new();

    if in_query {
        while let Some(character) = rl.get_char_pressed() {
            actions.push(Action::InsertChar(character));
        }

        for (key, action) in [
            (KeyboardKey::KEY_BACKSPACE, Action::Backspace),
            (KeyboardKey::KEY_ENTER, Action::SubmitQuery),
            (KeyboardKey::KEY_KP_ENTER, Action::SubmitQuery),
            (KeyboardKey::KEY_ESCAPE, Action::CloseQuery),
        ] {
            if pressed(rl, key) {
                actions.push(action);
            }
        }

        return actions;
    }

    for (key, action) in [
        (KeyboardKey::KEY_TAB, Action::FocusNext),
        (KeyboardKey::KEY_UP, Action::Up),
        (KeyboardKey::KEY_DOWN, Action::Down),
        (KeyboardKey::KEY_LEFT, Action::Left),
        (KeyboardKey::KEY_RIGHT, Action::Right),
        (KeyboardKey::KEY_ENTER, Action::Send),
        (KeyboardKey::KEY_KP_ENTER, Action::Send),
        (KeyboardKey::KEY_SPACE, Action::QueueNext),
        (KeyboardKey::KEY_BACKSPACE, Action::QueuePrevious),
        (KeyboardKey::KEY_F1, Action::ToggleBlank),
    ] {
        if pressed(rl, key) {
            actions.push(action);
        }
    }

    // Los atajos que son un caracter se escuchan como caracter, nunca como
    // tecla.
    //
    // raylib identifica las teclas por su posicion fisica en un teclado US, no
    // por lo que escriben: `KEY_SLASH` es la tecla que en QWERTY lleva el `/`,
    // que en Dvorak escribe `z`, y la que si escribe `/` llega como
    // `KEY_LEFT_BRACKET`. Mirando el caracter, el atajo es el mismo simbolo en
    // cualquier distribucion. De paso no se dispara dos veces: una pulsacion
    // genera el evento de tecla *y* el de caracter.
    while let Some(character) = rl.get_char_pressed() {
        match character {
            '+' => actions.push(Action::Enqueue),
            '/' => actions.push(Action::OpenQuery),
            _ => {}
        }
    }

    let shift = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
        || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
    if shift && actions.contains(&Action::FocusNext) {
        actions.retain(|a| *a != Action::FocusNext);
        actions.push(Action::FocusPrevious);
    }

    // Con Control apretado no llega evento de caracter, asi que este si va por
    // posicion fisica: es la tecla que un teclado US rotula `Q`. Con la ventana
    // se puede cerrar igual desde el gestor de ventanas.
    if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) && pressed(rl, KeyboardKey::KEY_Q) {
        actions.push(Action::Quit);
    }

    actions
}

/// `is_key_pressed_repeat` cubre la repeticion al dejar la tecla apretada, que
/// es lo que se espera al recorrer una lista larga.
fn pressed(rl: &RaylibHandle, key: KeyboardKey) -> bool {
    rl.is_key_pressed(key) || rl.is_key_pressed_repeat(key)
}
