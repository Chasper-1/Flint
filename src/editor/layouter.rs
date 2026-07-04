use crate::editor::markup::{SegmentStyle, parse_line};
use eframe::egui::text::{CCursorRange, CharIndex, LayoutJob};
use eframe::egui::{
    Color32, Context, FontFamily, FontId, Galley, Id, Stroke, TextEdit, TextFormat, Align,
};

fn append_compensated(
    job: &mut LayoutJob,
    left: usize,
    content: &str,
    right: usize,
    format: TextFormat,
) {
    if left > 0 {
        job.append(&"\u{200B}".repeat(left), 0.0, format.clone());
    }

    job.append(content, 0.0, format.clone());

    if right > 0 {
        job.append(&"\u{200B}".repeat(right), 0.0, format);
    }
}

pub fn render_line(
    job: &mut LayoutJob,
    line: &str,
    is_active: bool,
    base_size: f32,
    heading_size: f32,
    font_family: FontFamily,
    show_markup: bool,
) {
    if show_markup || is_active {
        let is_heading = line.starts_with("# ");
        let format = TextFormat::simple(
            FontId::new(
                if is_heading { heading_size } else { base_size },
                font_family,
            ),
            Color32::WHITE,
        );
        job.append(line, 0.0, format);
        return;
    }

    // Заголовок обрабатываем отдельно (если строка начинается с "# ")
    if line.starts_with("# ") {
        let content = &line[2..];
        let format = TextFormat::simple(
            FontId::new(heading_size, FontFamily::Proportional),
            Color32::WHITE,
        );
        append_compensated(
            job,
            2,
            content,
            0,
            format,
        );
        return;
    }

    // Получаем все сегменты
    let segments = parse_line(line);

    let default_format = TextFormat::simple(
        FontId::new(base_size, font_family.clone()),
        Color32::from_rgb(180, 180, 180),
    );

    // Один проход по сегментам – выводим каждый
    for seg in segments {
        let format = match seg.style {
            SegmentStyle::Plain => default_format.clone(),
            SegmentStyle::Bold => TextFormat::simple(
                FontId::new(base_size, font_family.clone()),
                Color32::from_rgb(255, 100, 100),
            ),
            SegmentStyle::Italic => {
                let mut f = TextFormat::simple(
                    FontId::new(base_size, font_family.clone()),
                    Color32::from_rgb(100, 200, 255),
                );
                f.italics = true;
                f
            }
            SegmentStyle::Strikethrough => {
                let mut f = TextFormat::simple(
                    FontId::new(base_size, font_family.clone()),
                    Color32::from_rgb(200, 150, 150),
                );
                f.strikethrough = Stroke::new(1.0, Color32::from_rgb(200, 150, 150));
                f
            }
            SegmentStyle::Superscript => {
                let mut f = TextFormat::simple(
                    FontId::new(base_size * 0.7, font_family.clone()),
                    Color32::from_rgb(150, 255, 150),
                );
                f.valign = Align::TOP;
                f
            }
            SegmentStyle::Subscript => {
                let mut f = TextFormat::simple(
                    FontId::new(base_size * 0.7, font_family.clone()),
                    Color32::from_rgb(255, 200, 100),
                );
                f.valign = Align::BOTTOM;
                f
            }
            SegmentStyle::Code => TextFormat::simple(
                FontId::new(base_size, FontFamily::Monospace),
                Color32::from_rgb(200, 200, 200),
            ),
        };

        // Добавляем компенсацию для маркеров
        if seg.left_marker_len > 0 {
            job.append(&"\u{200B}".repeat(seg.left_marker_len), 0.0, format.clone());
        }
        job.append(&seg.text, 0.0, format.clone());
        if seg.right_marker_len > 0 {
            job.append(&"\u{200B}".repeat(seg.right_marker_len), 0.0, format);
        }
    }
}

pub fn adjust_cursor_for_markup(
    ctx: &Context,
    id: Id,
    line_text: &str,
    right: bool,
    galley: &Galley,
) {
    if let Some(mut state) = TextEdit::load_state(ctx, id) {
        let offset = if line_text.starts_with("# ") {
            2
        } else {
            let segments = parse_line(line_text);

            segments
                .iter()
                .find(|s| !matches!(s.style, SegmentStyle::Plain))
                .map(|s| s.left_marker_len)
                .unwrap_or(0)
        };

        if offset == 0 {
            return;
        }

        if let Some(range) = state.cursor.char_range() {
            let mut c = range.primary;

            let current = c.index.0;
            let new_index = if right {
                current + offset
            } else {
                current.saturating_sub(offset)
            };

            c.index = CharIndex(new_index.min(galley.text().chars().count()));

            state.cursor.set_char_range(Some(CCursorRange::one(c)));
            state.store(ctx, id);
        }
    }
}
