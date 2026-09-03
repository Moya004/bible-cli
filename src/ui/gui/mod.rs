//! Cimientos de raylib compartidos por las dos ventanas.
//!
//! No es una interfaz: `operator` y `projector` traen su propio bucle, su propio
//! estado y sus propias vistas, y de aca solo sacan como se cargan y se miden las
//! tipografias (`fonts`) y como se pintan los comandos de Clay (`paint`).

pub mod fonts;
pub mod paint;
