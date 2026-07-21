//! Растеризация глифов в offscreen-RGBA изображение.
//!
//! Глифы переносятся из `cosmic_text::Buffer` в пиксельный буфер через
//! `SwashCache` (`buffer.draw`), композируясь поверх цвета фона по
//! пер-пиксельной альфе. Результат упаковывается в `iced::image::Handle`
//! и выводится виджетом одним `draw_image`.

use iced::advanced::image::Handle;
use iced::{Rectangle, Size};

use crate::editor::font;
use crate::editor::theme::color::rgba_to_u8;
use crate::gui::iced_editor::EditorInner;

/// Пересобрать растр текста (если `force` или изменился размер вьюпорта).
///
/// Растр пересобирается только при реальном изменении (грязный контент,
/// скролл или ресайз), чтобы не растеризовать на каждый кадр мигания
/// курсора.
pub fn ensure_raster(inner: &EditorInner, bounds: Rectangle, force: bool) {
    let w = bounds.width.max(1.0) as u32;
    let h = bounds.height.max(1.0) as u32;

    let size_changed = {
        let r = inner.raster.borrow();
        match &*r {
            Some((_, rw, rh)) => *rw != w || *rh != h,
            None => true,
        }
    };
    if !force && !size_changed {
        return;
    }

    let scroll_y = inner.scroll_y.get();
    let theme = &inner.theme;
    let (br, bg, bb, _) = rgba_to_u8(&theme.background);

    let mut pixels: Vec<u8> = vec![0u8; (w * h * 4) as usize];
    for p in pixels.chunks_mut(4) {
        p[0] = br;
        p[1] = bg;
        p[2] = bb;
        p[3] = 255;
    }

    let default_color = cosmic_text::Color::rgba(
        (theme.text.color.r * 255.0) as u8,
        (theme.text.color.g * 255.0) as u8,
        (theme.text.color.b * 255.0) as u8,
        (theme.text.color.a * 255.0) as u8,
    );

    let mut shaped = inner.shaped_doc.borrow_mut();
    font::with_font_and_cache(|fs, sc| {
        shaped.buffer.draw(fs, sc, default_color, |x, y, _w, _h, color| {
            let px = x;
            let py = y - scroll_y as i32;
            if px < 0 || py < 0 || px >= w as i32 || py >= h as i32 {
                return;
            }
            let (r, g, b, a) = (color.r(), color.g(), color.b(), color.a());
            let alpha = a as f32 / 255.0;
            let idx = (py as u32 * w + px as u32) as usize * 4;
            let bg_r = pixels[idx] as f32;
            let bg_g = pixels[idx + 1] as f32;
            let bg_b = pixels[idx + 2] as f32;
            pixels[idx] = (r as f32 * alpha + bg_r * (1.0 - alpha)) as u8;
            pixels[idx + 1] = (g as f32 * alpha + bg_g * (1.0 - alpha)) as u8;
            pixels[idx + 2] = (b as f32 * alpha + bg_b * (1.0 - alpha)) as u8;
            pixels[idx + 3] = 255;
        });
    });

    let handle = Handle::from_rgba(w, h, pixels);
    *inner.raster.borrow_mut() = Some((handle, w, h));
}
