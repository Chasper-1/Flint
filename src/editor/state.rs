use crate::editor::theme::EditorTheme;

pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

pub struct EditorState {
    pub theme: EditorTheme,
    pub lines: Vec<String>,
    pub cursor: Cursor,
}

impl EditorState {
    pub fn new(theme: EditorTheme, text: String) -> Self {
        Self {
            theme,
            lines: text.lines().map(|s| s.to_string()).collect(),
            cursor: Cursor { line: 0, column: 0 },
        }
    }

    pub fn get_theme(&self) -> &EditorTheme {
        &self.theme
    }
}

impl EditorState {
    // Вспомогательная функция для работы с байтовыми индексами
    fn get_byte_index(text: &str, column: usize) -> usize {
        text.char_indices()
            .nth(column)
            .map(|(i, _)| i)
            .unwrap_or(text.len())
    }

    pub fn move_left(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(line) = self.lines.get(self.cursor.line) {
            if self.cursor.column < line.chars().count() {
                self.cursor.column += 1;
            }
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            if let Some(line) = self.lines.get(self.cursor.line) {
                self.cursor.column = self.cursor.column.min(line.chars().count());
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.cursor.line + 1 < self.lines.len() {
            self.cursor.line += 1;
            if let Some(line) = self.lines.get(self.cursor.line) {
                self.cursor.column = self.cursor.column.min(line.chars().count());
            }
        }
    }

    pub fn backspace(&mut self) {
        let line = self.cursor.line;
        let col = self.cursor.column;

        if col > 0 {
            if let Some(text) = self.lines.get_mut(line) {
                let byte = Self::get_byte_index(text, col - 1);
                text.remove(byte);
                self.cursor.column -= 1;
            }
        }
    }

    pub fn newline(&mut self) {
        let line = self.cursor.line;
        let col = self.cursor.column;

        if let Some(text) = self.lines.get_mut(line) {
            let byte = Self::get_byte_index(text, col);
            let tail = text.split_off(byte);
            self.lines.insert(line + 1, tail);
            self.cursor.line += 1;
            self.cursor.column = 0;
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if !ch.is_control() {
            let line = self.cursor.line;
            let col = self.cursor.column;
            if let Some(text) = self.lines.get_mut(line) {
                let byte = Self::get_byte_index(text, col);
                text.insert(byte, ch);
                self.cursor.column += 1;
            }
        }
    }
}
