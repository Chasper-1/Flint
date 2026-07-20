use std::collections::HashMap;

use super::action::EditorAction;
use super::key::{Key, KeyEvent, Modifiers};

/// Маппинг (Key + Modifiers) → EditorAction для конкретного режима.
#[derive(Debug, Clone)]
pub struct Keymap {
    map: HashMap<(Key, Modifiers), EditorAction>,
    /// Fallback для символьных клавиш (обычно InsertText(char)).
    text_fallback: bool,
}

impl Keymap {
    pub fn new(text_fallback: bool) -> Self {
        Self {
            map: HashMap::new(),
            text_fallback,
        }
    }

    /// Добавить привязку.
    pub fn bind(&mut self, key: Key, modifiers: Modifiers, action: EditorAction) {
        self.map.insert((key, modifiers), action);
    }

    /// Удобная обёртка: клавиша без модификаторов.
    pub fn bind_simple(&mut self, key: Key, action: EditorAction) {
        self.bind(key, Modifiers::none(), action);
    }

    /// Удобная обёртка: Ctrl+Key.
    pub fn bind_ctrl(&mut self, key: Key, action: EditorAction) {
        self.bind(
            key,
            Modifiers {
                ctrl: true,
                ..Modifiers::none()
            },
            action,
        );
    }

    /// Разрешить/запретить текстовый fallback (непривязанные Char → InsertText).
    pub fn set_text_fallback(&mut self, enabled: bool) {
        self.text_fallback = enabled;
    }

    /// Получить действие по событию.
    pub fn resolve(&self, event: &KeyEvent) -> EditorAction {
        match event {
            KeyEvent::Pressed { key, modifiers } => self.resolve_pressed(key, *modifiers),
            KeyEvent::Text(t) => EditorAction::InsertText(t.clone()),
            KeyEvent::Paste(t) => EditorAction::Paste(t.clone()),
            KeyEvent::Copy => EditorAction::CopySelection,
            KeyEvent::Cut => EditorAction::CutSelection,
            KeyEvent::Released { .. } => EditorAction::None,
        }
    }

    fn resolve_pressed(&self, key: &Key, modifiers: Modifiers) -> EditorAction {
        // 1. Точное совпадение (Key + Modifiers).
        if let Some(action) = self.map.get(&(key.clone(), modifiers)) {
            return action.clone();
        }

        // 2. Совпадение только по клавише (без модификаторов).
        if !modifiers.is_empty() {
            if let Some(action) = self.map.get(&(key.clone(), Modifiers::none())) {
                return action.clone();
            }
        }

        // 3. Текстовый fallback: одиночный символ → InsertText.
        if self.text_fallback {
            if let Key::Char(c) = key {
                if !modifiers.ctrl && !modifiers.alt && !modifiers.super_ {
                    return EditorAction::InsertText(c.to_string());
                }
            }
        }

        EditorAction::None
    }
}

// ---------------------------------------------------------------------------
// Стандартная раскладка (режим вставки / обычный редактор)
// ---------------------------------------------------------------------------

/// Стандартная раскладка: поведение как в обычном текстовом редакторе.
pub fn default_keymap() -> Keymap {
    let mut km = Keymap::new(true); // text_fallback = true

    // ---- Навигация ----
    km.bind_simple(Key::ArrowLeft, EditorAction::MoveLeft);
    km.bind_simple(Key::ArrowRight, EditorAction::MoveRight);
    km.bind_simple(Key::ArrowUp, EditorAction::MoveUp);
    km.bind_simple(Key::ArrowDown, EditorAction::MoveDown);
    km.bind_simple(Key::Home, EditorAction::MoveHome);
    km.bind_simple(Key::End, EditorAction::MoveEnd);
    km.bind_simple(Key::PageUp, EditorAction::MovePageUp);
    km.bind_simple(Key::PageDown, EditorAction::MovePageDown);
    km.bind_ctrl(Key::ArrowLeft, EditorAction::MoveWordLeft);
    km.bind_ctrl(Key::ArrowRight, EditorAction::MoveWordRight);

    // ---- Редактирование ----
    km.bind_simple(Key::Backspace, EditorAction::DeleteBeforeCursor);
    km.bind_simple(Key::Delete, EditorAction::DeleteAfterCursor);
    km.bind_simple(Key::Enter, EditorAction::InsertNewline);

    // ---- Системные ----
    km.bind_ctrl(Key::Char('s'), EditorAction::Save);
    km.bind_simple(Key::Escape, EditorAction::Cancel);
    km.bind_simple(Key::Tab, EditorAction::InsertText("\t".into()));

    km
}

// ---------------------------------------------------------------------------
// Vim-подобная раскладка для Normal-режима
// ---------------------------------------------------------------------------

pub fn normal_keymap() -> Keymap {
    let mut km = Keymap::new(false); // text_fallback = false

    // Движение
    km.bind_simple(Key::Char('h'), EditorAction::MoveLeft);
    km.bind_simple(Key::Char('j'), EditorAction::MoveDown);
    km.bind_simple(Key::Char('k'), EditorAction::MoveUp);
    km.bind_simple(Key::Char('l'), EditorAction::MoveRight);
    km.bind_simple(Key::Char('w'), EditorAction::MoveWordRight);
    km.bind_simple(Key::Char('b'), EditorAction::MoveWordLeft);
    km.bind_simple(Key::Char('0'), EditorAction::MoveHome);
    km.bind_simple(Key::Char('$'), EditorAction::MoveEnd);

    // Редактирование
    km.bind_simple(Key::Char('i'), EditorAction::SwitchToInsert);
    km.bind_simple(Key::Char('x'), EditorAction::DeleteAfterCursor);
    km.bind_simple(Key::Backspace, EditorAction::DeleteBeforeCursor);

    // Системные
    km.bind_ctrl(Key::Char('s'), EditorAction::Save);
    km.bind_simple(Key::Escape, EditorAction::Cancel);

    km
}

// ---------------------------------------------------------------------------
// Тесты
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_left_arrow() {
        let km = default_keymap();
        assert_eq!(
            km.resolve(&KeyEvent::Pressed {
                key: Key::ArrowLeft,
                modifiers: Modifiers::none()
            }),
            EditorAction::MoveLeft
        );
    }

    #[test]
    fn default_ctrl_left() {
        let km = default_keymap();
        assert_eq!(
            km.resolve(&KeyEvent::Pressed {
                key: Key::ArrowLeft,
                modifiers: Modifiers {
                    ctrl: true,
                    ..Modifiers::none()
                }
            }),
            EditorAction::MoveWordLeft
        );
    }

    #[test]
    fn default_text_fallback() {
        let km = default_keymap();
        assert_eq!(
            km.resolve(&KeyEvent::Pressed {
                key: Key::Char('a'),
                modifiers: Modifiers::none()
            }),
            EditorAction::InsertText("a".into())
        );
    }

    #[test]
    fn default_ctrl_char_no_fallback() {
        let km = default_keymap();
        // Ctrl+A не привязан → None (ctrl отключает text_fallback)
        assert_eq!(
            km.resolve(&KeyEvent::Pressed {
                key: Key::Char('a'),
                modifiers: Modifiers {
                    ctrl: true,
                    ..Modifiers::none()
                }
            }),
            EditorAction::None
        );
    }

    #[test]
    fn normal_no_text_fallback() {
        let km = normal_keymap();
        // 'a' не привязан в normal → None (text_fallback выключен)
        assert_eq!(
            km.resolve(&KeyEvent::Pressed {
                key: Key::Char('a'),
                modifiers: Modifiers::none()
            }),
            EditorAction::None
        );
    }

    #[test]
    fn normal_h_moves_left() {
        let km = normal_keymap();
        assert_eq!(
            km.resolve(&KeyEvent::Pressed {
                key: Key::Char('h'),
                modifiers: Modifiers::none()
            }),
            EditorAction::MoveLeft
        );
    }

    #[test]
    fn text_event() {
        let km = default_keymap();
        assert_eq!(
            km.resolve(&KeyEvent::Text("hello".into())),
            EditorAction::InsertText("hello".into())
        );
    }

    #[test]
    fn paste_event() {
        let km = default_keymap();
        assert_eq!(
            km.resolve(&KeyEvent::Paste("content".into())),
            EditorAction::Paste("content".into())
        );
    }

    #[test]
    fn released_event_returns_none() {
        let km = default_keymap();
        assert_eq!(
            km.resolve(&KeyEvent::Released {
                key: Key::ArrowLeft,
                modifiers: Modifiers::none()
            }),
            EditorAction::None
        );
    }

    #[test]
    fn copy_event() {
        let km = default_keymap();
        assert_eq!(km.resolve(&KeyEvent::Copy), EditorAction::CopySelection);
    }
}
