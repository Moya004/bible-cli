use std::io::BufReader;
use std::sync::mpsc::{Receiver, channel};
use std::thread;

use crate::ui::projector::protocol::{self, Command};

/// Lee las ordenes en un hilo aparte.
///
/// El bucle de dibujo de raylib no puede bloquearse esperando la entrada, asi
/// que las ordenes llegan por un canal y el bucle lo vacia una vez por
/// fotograma. Mismo arreglo que usa el lector de teclado de la terminal.
pub fn spawn() -> Receiver<Command> {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        for command in protocol::read(BufReader::new(std::io::stdin())) {
            if sender.send(command).is_err() {
                break;
            }
        }
    });

    receiver
}
