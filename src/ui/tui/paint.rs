use clay_layout::render_commands::{Border, RenderCommand, RenderCommandConfig};

use crate::ui::design::theme::TRANSPARENT;
use crate::ui::tui::grid::{Grid, Rect, Style};

/// Lleva los comandos de dibujo de Clay al lienzo de celdas.
///
/// Clay ya resolvio posiciones, ajuste de linea y recortes: aqui solo se
/// traduce cada comando a celdas. Las imagenes y los elementos personalizados no
/// se usan en esta interfaz y se ignoran.
pub fn paint(commands: &[RenderCommand<'_, (), ()>], grid: &mut Grid) {
    let mut clips = vec![grid.bounds()];

    for command in commands {
        let clip = *clips.last().unwrap_or(&grid.bounds());
        let rect = Rect::from_bounding_box(&command.bounding_box);

        match &command.config {
            RenderCommandConfig::Rectangle(rectangle) => {
                grid.fill_rect(rect, rectangle.color, clip)
            }

            RenderCommandConfig::Border(border) => draw_border(grid, rect, border, clip),

            RenderCommandConfig::Text(text) => grid.put_str(
                rect.x0,
                rect.y0,
                text.text,
                Style {
                    fg: text.color,
                    bg: TRANSPARENT,
                    attrs: text.font_id,
                },
                clip,
            ),

            RenderCommandConfig::ScissorStart() => clips.push(rect.intersect(clip)),

            RenderCommandConfig::ScissorEnd() => {
                if clips.len() > 1 {
                    clips.pop();
                }
            }

            RenderCommandConfig::None()
            | RenderCommandConfig::Image(_)
            | RenderCommandConfig::Custom(_) => {}
        }
    }
}

/// Dibuja el borde con caracteres de caja. Cualquier grosor mayor que cero se
/// traduce a una sola linea, que es lo mas fino que puede dibujar una terminal.
fn draw_border(grid: &mut Grid, rect: Rect, border: &Border, clip: Rect) {
    if border.color.a <= 0. || rect.x1 <= rect.x0 || rect.y1 <= rect.y0 {
        return;
    }

    let style = Style {
        fg: border.color,
        bg: TRANSPARENT,
        attrs: 0,
    };

    let (left, right) = (rect.x0, rect.x1 - 1);
    let (top, bottom) = (rect.y0, rect.y1 - 1);

    let has = &border.width;

    if has.top > 0 {
        for x in left..=right {
            grid.set_char(x, top, '─', style, clip);
        }
    }
    if has.bottom > 0 {
        for x in left..=right {
            grid.set_char(x, bottom, '─', style, clip);
        }
    }
    if has.left > 0 {
        for y in top..=bottom {
            grid.set_char(left, y, '│', style, clip);
        }
    }
    if has.right > 0 {
        for y in top..=bottom {
            grid.set_char(right, y, '│', style, clip);
        }
    }

    // Las esquinas solo se dibujan donde se juntan dos lados.
    if has.top > 0 && has.left > 0 {
        grid.set_char(left, top, '┌', style, clip);
    }
    if has.top > 0 && has.right > 0 {
        grid.set_char(right, top, '┐', style, clip);
    }
    if has.bottom > 0 && has.left > 0 {
        grid.set_char(left, bottom, '└', style, clip);
    }
    if has.bottom > 0 && has.right > 0 {
        grid.set_char(right, bottom, '┘', style, clip);
    }
}
