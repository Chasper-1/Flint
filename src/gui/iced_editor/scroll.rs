//! Автоскролл курсора в видимую зону.
//!
//! При редактировании/навигации курсор может уйти за границу видимой
//! области (вверх или вниз). Этот модуль корректирует `scroll_y` так,
//! чтобы курсор оставался в видимом диапазоне `[0, viewport_height]`.
//!
//! ## Логика
//! 1. Вычислить Y-позицию и высоту строки курсора через `ShapedDocument`.
//! 2. Если `cursor_y < scroll_y` → `scroll_y = cursor_y` (скролл вверх).
//! 3. Если `cursor_y + line_height > scroll_y + viewport_height` →
//!    `scroll_y = cursor_y + line_height - viewport_height` (скролл вниз).

use crate::editor::render::ShapedDocument;

/// Откорректировать `scroll_y` так, чтобы строка с курсором была видна.
///
/// Возвращает новое значение `scroll_y`. Если курсор уже виден — вернёт
/// текущее `scroll_y` без изменений.
pub fn ensure_cursor_visible(
    scroll_y: f32,
    viewport_height: f32,
    shaped: &ShapedDocument,
    cursor_line: usize,
) -> f32 {
    if viewport_height <= 0.0 {
        return scroll_y;
    }

    // Вычисляем Y-позицию верхней границы строки курсора.
    let cursor_y = layout_line_y(shaped, cursor_line);
    let line_h = shaped.line_height(cursor_line);

    if cursor_y < scroll_y {
        // Курсор выше видимой области
        cursor_y
    } else if cursor_y + line_h > scroll_y + viewport_height {
        // Курсор ниже видимой области
        cursor_y + line_h - viewport_height
    } else {
        // Уже виден
        scroll_y
    }
}

/// Y-позиция (line_top) i-й строки.
fn layout_line_y(shaped: &ShapedDocument, line: usize) -> f32 {
    for run in shaped.buffer.layout_runs() {
        if run.line_i == line {
            return run.line_top;
        }
    }
    0.0
}
