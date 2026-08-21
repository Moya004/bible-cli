use clay_layout::render_commands::{Border, RenderCommand, RenderCommandConfig};
use raylib::color::Color as RaylibColor;
use raylib::drawing::RaylibDraw;
use raylib::ffi;
use raylib::math::Vector2;

use crate::ui::gui::fonts::{Fonts, SPACING};

/// Lleva los comandos de dibujo de Clay a la ventana.
///
/// Es la contraparte de `ui::tui::paint`: los mismos comandos, pintados con
/// rectangulos y tipografias en vez de celdas.
pub fn paint(
    d: &mut impl RaylibDraw,
    fonts: &Fonts,
    commands: &[RenderCommand<'_, (), ()>],
) {
    for command in commands {
        let bbox = command.bounding_box;

        match &command.config {
            RenderCommandConfig::Rectangle(rectangle) => {
                if rectangle.corner_radii.top_left > 0. {
                    let shorter = bbox.width.min(bbox.height);
                    let roundness = if shorter > 0. {
                        (rectangle.corner_radii.top_left * 2.) / shorter
                    } else {
                        0.
                    };

                    d.draw_rectangle_rounded(
                        raylib::math::Rectangle::new(bbox.x, bbox.y, bbox.width, bbox.height),
                        roundness,
                        8,
                        color(rectangle.color),
                    );
                } else {
                    d.draw_rectangle(
                        bbox.x as i32,
                        bbox.y as i32,
                        bbox.width as i32,
                        bbox.height as i32,
                        color(rectangle.color),
                    );
                }
            }

            RenderCommandConfig::Border(border) => draw_border(d, &bbox, border),

            RenderCommandConfig::Text(text) => {
                d.draw_text_ex(
                    fonts.get(text.font_id, text.font_size),
                    text.text,
                    Vector2::new(bbox.x, bbox.y),
                    text.font_size as f32,
                    SPACING,
                    color(text.color),
                );
            }

            // Se usa la version cruda porque la envoltura segura cierra el
            // recorte al salir del ambito, y aqui el inicio y el fin llegan como
            // dos comandos separados.
            RenderCommandConfig::ScissorStart() => unsafe {
                ffi::BeginScissorMode(
                    bbox.x as i32,
                    bbox.y as i32,
                    bbox.width as i32,
                    bbox.height as i32,
                );
            },

            RenderCommandConfig::ScissorEnd() => unsafe {
                ffi::EndScissorMode();
            },

            RenderCommandConfig::None()
            | RenderCommandConfig::Image(_)
            | RenderCommandConfig::Custom(_) => {}
        }
    }
}

/// Los bordes de Clay van por dentro del recuadro, asi que cada lado es un
/// rectangulo pegado al canto correspondiente.
fn draw_border(d: &mut impl RaylibDraw, bbox: &clay_layout::math::BoundingBox, border: &Border) {
    if border.color.a <= 0. {
        return;
    }

    let tint = color(border.color);
    let width = &border.width;

    if width.left > 0 {
        d.draw_rectangle(
            bbox.x as i32,
            bbox.y as i32,
            width.left as i32,
            bbox.height as i32,
            tint,
        );
    }
    if width.right > 0 {
        d.draw_rectangle(
            (bbox.x + bbox.width - width.right as f32) as i32,
            bbox.y as i32,
            width.right as i32,
            bbox.height as i32,
            tint,
        );
    }
    if width.top > 0 {
        d.draw_rectangle(
            bbox.x as i32,
            bbox.y as i32,
            bbox.width as i32,
            width.top as i32,
            tint,
        );
    }
    if width.bottom > 0 {
        d.draw_rectangle(
            bbox.x as i32,
            (bbox.y + bbox.height - width.bottom as f32) as i32,
            bbox.width as i32,
            width.bottom as i32,
            tint,
        );
    }
}

pub fn color(color: clay_layout::color::Color) -> RaylibColor {
    RaylibColor::new(
        color.r as u8,
        color.g as u8,
        color.b as u8,
        color.a as u8,
    )
}
