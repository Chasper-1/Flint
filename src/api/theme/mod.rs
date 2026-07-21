//! Темы оформления редактора.
//!
//! # Ручки
//!
//! | Функция              | Описание                        |
//! |----------------------|---------------------------------|
//! | `theme_default`      | Тема по умолчанию               |
//! | `theme_set_name`     | Установить название темы        |
//! | `theme_get_name`     | Получить название темы          |
//! | `theme_set_bg`       | Установить цвет фона (hex RGB)  |
//! | `theme_set_text`     | Установить цвет текста (hex RGB)|

use crate::editor::theme::EditorTheme;

/// Создать тему по умолчанию.
pub fn theme_default() -> EditorTheme {
    EditorTheme::default()
}

/// Установить имя темы (для идентификации).
pub fn theme_set_name(theme: &mut EditorTheme, name: &str) {
    theme.name = name.to_string();
}

/// Получить имя темы.
pub fn theme_get_name(theme: &EditorTheme) -> &str {
    &theme.name
}

/// Установить цвет фона из hex-строки (например `"#1e1e2e"`).
pub fn theme_set_bg(theme: &mut EditorTheme, hex: &str) -> Result<(), String> {
    let rgba = crate::editor::theme::color::parse_color(hex)?;
    theme.background = rgba;
    Ok(())
}

/// Установить цвет текста из hex-строки.
pub fn theme_set_text(theme: &mut EditorTheme, hex: &str) -> Result<(), String> {
    let rgba = crate::editor::theme::color::parse_color(hex)?;
    theme.text.color = rgba;
    Ok(())
}

#[cfg(test)]
mod tests;
