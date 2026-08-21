pub mod action;
pub mod channel;
pub mod frame;
pub mod state;
pub mod views;

use std::io::Result;

use clay_layout::{Clay, math::{Dimensions, Vector2}, text::TextConfig};
use raylib::consts::{MouseButton, TraceLogLevel};
use raylib::drawing::RaylibDraw;

use crate::business::repositories::VerseRepository;
use crate::ui::design::components::input_bar::keep_caret_visible;
use crate::ui::design::components::scroll_area::{clamp, follow};
use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::design::theme::Theme;
use crate::ui::gui::fonts::Fonts;
use crate::ui::gui::paint;
use crate::ui::operator::action::{Action, Clicks};
use crate::ui::operator::channel::Projector;
use crate::ui::operator::frame::Frame;
use crate::ui::operator::state::OperatorState;
use crate::ui::operator::views::{Anchors, Hit};

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
    let mut hit = Hit::default();
    let mut clicks = Clicks::default();
    let body = metrics.style(Role::Body);

    while !rl.window_should_close() {
        // Clay resuelve el puntero contra la maquetacion anterior, asi que esto
        // va antes de abrir la nueva. Sin esta llamada Clay no sabe donde esta
        // el cursor y nada responde al raton.
        let pointer = rl.get_mouse_position();
        clay.pointer_state(
            Vector2::new(pointer.x, pointer.y),
            rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT),
        );

        let mut pending = action::actions(&mut rl, state.in_query);
        pending.extend(clicks.actions(&rl, hit.panel, hit.item, metrics.scroll_step));

        for action in pending {
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
            fit_columns(&mut state, &clay, anchors, fonts, &metrics);

            if let Some(field) = clay.bounding_box(anchors.query_field) {
                let prefix = fonts
                    .measure(state.caret_prefix(), body.font_id(), body.size)
                    .x;
                state.query_h_scroll =
                    keep_caret_visible(state.query_h_scroll, prefix, field.width);
            }
        }

        let frame = Frame::build(&state, hit);

        let mut scope = clay.begin::<(), ()>();
        let (new_anchors, new_hit) = views::root(&mut scope, &metrics, &theme, &frame);
        let render = scope.end().collect::<Vec<_>>();

        anchors = Some(new_anchors);
        hit = new_hit;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(paint::color(theme.base.bg));
        paint::paint(&mut d, fonts, &render);
    }

    projector.close();
    Ok(())
}

/// Recalcula cuantos bloques caben por fila en cada rejilla.
///
/// El ancho minimo del bloque de libros se **mide**, no se supone: se toma la
/// etiqueta mas larga con las tipografias ya cargadas. Asi ningun nombre se
/// puede recortar, sea cual sea el tamaño de la ventana o el cuerpo de letra.
///
/// El resultado vive en el estado y no solo en la vista porque `columns()` lo
/// necesita para decidir cuanto salta una flecha arriba: si los dos numeros no
/// coinciden, el teclado y lo dibujado dejan de estar de acuerdo.
fn fit_columns(
    state: &mut OperatorState,
    clay: &Clay,
    anchors: Anchors,
    fonts: &Fonts,
    metrics: &Metrics,
) {
    let title = metrics.style(Role::Title);
    let caption = metrics.style(Role::Caption);
    let gap = metrics.space(Space::Xs) as f32;
    let padding = 2. * metrics.space(Space::Xs) as f32;

    let widest = |texts: &mut dyn Iterator<Item = (&str, u16, u16)>| -> f32 {
        texts
            .map(|(text, id, size)| fonts.measure(text, id, size).x)
            .fold(0., f32::max)
    };

    if let Some(viewport) = clay.bounding_box(anchors.books.viewport) {
        let block = widest(
            &mut state
                .books
                .iter()
                .flat_map(|entry| {
                    [
                        (entry.abbreviation, title.font_id(), title.size),
                        (entry.name, caption.font_id(), caption.size),
                    ]
                })
                .map(|(text, id, size)| (text, id, size)),
        ) + padding;

        state.book_columns = columns_that_fit(viewport.width, block, gap);
    }

    if let Some(viewport) = clay.bounding_box(anchors.chapters.viewport) {
        // El peor caso son tres digitos: Salmos llega a 150. Se mide ese y no el
        // capitulo mas largo de este libro, o los bloques cambiarian de tamaño al
        // pasar de un libro corto a uno largo.
        let block = fonts.measure("150", title.font_id(), title.size).x + padding;

        state.chapter_columns = columns_that_fit(viewport.width, block, gap);
    }
}

fn columns_that_fit(available: f32, block: f32, gap: f32) -> usize {
    if block <= 0. {
        return 1;
    }

    (((available + gap) / (block + gap)).floor() as usize).max(1)
}

/// Corre cada panel hasta dejar visible su elemento elegido, y acota el
/// resultado a lo que hay de contenido.
///
/// Es lo que mantiene la seleccion a la vista al bajar con las flechas o al
/// saltar a una cita escrita; sin esto uno navega a ciegas apenas la lista pasa
/// de una pantalla.
fn follow_selection(state: &mut OperatorState, clay: &Clay, anchors: Anchors) {
    // Indice elegido de cada panel, en el orden de `anchors.selected`.
    let chosen = [state.book, state.chapter, state.verse, state.queue_index];

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

    for (panel, (ids, selected, offset)) in panels.into_iter().enumerate() {
        let (Some(content), Some(viewport)) = (
            clay.bounding_box(ids.content),
            clay.bounding_box(ids.viewport),
        ) else {
            continue;
        };

        // Solo se persigue la seleccion cuando cambia. Persiguiendola siempre,
        // la rueda no serviria de nada: la vista volveria sola al elemento
        // elegido en el fotograma siguiente y no se podria mirar el resto.
        if state.followed[panel] != Some(chosen[panel]) {
            if let Some(selected) = clay.bounding_box(selected) {
                *offset = follow(*offset, &selected, &viewport);
            }
            state.followed[panel] = Some(chosen[panel]);
        }

        *offset = clamp(*offset, content.height, viewport.height);
    }
}
