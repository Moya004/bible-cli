pub mod grid;
pub mod guard;
pub mod input;
pub mod measure;
pub mod paint;

use std::io::{Result, Write};
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

use clay_layout::{Clay, math::Dimensions};
use termion::{cursor, terminal_size};

use crate::business::repositories::{BufferRepository, VerseRepository};
use crate::logic::buffer::Buffer;
use crate::ui::action::Flow;
use crate::ui::design::metrics::Metrics;
use crate::ui::design::space::Space;
use crate::ui::model::{Frame, Glyphs};
use crate::ui::state::AppState;
use crate::ui::design::theme::Theme;
use crate::ui::tui::grid::Grid;
use crate::ui::tui::guard::TerminalGuard;
use crate::ui::views::{self, Anchors};

/// Cada cuanto despierta el bucle cuando no hay teclas: lo justo para notar un
/// cambio de tamaño de la terminal sin consumir CPU.
const TICK: Duration = Duration::from_millis(50);

/// Interfaz de terminal. Una unidad de Clay es una celda.
pub fn run<V: VerseRepository, B: BufferRepository>(
    buffer: &mut Buffer,
    verses: &V,
    history: &B,
) -> Result<()> {
    let metrics = Metrics::TERMINAL;
    let theme = Theme::terminal();

    let mut guard = TerminalGuard::new()?;
    let keys = input::spawn_key_reader();

    let (mut width, mut height) = terminal_size()?;
    let mut clay = Clay::new(Dimensions::new(width as f32, height as f32));
    clay.set_measure_text_function(measure::measure);

    let mut state = AppState::new();
    let mut front = Grid::new(width, height);
    let mut back = Grid::new(width, height);
    let mut full_repaint = true;

    loop {
        match keys.recv_timeout(TICK) {
            Ok(key) => {
                if let Some(action) = input::action_for(key, metrics.scroll_step)
                    && state.dispatch(action, buffer, verses, history) == Flow::Quit
                {
                    break;
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }

        // Vacia las teclas que se hayan acumulado antes de dibujar, para que al
        // mantener una tecla pulsada no se pinte un fotograma por pulsacion.
        while let Ok(key) = keys.try_recv() {
            if let Some(action) = input::action_for(key, metrics.scroll_step)
                && state.dispatch(action, buffer, verses, history) == Flow::Quit
            {
                return finish(guard, &state);
            }
        }

        let size = terminal_size()?;
        if size != (width, height) {
            (width, height) = size;
            clay.set_layout_dimensions(Dimensions::new(width as f32, height as f32));
            front.resize(width, height);
            back.resize(width, height);
            full_repaint = true;
            state.dirty = true;
        }

        if !state.dirty && !full_repaint {
            continue;
        }
        state.dirty = false;

        let anchors = render(&mut clay, &mut back, &state, &metrics, &theme);

        // El cursor de la barra de entrada: su columna es el ancho del texto que
        // lo precede, medido con el mismo criterio que usa Clay.
        let field = clay.bounding_box(anchors.input_field);
        let prefix = measure::width(state.caret_prefix());

        if let Some(field) = field
            && state.ensure_caret_visible(prefix, field.width)
        {
            // Cambio el desplazamiento horizontal: el fotograma recien maquetado
            // ya no sirve, se vuelve a dibujar en la siguiente vuelta.
            state.dirty = true;
            continue;
        }

        if let (Some(content), Some(viewport)) = (
            clay.bounding_box(anchors.results.content),
            clay.bounding_box(anchors.results.viewport),
        ) {
            let inner = viewport.height - 2. * (metrics.border + metrics.space(Space::Xs)) as f32;
            state.set_scroll_extent(content.height, inner.max(0.));
        }

        let previous = (!full_repaint).then_some(&front);
        back.flush(previous, guard.output())?;
        full_repaint = false;

        if let Some(field) = field {
            let column = (field.x + prefix - state.h_scroll).round().max(0.) as u16;
            let row = field.y.round().max(0.) as u16;

            write!(
                guard.output(),
                "{}{}",
                cursor::Goto(column.min(width.saturating_sub(1)) + 1, row + 1),
                cursor::Show
            )?;
        }

        guard.output().flush()?;
        std::mem::swap(&mut front, &mut back);
    }

    finish(guard, &state)
}

/// Maqueta y pinta un fotograma, y devuelve los anclajes para medir despues.
fn render(
    clay: &mut Clay,
    grid: &mut Grid,
    state: &AppState,
    metrics: &Metrics,
    theme: &Theme,
) -> Anchors {
    // La arena de cadenas se construye antes de abrir la maquetacion y vive
    // hasta despues de pintar: Clay guarda punteros al texto, no copias.
    let frame = Frame::build(state, &Glyphs::TERMINAL);

    grid.clear();

    let mut scope = clay.begin::<(), ()>();
    let anchors = views::root(&mut scope, metrics, theme, &frame);

    let commands: Vec<_> = scope.end().collect();
    paint::paint(&commands, grid);

    anchors
}

/// Restaura la terminal y recien ahi informa de un error de salida, que dentro
/// de la pantalla alterna no se habria visto.
fn finish(guard: TerminalGuard, state: &AppState) -> Result<()> {
    drop(guard);

    if let Some(error) = &state.exit_error {
        eprintln!("{}", error);
    }

    Ok(())
}
