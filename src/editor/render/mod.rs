mod painter;

use std::sync::Arc;

use eframe::egui::text::{Galley, LayoutJob};
use eframe::egui::{Align, Color32, FontFamily, FontId, Stroke, TextFormat, Ui};

use crate::editor::cache::DocumentCache;
use crate::editor::layout;
use crate::editor::state::EditMode;
use crate::editor::theme::color::Rgba;
use crate::editor::theme::EditorTheme;

pub use painter::paint;

pub struct Galleys {
    pub galleys: Vec<Option<Arc<Galley>>>,
    pub total_height: f32,
}

impl Galleys {
    pub fn new() -> Self {
        Self {
            galleys: Vec::new(),
            total_height: 0.0,
        }
    }
}

pub fn build(
    galleys: &mut Galleys,
    content: &str,
    cache: &DocumentCache,
    mode: EditMode,
    active_line: usize,
    ui: &Ui,
    theme: &EditorTheme,
    base_size: f32,
    heading_size: f32,
) {
    let font_family_name = theme.text.font_family.as_deref();
    let font_family: FontFamily = font_family_name
        .map(|name| FontFamily::Name(Arc::from(name)))
        .unwrap_or(FontFamily::Proportional);

    let lines: Vec<&str> = content.split('\n').collect();
    let num_lines = lines.len();
    let mut new_galleys = Vec::with_capacity(num_lines);
    let mut total_height = 0.0;

    let mut line_start = 0usize;
    for (i, line) in lines.iter().enumerate() {
        let show_markers = match mode {
            EditMode::Source => true,
            EditMode::Preview => false,
            EditMode::LivePreview => i == active_line,
        };

        let job = if line.is_empty() {
            make_empty_job(base_size, &font_family)
        } else {
            let runs = layout::compute::compute_line_runs(
                line,
                line_start,
                cache.lines.get(i),
                base_size,
                heading_size,
                show_markers,
            );
            runs_to_job(&runs, &font_family, ui.available_width())
        };

        let galley = ui.fonts_mut(|f| f.layout_job(job));
        total_height += galley.size().y;
        new_galleys.push(Some(galley));

        line_start += line.len() + 1;
        if line_start > content.len() {
            line_start = content.len();
        }
    }

    galleys.galleys = new_galleys;
    galleys.total_height = total_height;
}

// —————— адаптер TextRun → egui LayoutJob ——————

/// Создать пустой job минимальной высоты (для пустых строк).
fn make_empty_job(base_size: f32, font_family: &FontFamily) -> LayoutJob {
    let mut job = LayoutJob::default();
    let fmt = TextFormat::simple(
        FontId::new(base_size, font_family.clone()),
        Color32::from_rgb(200, 200, 200),
    );
    job.append("\u{200B}", 0.0, fmt);
    job
}

/// Преобразовать [`TextRun`]ы в один [`LayoutJob`].
fn runs_to_job(runs: &[layout::TextRun], default_family: &FontFamily, available_width: f32) -> LayoutJob {
    let mut job = LayoutJob::default();

    for run in runs {
        let fmt = text_run_to_format(run, default_family);
        job.append(&run.text, 0.0, fmt);
    }

    // Display formula: центрирование
    if runs.iter().any(|r| r.style_flags & crate::editor::markup::segment::STYLE_DISPLAY_FORMULA != 0) {
        job.wrap.max_width = available_width;
        job.halign = Align::Center;
    }

    job
}

/// Сконвертировать [`TextRun`] в egui [`TextFormat`].
fn text_run_to_format(run: &layout::TextRun, default_family: &FontFamily) -> TextFormat {
    use crate::editor::markup::segment::{
        STYLE_CODE, STYLE_DELETION, STYLE_HIGHLIGHT, STYLE_INSERTION,
        STYLE_ITALIC, STYLE_STRIKETHROUGH, STYLE_SUBSCRIPT, STYLE_SUPERSCRIPT, STYLE_UNDERLINE,
    };

    let family = match &run.font_family {
        Some(f) if f == "monospace" => FontFamily::Monospace,
        Some(name) => FontFamily::Name(Arc::from(name.as_str())),
        None => default_family.clone(),
    };

    let color = rgba_to_color32(&run.color);
    let mut fmt = TextFormat::simple(FontId::new(run.size, family), color);

    let s = run.style_flags;
    if s & STYLE_ITALIC != 0 {
        fmt.italics = true;
    }
    if s & STYLE_STRIKETHROUGH != 0 {
        fmt.strikethrough = Stroke::new(1.0, color);
    }
    if s & STYLE_SUPERSCRIPT != 0 {
        fmt.valign = Align::TOP;
    }
    if s & STYLE_SUBSCRIPT != 0 {
        fmt.valign = Align::BOTTOM;
    }
    if s & STYLE_UNDERLINE != 0 {
        fmt.underline = Stroke::new(1.0, color);
    }
    if s & STYLE_HIGHLIGHT != 0 {
        fmt.background = Color32::from_rgba_unmultiplied(255, 255, 0, 40);
    }
    if s & STYLE_CODE != 0 {
        fmt.font_id = FontId::new(run.size, FontFamily::Monospace);
    }
    if s & STYLE_DELETION != 0 {
        fmt.strikethrough = Stroke::new(1.0, color);
    }
    if s & STYLE_INSERTION != 0 {
        // цвет уже установлен
    }

    fmt
}

/// Временная конвертация [`Rgba`] → egui [`Color32`].
/// Удалится при переезде на Iced.
fn rgba_to_color32(c: &Rgba) -> Color32 {
    Color32::from_rgba_unmultiplied(
        (c.r * 255.0) as u8,
        (c.g * 255.0) as u8,
        (c.b * 255.0) as u8,
        (c.a * 255.0) as u8,
    )
}
