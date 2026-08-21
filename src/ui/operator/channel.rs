use std::io::ErrorKind;
use std::process::{Child, Command as Process, Stdio};

use crate::ui::projector::protocol::{self, Command};

/// Cuantas veces seguidas se acepta que el proyector muera al abrirse antes de
/// dejar de reintentar. Sin freno, un proyector que revienta al arrancar se
/// convierte en un ciclo infinito de procesos.
const MAX_FAILURES: u8 = 3;

/// Canal hacia el proceso proyector.
///
/// Se lanza a demanda y se relanza solo: si alguien cierra la ventana de
/// proyeccion, el siguiente envio la vuelve a abrir y reenvia lo que se estaba
/// mandando. En vivo nadie quiere confirmar un dialogo.
pub struct Projector {
    child: Option<Child>,
    failures: u8,
}

impl Projector {
    pub fn new() -> Self {
        Self {
            child: None,
            failures: 0,
        }
    }

    /// Manda una orden, abriendo o reabriendo el proyector si hace falta.
    pub fn send(&mut self, command: &Command) -> Result<(), String> {
        if self.write(command).is_ok() {
            self.failures = 0;
            return Ok(());
        }

        // La tuberia se corto: el proceso ya no esta.
        self.child = None;

        if self.failures >= MAX_FAILURES {
            return Err(String::from(
                "El proyector no se mantiene abierto; revisa la consola",
            ));
        }

        self.failures += 1;
        self.spawn()?;
        self.write(command)
            .map_err(|error| format!("No se pudo enviar al proyector: {}", error))
    }

    /// Cierra el proyector con educacion, y si no responde lo mata.
    pub fn close(&mut self) {
        if self.child.is_some() {
            let _ = self.write(&Command::Quit);
        }

        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }

    fn write(&mut self, command: &Command) -> std::io::Result<()> {
        let Some(child) = self.child.as_mut() else {
            return Err(std::io::Error::new(ErrorKind::BrokenPipe, "sin proyector"));
        };
        let Some(stdin) = child.stdin.as_mut() else {
            return Err(std::io::Error::new(ErrorKind::BrokenPipe, "sin tuberia"));
        };

        protocol::write(stdin, command)
    }

    fn spawn(&mut self) -> Result<(), String> {
        let executable = std::env::current_exe()
            .map_err(|error| format!("No se encontro el ejecutable: {}", error))?;

        let child = Process::new(executable)
            .arg("--projector")
            .stdin(Stdio::piped())
            // La salida del hijo se hereda: sus errores aparecen en la misma
            // consola desde donde se lanzo el operador.
            .spawn()
            .map_err(|error| format!("No se pudo abrir el proyector: {}", error))?;

        self.child = Some(child);
        Ok(())
    }
}

impl Drop for Projector {
    fn drop(&mut self) {
        self.close();
    }
}
