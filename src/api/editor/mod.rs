//! Режимы редактора (Preview / LivePreview / Source).
//!
//! Режим определяет, как отображается документ:
//! - `Preview` — отрендеренная разметка (только чтение)
//! - `LivePreview` — рендер + редактирование (по умолчанию)
//! - `Source` — сырой текст разметки
//!
//! # Ручки
//!
//! | Функция          | Описание                  |
//! |------------------|---------------------------|
//! | `editor_set_mode`| Установить режим          |
//! | `editor_get_mode`| Получить текущий режим    |
//! | `editor_mode_name`| Имя режима как строка    |

use crate::editor::state::{EditMode, EditorState};

/// Установить режим редактора.
pub fn editor_set_mode(state: &mut EditorState, mode: EditMode) {
    state.mode = mode;
}

/// Получить текущий режим редактора.
pub fn editor_get_mode(state: &EditorState) -> EditMode {
    state.mode
}

/// Имя режима как строка.
pub fn editor_mode_name(mode: EditMode) -> &'static str {
    match mode {
        EditMode::Preview => "preview",
        EditMode::LivePreview => "live_preview",
        EditMode::Source => "source",
    }
}

/// Создать состояние редактора с режимом по умолчанию.
pub fn editor_state_create(text: &str) -> EditorState {
    EditorState::new(crate::editor::theme::EditorTheme::default(), text.to_string())
}

#[cfg(test)]
mod tests;
