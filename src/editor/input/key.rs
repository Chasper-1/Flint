/// Независимое от GUI представление клавиши.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    // ---- Символьные клавиши ----
    Char(char),
    /// Специальные символы (таб, enter, escape и т.д.)
    Tab,
    Enter,
    Escape,
    Space,

    // ---- Модификаторы (как отдельные клавиши) ----
    Shift,
    Control,
    Alt,
    Super,

    // ---- Навигация ----
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
    Home,
    End,
    PageUp,
    PageDown,

    // ---- Редактирование ----
    Backspace,
    Delete,
    Insert,

    // ---- Функциональные ----
    F(u8),
}

/// Состояние модификаторов.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_: bool,
}

/// Событие клавиатуры.
#[derive(Debug, Clone, PartialEq)]
pub enum KeyEvent {
    /// Нажатие клавиши (key down).
    Pressed { key: Key, modifiers: Modifiers },
    /// Отпускание клавиши.
    Released { key: Key, modifiers: Modifiers },
    /// Ввод текста (посимвольный, после IM-обработки).
    Text(String),
    /// Вставка из буфера обмена.
    Paste(String),
    /// Копирование / вырезание (инициировано системой).
    Copy,
    Cut,
}

impl Key {
    /// Создать Key из символа, нормализуя Enter/Tab/Space.
    pub fn from_char(c: char) -> Self {
        match c {
            '\r' | '\n' => Key::Enter,
            '\t' => Key::Tab,
            ' ' => Key::Space,
            other => Key::Char(other),
        }
    }
}

impl Modifiers {
    pub const fn none() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            super_: false,
        }
    }

    pub fn is_empty(self) -> bool {
        !self.shift && !self.ctrl && !self.alt && !self.super_
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_char_newline() {
        assert_eq!(Key::from_char('\n'), Key::Enter);
    }

    #[test]
    fn from_char_tab() {
        assert_eq!(Key::from_char('\t'), Key::Tab);
    }

    #[test]
    fn from_char_space() {
        assert_eq!(Key::from_char(' '), Key::Space);
    }

    #[test]
    fn from_char_letter() {
        assert_eq!(Key::from_char('a'), Key::Char('a'));
    }

    #[test]
    fn modifiers_none_is_empty() {
        assert!(Modifiers::none().is_empty());
    }

    #[test]
    fn modifiers_not_empty() {
        let m = Modifiers {
            ctrl: true,
            ..Modifiers::none()
        };
        assert!(!m.is_empty());
    }
}
