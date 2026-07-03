use crate::editor::layout::build;
use crate::editor::state::EditorState;

use gtk4::cairo::Context;
use gtk4::pango::{FontDescription, Layout};

pub struct Renderer;

impl Renderer {
    pub fn draw(cr: &Context, width: i32, height: i32, state: &EditorState) {
        let theme = state.get_theme();

        // Фон
        cr.set_source_rgb(
            theme.background.r as f64,
            theme.background.g as f64,
            theme.background.b as f64,
        );
        cr.paint().unwrap();

        // Рамка
        cr.set_source_rgb(
            theme.border_color.r as f64,
            theme.border_color.g as f64,
            theme.border_color.b as f64,
        );
        cr.set_line_width(theme.border_width as f64);
        cr.rectangle(0.5, 0.5, width as f64 - 1.0, height as f64 - 1.0);
        cr.stroke().unwrap();

        // ---------- ТЕКСТ ----------

        let pango_ctx = pangocairo::functions::create_context(cr);

        let lines = build(state);

        cr.set_source_rgb(1.0, 1.0, 1.0);

        for (index, line) in lines.iter().enumerate() {
            let layout = Layout::new(&pango_ctx);

            layout.set_text(&line.text);

            let mut font = FontDescription::new();
            font.set_family(&line.font_family);
            font.set_size((line.font_size as i32) * gtk4::pango::SCALE);

            layout.set_font_description(Some(&font));

            cr.move_to(line.x as f64, line.y as f64);

            pangocairo::functions::show_layout(cr, &layout);

            if index == state.cursor.line {
                cr.set_source_rgb(1.0, 1.0, 1.0);

                let layout = Layout::new(&pango_ctx);
                layout.set_text(&line.text);

                let mut font = FontDescription::new();
                font.set_family(&line.font_family);
                font.set_size((line.font_size as i32) * gtk4::pango::SCALE);
                layout.set_font_description(Some(&font));

                let byte_index = line
                    .text
                    .char_indices()
                    .nth(state.cursor.column)
                    .map(|(i, _)| i)
                    .unwrap_or(line.text.len());

                let pos = layout.index_to_pos(byte_index as i32);

                let cursor_x = line.x as f64 + (pos.x() as f64 / gtk4::pango::SCALE as f64);

                let (_, logical) = layout.extents();

                let cursor_y = line.y as f64 + (logical.y() as f64 / gtk4::pango::SCALE as f64);

                let cursor_h = logical.height() as f64 / gtk4::pango::SCALE as f64;

                cr.set_source_rgb(0.2, 0.8, 1.0);

                cr.set_line_width(2.0);

                cr.move_to(cursor_x, cursor_y);
                cr.line_to(cursor_x, cursor_y + cursor_h);

                cr.stroke().unwrap();
            }
        }
    }
}
