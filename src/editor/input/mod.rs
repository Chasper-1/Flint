//! Чистая (framework-agnostic) обработка ввода.
//!
//! Содержит:
//! - `Key` / `Modifiers` / `KeyEvent` — независимое представление клавиш
//! - `EditorAction` — команды редактора
//! - `Keymap` — маппинг клавиш → действия
//!
//! Для интеграции с GUI напишите адаптер, который конвертирует события
//! конкретного фреймворка в наши типы.

pub mod action;
pub mod key;
pub mod keymap;

pub use action::EditorAction;
pub use key::{Key, KeyEvent, Modifiers};
pub use keymap::{default_keymap, normal_keymap, Keymap};
