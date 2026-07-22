use crate::cursor::grapheme::{clamp_to_char_boundary, next_grapheme_boundary, prev_grapheme_boundary};
use crate::cursor::types::Cursor;
use crate::utils;

impl Cursor {
    /// Установить `raw` с проверкой границ.
    pub fn set_raw(&mut self, content: &str, new_raw: usize) {
        self.raw = clamp_to_char_boundary(content, new_raw);
        self.line = utils::line_of_byte(content, self.raw);
        self.force_blink();
    }

    /// На один grapheme-кластер влево.
    pub fn move_left(&mut self, content: &str) {
        if self.raw == 0 { return; }
        self.raw = prev_grapheme_boundary(content, self.raw).unwrap_or(0);
        self.line = utils::line_of_byte(content, self.raw);
        self.force_blink();
    }

    /// На один grapheme-кластер вправо.
    pub fn move_right(&mut self, content: &str) {
        if self.raw >= content.len() { return; }
        self.raw = next_grapheme_boundary(content, self.raw).unwrap_or(content.len());
        self.line = utils::line_of_byte(content, self.raw);
        self.force_blink();
    }

    /// В начало текущей строки.
    pub fn move_home(&mut self, content: &str) {
        self.raw = utils::line_start_byte(content, self.line);
        self.col_visual = 0.0;
        self.force_blink();
    }

    /// В конец текущей строки.
    pub fn move_end(&mut self, content: &str) {
        self.raw = utils::line_end_byte(content, self.line);
        self.col_visual = f32::MAX;
        self.force_blink();
    }
}
