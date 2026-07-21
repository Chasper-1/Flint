//! Файловые операции: загрузка и сохранение документов.
//!
//! # Ручки
//!
//! | Функция        | Описание                        |
//! |----------------|---------------------------------|
//! | `file_save`    | Сохранить документ в файл       |
//! | `file_load`    | Загрузить документ из файла     |
//! | `file_save_str`| Сохранить строку в файл         |
//! | `file_load_str`| Загрузить строку из файла       |

use crate::document::Document;
use std::path::Path;

/// Сохранить документ в файл.
///
/// Возвращает `Ok(())` или `Err` с описанием ошибки.
pub fn file_save(doc: &Document, path: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::write(path, doc.content.as_bytes())
}

/// Загрузить документ из файла.
///
/// Если файла нет — возвращает пустой документ (не ошибка).
pub fn file_load(path: impl AsRef<Path>) -> std::io::Result<Document> {
    let content = std::fs::read_to_string(path)?;
    Ok(Document::new(&content))
}

/// Сохранить строку в файл.
pub fn file_save_str(text: &str, path: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::write(path, text.as_bytes())
}

/// Загрузить строку из файла.
pub fn file_load_str(path: impl AsRef<Path>) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

#[cfg(test)]
mod tests;
