pub mod fit;
pub mod protocol;
pub mod receiver;
pub mod view;

use std::io::Result;
use std::sync::mpsc::TryRecvError;

use clay_layout::{Clay, math::Dimensions, text::TextConfig};
use raylib::consts::TraceLogLevel;
use raylib::drawing::RaylibDraw;

use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::theme::Theme;
use crate::ui::gui::fonts::Fonts;
use crate::ui::gui::paint;
use crate::ui::projector::protocol::Command;
use crate::ui::projector::view::Slide;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

/// Ventana de proyeccion.
///
/// Proceso aparte porque raylib no abre dos ventanas: `InitWindow` es estado
/// global y el binding revienta al segundo intento. No toca la base de datos —
/// recibe el texto ya resuelto y solo dibuja — asi que si algo falla aca, el
/// operador sigue en pie.
pub fn run() -> Result<()> {
    let theme = Theme::projector();
    let base = Metrics::PROJECTOR;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("biblia · proyector")
        .resizable()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    rl.set_target_fps(60);
    // Cerrar con Esc es una mania de raylib, no una accion normal de ventana.
    // Un Esc perdido dejaria a la congregacion mirando el escritorio.
    rl.set_exit_key(None);

    let fonts: &'static Fonts = Box::leak(Box::new(
        Fonts::load(&base, &fit::LADDER)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::NotFound, error))?,
    ));

    let mut clay = Clay::new(Dimensions::new(WIDTH as f32, HEIGHT as f32));
    clay.set_measure_text_function(move |text: &str, config: &TextConfig| {
        let size = fonts.measure(text, config.font_id, config.font_size);
        Dimensions::new(size.x, config.line_height.max(1) as f32)
    });

    let commands = receiver::spawn();
    let mut slide = Slide::empty();

    while !rl.window_should_close() {
        loop {
            match commands.try_recv() {
                Ok(Command::Show { cite, text }) => {
                    slide.cite = cite;
                    slide.text = text;
                }
                Ok(Command::Blank(on)) => slide.blank = on,
                Ok(Command::Clear) => {
                    slide.cite.clear();
                    slide.text.clear();
                }
                // El operador cerro la tuberia: no hay nada mas que proyectar.
                Ok(Command::Quit) | Err(TryRecvError::Disconnected) => return Ok(()),
                Err(TryRecvError::Empty) => break,
            }
        }

        clay.set_layout_dimensions(Dimensions::new(
            rl.get_screen_width() as f32,
            rl.get_screen_height() as f32,
        ));

        // El cuerpo del verso se elige contra el alto real de la ventana, asi
        // que se recalcula por fotograma y sobrevive a cualquier cambio de
        // tamaño sin logica aparte.
        let available = fit::available_height(&base, rl.get_screen_height() as f32);
        let size = fit::choose(&mut clay, &base, &theme, &slide, available);
        let metrics = base.with_size(Role::Display, size, fit::line_height(size));

        let render = {
            let mut scope = clay.begin::<(), ()>();
            view::root(&mut scope, &metrics, &theme, &slide, available);
            scope.end().collect::<Vec<_>>()
        };

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(paint::color(theme.base.bg));
        paint::paint(&mut d, fonts, &render);
    }

    Ok(())
}
