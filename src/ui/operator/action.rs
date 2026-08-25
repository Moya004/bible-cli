use raylib::RaylibHandle;
use raylib::consts::{KeyboardKey, MouseButton};

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
    /// Borra el caracter bajo el cursor
    Delete,
    /// Mueve el cursor dentro de la consulta
    CaretLeft,
    CaretRight,
    CaretStart,
    CaretEnd,
    SubmitQuery,
    /// Recorre el historico de consultas mientras se escribe
    HistoryPrevious,
    HistoryNext,
    /// Click: lleva el foco a ese panel y elige ese elemento
    Click { panel: Panel, index: Option<usize> },
    /// Doble click: proyecta lo elegido
    Activate { panel: Panel, index: usize },
    /// Rueda sobre un panel concreto, no sobre el que tiene el foco
    Scroll { panel: Panel, delta: f32 },
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
            (KeyboardKey::KEY_DELETE, Action::Delete),
            (KeyboardKey::KEY_LEFT, Action::CaretLeft),
            (KeyboardKey::KEY_RIGHT, Action::CaretRight),
            (KeyboardKey::KEY_HOME, Action::CaretStart),
            (KeyboardKey::KEY_END, Action::CaretEnd),
            (KeyboardKey::KEY_ENTER, Action::SubmitQuery),
            (KeyboardKey::KEY_KP_ENTER, Action::SubmitQuery),
            (KeyboardKey::KEY_ESCAPE, Action::CloseQuery),
            (KeyboardKey::KEY_UP, Action::HistoryPrevious),
            (KeyboardKey::KEY_DOWN, Action::HistoryNext),
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

/// Cuanto puede tardar el segundo click para que cuente como doble.
const DOUBLE_CLICK: f64 = 0.35;

/// Detecta el doble click, que raylib no trae hecho.
#[derive(Default)]
pub struct Clicks {
    last: Option<(Panel, usize, f64)>,
}

impl Clicks {
    /// Traduce lo que hace el raton sobre `hit` a intenciones.
    ///
    /// El desplazamiento va al panel bajo el puntero y no al que tiene el foco:
    /// es lo que uno espera de la rueda, y deja mirar una lista sin perder la
    /// seleccion de otra.
    pub fn actions(
        &mut self,
        rl: &RaylibHandle,
        panel: Option<Panel>,
        index: Option<usize>,
        scroll_step: f32,
    ) -> Vec<Action> {
        let mut actions = Vec::new();
        let Some(panel) = panel else {
            return actions;
        };

        let wheel = rl.get_mouse_wheel_move();
        if wheel != 0. {
            actions.push(Action::Scroll {
                panel,
                delta: -wheel * scroll_step,
            });
        }

        if !rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            return actions;
        }

        actions.push(Action::Click { panel, index });

        let Some(index) = index else {
            self.last = None;
            return actions;
        };

        let now = rl.get_time();
        let repeated = self
            .last
            .is_some_and(|(p, i, t)| p == panel && i == index && now - t <= DOUBLE_CLICK);

        if repeated {
            actions.push(Action::Activate { panel, index });
            // Se olvida el ultimo, o un tercer click encadenaria otro envio.
            self.last = None;
        } else {
            self.last = Some((panel, index, now));
        }

        actions
    }
}
