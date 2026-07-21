//! Создание документа и чтение содержимого.
//!
//! # Ручки
//!
//! | Функция              | Описание                        |
//! |----------------------|---------------------------------|
//! | `doc_create`         | Создать документ из текста      |
//! | `doc_text`           | Получить весь текст             |
//! | `doc_line`           | Получить строку по индексу      |
//! | `doc_line_count`     | Число строк                     |
//! | `doc_len`            | Байтовая длина                  |
//! | `doc_is_empty`       | Проверить пуст ли документ      |

use crate::document::Document;
use crate::editor::utils::line_utils;

/// Создать документ с текстом.
pub fn doc_create(text: &str) -> Document {
    Document::new(text)
}

/// Получить весь текст документа.
pub fn doc_text(doc: &Document) -> &str {
    &doc.content
}

/// Получить строку по индексу (0-based).
pub fn doc_line(doc: &Document, idx: usize) -> Option<&str> {
    line_utils::line_text(&doc.content, idx)
}

/// Число строк в документе.
pub fn doc_line_count(doc: &Document) -> usize {
    line_utils::count_lines(&doc.content)
}

/// Байтовая длина содержимого.
pub fn doc_len(doc: &Document) -> usize {
    doc.content.len()
}

/// Пуст ли документ (нет содержимого).
pub fn doc_is_empty(doc: &Document) -> bool {
    doc.content.is_empty()
}

#[cfg(test)]
mod tests;
