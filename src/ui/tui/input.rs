use std::sync::mpsc::{Receiver, channel};
use std::thread;

use termion::event::Key;
use termion::input::TermRead;

use crate::ui::action::Action;

/// Lee el teclado en un hilo aparte.
///
/// `stdin().keys()` bloquea, y el bucle principal necesita despertar aunque no
/// haya teclas para detectar cambios de tamaño de la ventana; por eso las teclas
/// llegan por un canal en vez de leerse directamente.
pub fn spawn_key_reader() -> Receiver<Key> {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        for key in std::io::stdin().keys() {
            let Ok(key) = key else { continue };

            if sender.send(key).is_err() {
                break;
            }
        }
    });

    receiver
}

/// Traduce una tecla a una intencion de la interfaz.
///
/// `scroll_step` viene de `Metrics`, que es quien sabe si una unidad es una
/// celda o un pixel.
pub fn action_for(key: Key, scroll_step: f32) -> Option<Action> {
    match key {
        Key::Char('\n') => Some(Action::Submit),
        Key::Char('\t') => None,
        Key::Char(c) => Some(Action::InsertChar(c)),
        Key::Backspace => Some(Action::Backspace),
        Key::Delete => Some(Action::Delete),
        Key::Left => Some(Action::CaretLeft),
        Key::Right => Some(Action::CaretRight),
        Key::Home => Some(Action::CaretStart),
        Key::End => Some(Action::CaretEnd),
        Key::Up => Some(Action::HistoryPrev),
        Key::Down => Some(Action::HistoryNext),
        Key::PageUp => Some(Action::Scroll(-scroll_step)),
        Key::PageDown => Some(Action::Scroll(scroll_step)),
        Key::Ctrl('c') => Some(Action::Quit),
        Key::Ctrl('l') => Some(Action::Redraw),
        _ => None,
    }
}
