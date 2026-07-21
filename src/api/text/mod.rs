//! Редактирование текста документа.
//!
//! # Ручки
//!
//! | Функция              | Описание                               |
//! |----------------------|----------------------------------------|
//! | `insert_at_cursor`   | Вставить текст в позиции курсора       |
//! | `delete_before`      | Удалить графему перед курсором         |
//! | `delete_after`       | Удалить графему после курсора          |
//! | `newline`            | Вставить перевод строки                |
//! | `insert_at`          | Вставить текст в произвольную позицию  |
//! | `delete_range`       | Удалить диапазон байт                  |

use crate::document::Document;
use crate::editor::cursor;

/// Вставить текст в позиции курсора.
pub fn insert_at_cursor(doc: &mut Document, text: &str) {
    let raw = doc.cursor.raw();
    doc.content.insert_str(raw, text);
    doc.cursor.set_raw(&doc.content, raw + text.len());
    doc.dirty = true;
}

/// Удалить графемный кластер перед курсором.
pub fn delete_before(doc: &mut Document) {
    let raw = doc.cursor.raw();
    if raw == 0 || doc.content.is_empty() {
        return;
    }
    let prev = cursor::prev_grapheme_boundary(&doc.content, raw).unwrap_or(0);
    doc.content.drain(prev..raw);
    doc.cursor.set_raw(&doc.content, prev);
    doc.dirty = true;
}

/// Удалить графемный кластер после курсора.
pub fn delete_after(doc: &mut Document) {
    let raw = doc.cursor.raw();
    if raw >= doc.content.len() || doc.content.is_empty() {
        return;
    }
    let next = cursor::next_grapheme_boundary(&doc.content, raw).unwrap_or(doc.content.len());
    doc.content.drain(raw..next);
    doc.cursor.set_raw(&doc.content, raw);
    doc.dirty = true;
}

/// Вставить перевод строки.
pub fn newline(doc: &mut Document) {
    let raw = doc.cursor.raw();
    doc.content.insert(raw, '\n');
    doc.cursor.set_raw(&doc.content, raw + 1);
    doc.cursor.reset_col_visual();
    doc.dirty = true;
}

/// Вставить текст в произвольную байтовую позицию.
///
/// Курсор не перемещается (в отличие от `insert_at_cursor`).
pub fn insert_at(doc: &mut Document, byte: usize, text: &str) {
    doc.content.insert_str(byte, text);
    doc.dirty = true;
}

/// Удалить диапазон байт `[start, end)`.
///
/// Если `start == end` — ничего не делает.
/// Курсор не перемещается.
pub fn delete_range(doc: &mut Document, start: usize, end: usize) {
    if start >= end || start >= doc.content.len() {
        return;
    }
    let end = end.min(doc.content.len());
    doc.content.drain(start..end);
    doc.dirty = true;
}

#[cfg(test)]
mod tests;
