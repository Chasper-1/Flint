//! Вертикальная навигация с сохранением пиксельной X-позиции курсора.
//!
//! При перемещении вверх/вниз курсор сохраняет `col_visual` — пиксельную
//! координату X. Функции [`cursor_x_on_line`] и [`raw_at_x_on_line`]
//! конвертируют между пиксельной X и byte-смещением на строке, проходя
//! по глифам сформованного буфера.

use crate::editor::layout::cursor_line_bounds;
use crate::editor::render::ShapedDocument;
use crate::gui::iced_editor::EditorInner;

/// X-позиция курсора на строке `line` по глифам буфера.
pub fn cursor_x_on_line(shaped: &ShapedDocument, line: usize, byte_in_line: usize) -> f32 {
    for run in shaped.buffer.layout_runs() {
        if run.line_i != line {
            continue;
        }
        for glyph in run.glyphs.iter() {
            if glyph.start >= byte_in_line {
                return glyph.x;
            }
        }
        return run
            .glyphs
            .last()
            .map(|g| g.x + g.w)
            .unwrap_or(0.0);
    }
    0.0
}

/// Ближайший к `x` content-offset на строке `line`.
///
/// Пустая строка → начало строки. Иначе среди глифов и конца строки
/// выбирается точка с минимальным расстоянием по X.
pub fn raw_at_x_on_line(
    shaped: &ShapedDocument,
    line: usize,
    x: f32,
    line_start: usize,
    line_end: usize,
) -> usize {
    if line_end <= line_start {
        return line_start;
    }
    let mut best: Option<(f32, usize)> = None;
    for run in shaped.buffer.layout_runs() {
        if run.line_i != line {
            continue;
        }
        for glyph in run.glyphs.iter() {
            let dist = (glyph.x - x).abs();
            let cand = line_start + glyph.start;
            if best.map_or(true, |(bd, _)| dist < bd) {
                best = Some((dist, cand));
            }
        }
        if let Some(last) = run.glyphs.last() {
            let end_x = last.x + last.w;
            let dist = (end_x - x).abs();
            if best.map_or(true, |(bd, _)| dist < bd) {
                best = Some((dist, line_end));
            }
        }
        break;
    }
    best.map_or(line_start, |(_, c)| c)
}

/// Переместить курсор на строку `target_line`, сохраняя пиксельную X.
pub fn move_vertical(inner: &EditorInner, target_line: usize) {
    let x = {
        let content = inner.content.borrow();
        let shaped = inner.shaped_doc.borrow();
        let cursor = inner.cursor.borrow();
        let cl = cursor.line();
        let (ls, _) = cursor_line_bounds(&content, cl);
        let byte_in_line = cursor.raw().saturating_sub(ls);
        cursor_x_on_line(&shaped, cl, byte_in_line)
    };

    let new_raw = {
        let content = inner.content.borrow();
        let shaped = inner.shaped_doc.borrow();
        let (t_start, t_end) = cursor_line_bounds(&content, target_line);
        raw_at_x_on_line(&shaped, target_line, x, t_start, t_end)
    };

    let c = inner.content.borrow();
    let mut cursor = inner.cursor.borrow_mut();
    cursor.set_raw(&c, new_raw);
    cursor.set_col_visual(x);
}
