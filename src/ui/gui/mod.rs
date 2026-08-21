pub mod fonts;
pub mod input;
pub mod paint;

use std::io::{Error, ErrorKind, Result};

use clay_layout::{Clay, math::Dimensions, text::TextConfig};
use raylib::drawing::RaylibDraw;

use crate::business::repositories::{BufferRepository, VerseRepository};
use crate::logic::buffer::Buffer;
use crate::ui::action::{Action, Flow};
use crate::ui::gui::fonts::Fonts;
use crate::ui::design::metrics::{Metrics, Role};
use crate::ui::design::space::Space;
use crate::ui::model::{Frame, Glyphs};
use crate::ui::state::AppState;
use crate::ui::design::theme::Theme;
use crate::ui::views::{self, Anchors};

const WINDOW_WIDTH: i32 = 960;
const WINDOW_HEIGHT: i32 = 680;

/// Interfaz de ventana. Una unidad de Clay es un pixel.
///
/// La maquetacion, el estado y las acciones son exactamente los mismos que en la
/// terminal; lo unico propio de aqui es de donde salen los eventos, como se mide
/// el texto y como se pinta.
pub fn run<V: VerseRepository, B: BufferRepository>(
    buffer: &mut Buffer,
    verses: &V,
    history: &B,
) -> Result<()> {
    let metrics = Metrics::WINDOW;
    let theme = Theme::window();

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("biblia-cli")
        .resizable()
        .build();

    rl.set_target_fps(60);

    // Las tipografias se cargan despues de abrir la ventana (necesitan el
    // contexto de OpenGL) y se filtran a `'static`: la funcion de medida que
    // Clay guarda tiene que serlo, y de todos modos viven todo el programa.
    let fonts: &'static Fonts = Box::leak(Box::new(
        Fonts::load(&metrics, &[]).map_err(|error| Error::new(ErrorKind::NotFound, error))?,
    ));

    let mut clay = Clay::new(Dimensions::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32));
    clay.set_measure_text_function(move |text: &str, config: &TextConfig| {
        let size = fonts.measure(text, config.font_id, config.font_size);
        Dimensions::new(size.x, config.line_height.max(1) as f32)
    });

    let mut state = AppState::new();
    let body = metrics.style(Role::Body);
    let mut anchors: Option<Anchors> = None;

    while !rl.window_should_close() {
        for action in input::actions(&mut rl, metrics.scroll_step) {
            if state.dispatch(action, buffer, verses, history) == Flow::Quit {
                return Ok(());
            }
        }

        clay.set_layout_dimensions(Dimensions::new(
            rl.get_screen_width() as f32,
            rl.get_screen_height() as f32,
        ));

        // Medidas del fotograma anterior. Clay conserva la tabla de la ultima
        // maquetacion, y consultarla ahora evita pedirsela mientras la tiene
        // prestada la maquetacion nueva; a 60 fps ese fotograma de retraso en el
        // cursor y en el tope del desplazamiento no se nota.
        let field = anchors.and_then(|anchors| clay.bounding_box(anchors.input_field));
        let prefix = fonts
            .measure(state.caret_prefix(), body.font_id(), body.size)
            .x;

        if let Some(field) = field {
            state.ensure_caret_visible(prefix, field.width);
        }

        if let Some((content, viewport)) = anchors.and_then(|anchors| {
            Some((
                clay.bounding_box(anchors.results.content)?,
                clay.bounding_box(anchors.results.viewport)?,
            ))
        }) {
            let inner = viewport.height - 2. * (metrics.border + metrics.space(Space::Xs)) as f32;
            state.set_scroll_extent(content.height, inner.max(0.));
        }

        let caret_visible = (rl.get_time() * 1.6) as i64 % 2 == 0;

        // La arena de cadenas vive hasta despues de pintar: Clay guarda punteros
        // al texto, no copias.
        let frame = Frame::build(&state, &Glyphs::WINDOW);

        let mut scope = clay.begin::<(), ()>();
        anchors = Some(views::root(&mut scope, &metrics, &theme, &frame));
        let commands: Vec<_> = scope.end().collect();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(paint::color(theme.base.bg));
        paint::paint(&mut d, fonts, &commands);

        if let Some(field) = field
            && caret_visible
        {
            d.draw_rectangle(
                (field.x + prefix - state.h_scroll) as i32,
                field.y as i32,
                2,
                body.line_height as i32,
                paint::color(theme.accent),
            );
        }
    }

    // Cerrar la ventana equivale a salir: hay que guardar el historico igual que
    // hace ^C en la terminal.
    state.dispatch(Action::Quit, buffer, verses, history);

    if let Some(error) = &state.exit_error {
        eprintln!("{}", error);
    }

    Ok(())
}
