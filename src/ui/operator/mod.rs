pub mod action;
pub mod channel;
pub mod frame;
pub mod state;
pub mod views;

use std::io::Result;

use clay_layout::{Clay, math::Dimensions, text::TextConfig};
use raylib::consts::TraceLogLevel;
use raylib::drawing::RaylibDraw;

use crate::business::repositories::VerseRepository;
use crate::ui::design::components::input_bar::keep_caret_visible;
use crate::ui::design::components::scroll_area::{clamp, follow};
use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::theme::Theme;
use crate::ui::gui::fonts::Fonts;
use crate::ui::gui::paint;
use crate::ui::operator::action::Action;
use crate::ui::operator::channel::Projector;
use crate::ui::operator::frame::Frame;
use crate::ui::operator::state::OperatorState;
use crate::ui::operator::views::Anchors;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 780;

/// Ventana de control de la proyeccion.
///
/// Es la unica que toca la base de datos: resuelve el texto y se lo manda ya
/// hecho al proceso proyector, que solo dibuja.
pub fn run<V: VerseRepository>(verses: &V) -> Result<()> {
    let metrics = Metrics::WINDOW;
    let theme = Theme::window();

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("biblia · operador")
        .resizable()
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    rl.set_target_fps(60);
    // Igual que en el proyector: no quiero que un Esc perdido cierre el
    // operador en medio de un servicio.
    rl.set_exit_key(None);

    let fonts: &'static Fonts = Box::leak(Box::new(
        Fonts::load(&metrics, &[])
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::NotFound, error))?,
    ));

    let mut clay = Clay::new(Dimensions::new(WIDTH as f32, HEIGHT as f32));
    clay.set_measure_text_function(move |text: &str, config: &TextConfig| {
        let size = fonts.measure(text, config.font_id, config.font_size);
        Dimensions::new(size.x, config.line_height.max(1) as f32)
    });

    let mut state = OperatorState::new(verses);
    let mut projector = Projector::new();
    let mut anchors: Option<Anchors> = None;
    let body = metrics.style(Role::Body);

    while !rl.window_should_close() {
        for action in action::actions(&mut rl, state.in_query) {
            if action == Action::Quit {
                projector.close();
                return Ok(());
            }

            for command in state.dispatch(action, verses) {
                if let Err(error) = projector.send(&command) {
                    state.status = error;
                }
            }
        }

        clay.set_layout_dimensions(Dimensions::new(
            rl.get_screen_width() as f32,
            rl.get_screen_height() as f32,
        ));

        // Medidas del fotograma anterior: consultarlas ahora evita pedirselas a
        // Clay mientras la maquetacion nueva lo tiene prestado, y a 60 fps ese
        // fotograma de retraso no se nota.
        if let Some(anchors) = anchors {
            follow_selection(&mut state, &clay, anchors);

            if let Some(field) = clay.bounding_box(anchors.query_field) {
                let prefix = fonts
                    .measure(state.caret_prefix(), body.font_id(), body.size)
                    .x;
                state.query_h_scroll =
                    keep_caret_visible(state.query_h_scroll, prefix, field.width);
            }
        }

        let frame = Frame::build(&state);

        let mut scope = clay.begin::<(), ()>();
        anchors = Some(views::root(&mut scope, &metrics, &theme, &frame));
        let render = scope.end().collect::<Vec<_>>();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(paint::color(theme.base.bg));
        paint::paint(&mut d, fonts, &render);
    }

    projector.close();
    Ok(())
}

/// Corre cada panel hasta dejar visible su elemento elegido, y acota el
/// resultado a lo que hay de contenido.
///
/// Es lo que mantiene la seleccion a la vista al bajar con las flechas o al
/// saltar a una cita escrita; sin esto uno navega a ciegas apenas la lista pasa
/// de una pantalla.
fn follow_selection(state: &mut OperatorState, clay: &Clay, anchors: Anchors) {
    let panels = [
        (anchors.books, anchors.selected[0], &mut state.scroll.books),
        (
            anchors.chapters,
            anchors.selected[1],
            &mut state.scroll.chapters,
        ),
        (anchors.verses, anchors.selected[2], &mut state.scroll.verses),
        (anchors.queue, anchors.selected[3], &mut state.scroll.queue),
    ];

    for (ids, selected, offset) in panels {
        let (Some(content), Some(viewport)) = (
            clay.bounding_box(ids.content),
            clay.bounding_box(ids.viewport),
        ) else {
            continue;
        };

        if let Some(selected) = clay.bounding_box(selected) {
            *offset = follow(*offset, &selected, &viewport);
        }

        *offset = clamp(*offset, content.height, viewport.height);
    }
}
