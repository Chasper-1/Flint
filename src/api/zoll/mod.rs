//! Работа с .zoll-разметкой: токенизация, парсинг, AST, стили.
//!
//! # Ручки
//!
//! | Функция            | Описание                              |
//! |--------------------|---------------------------------------|
//! | `zoll_tokenize`    | Разбить текст на токены               |
//! | `zoll_parse`       | Получить AST документа                |
//! | `zoll_parse_cache` | Получить DocumentCache для редактора  |

use crate::zoll as zoll_core;

/// Разбить текст на токены .zoll-разметки.
pub fn zoll_tokenize(text: &str) -> Vec<zoll_core::token::Token> {
    zoll_core::token::tokenize(text)
}

/// Распарсить текст в AST.
pub fn zoll_parse(text: &str) -> zoll_core::ast::MarkupDoc {
    let tokens = zoll_core::token::tokenize(text);
    zoll_core::parser::parse(&tokens)
}

/// Получить DocumentCache (для интеграции с редактором).
pub fn zoll_parse_cache(text: &str) -> crate::editor::cache::DocumentCache {
    zoll_core::parse_document(text)
}

#[cfg(test)]
mod tests;
