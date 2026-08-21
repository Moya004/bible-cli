use raylib::RaylibHandle;
use raylib::consts::KeyboardKey;

use crate::ui::action::Action;

/// Recoge lo ocurrido en el fotograma y lo traduce a intenciones de la interfaz.
///
/// Es la contraparte de `ui::tui::input`: el estado resultante es el mismo, solo
/// cambia de donde vienen los eventos.
pub fn actions(rl: &mut RaylibHandle, scroll_step: f32) -> Vec<Action> {
    let mut actions = Vec::new();

    // `get_char_pressed` entrega codepoints ya resueltos por la distribucion de
    // teclado, asi que los acentos y la eñe entran sin tratamiento especial.
    while let Some(character) = rl.get_char_pressed() {
        actions.push(Action::InsertChar(character));
    }

    for (key, action) in [
        (KeyboardKey::KEY_ENTER, Action::Submit),
        (KeyboardKey::KEY_KP_ENTER, Action::Submit),
        (KeyboardKey::KEY_BACKSPACE, Action::Backspace),
        (KeyboardKey::KEY_DELETE, Action::Delete),
        (KeyboardKey::KEY_LEFT, Action::CaretLeft),
        (KeyboardKey::KEY_RIGHT, Action::CaretRight),
        (KeyboardKey::KEY_HOME, Action::CaretStart),
        (KeyboardKey::KEY_END, Action::CaretEnd),
        (KeyboardKey::KEY_UP, Action::HistoryPrev),
        (KeyboardKey::KEY_DOWN, Action::HistoryNext),
        (KeyboardKey::KEY_PAGE_UP, Action::Scroll(-scroll_step)),
        (KeyboardKey::KEY_PAGE_DOWN, Action::Scroll(scroll_step)),
    ] {
        // `is_key_pressed_repeat` cubre la repeticion al dejar la tecla apretada,
        // que es lo que se espera al borrar o al mover el cursor.
        if rl.is_key_pressed(key) || rl.is_key_pressed_repeat(key) {
            actions.push(action);
        }
    }

    let wheel = rl.get_mouse_wheel_move();
    if wheel != 0. {
        actions.push(Action::Scroll(-wheel * scroll_step));
    }

    actions
}
