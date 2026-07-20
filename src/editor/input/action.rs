/// Команда редактора, в которую преобразуется нажатие клавиш.
#[derive(Debug, Clone, PartialEq)]
pub enum EditorAction {
    // ---- Движение курсора ----
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveWordLeft,
    MoveWordRight,
    MoveHome,
    MoveEnd,
    MovePageUp,
    MovePageDown,

    // ---- Редактирование текста ----
    /// Вставить символ/строку (из текстового события).
    InsertText(String),
    /// Вставить перевод строки.
    InsertNewline,
    DeleteBeforeCursor,
    DeleteAfterCursor,

    // ---- Буфер обмена ----
    Paste(String),
    CopySelection,
    CutSelection,

    // ---- Файл ----
    Save,

    // ---- Режимы ----
    /// Переключиться в указанный режим.
    SwitchToNormal,
    SwitchToInsert,
    SwitchToVisual,
    /// Циклическое переключение Preview / LivePreview / Source.
    CycleMode,

    // ---- Редактор ----
    Undo,
    Redo,
    /// Начать поиск.
    Search,
    /// Отменить текущее действие / закрыть поиск.
    Cancel,
    /// Ничего не делать (несмапленная клавиша).
    None,
}

impl EditorAction {
    /// Является ли действие текстовым вводом (требует Insert-режима).
    pub fn is_text_input(&self) -> bool {
        matches!(self, EditorAction::InsertText(_) | EditorAction::InsertNewline | EditorAction::Paste(_))
    }

    /// Является ли действие навигацией (доступно во всех режимах).
    pub fn is_navigation(&self) -> bool {
        matches!(
            self,
            EditorAction::MoveLeft
                | EditorAction::MoveRight
                | EditorAction::MoveUp
                | EditorAction::MoveDown
                | EditorAction::MoveWordLeft
                | EditorAction::MoveWordRight
                | EditorAction::MoveHome
                | EditorAction::MoveEnd
                | EditorAction::MovePageUp
                | EditorAction::MovePageDown
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_input_identified() {
        assert!(EditorAction::InsertText("hello".into()).is_text_input());
        assert!(EditorAction::InsertNewline.is_text_input());
        assert!(EditorAction::Paste("x".into()).is_text_input());
        assert!(!EditorAction::MoveLeft.is_text_input());
    }

    #[test]
    fn navigation_identified() {
        assert!(EditorAction::MoveLeft.is_navigation());
        assert!(EditorAction::MoveWordRight.is_navigation());
        assert!(!EditorAction::DeleteBeforeCursor.is_navigation());
    }
}
