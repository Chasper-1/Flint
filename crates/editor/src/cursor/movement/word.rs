use crate::cursor::types::Cursor;
use crate::cursor::word::{prev_word_start, next_word_start};
use crate::utils;

impl Cursor {
    /// На слово влево.
    pub fn move_word_left(&mut self, content: &str) {
        self.raw = prev_word_start(content, self.raw);
        self.line = utils::line_of_byte(content, self.raw);
        self.reset_col_visual();
        self.force_blink();
    }

    /// На слово вправо.
    pub fn move_word_right(&mut self, content: &str) {
        self.raw = next_word_start(content, self.raw);
        self.line = utils::line_of_byte(content, self.raw);
        self.reset_col_visual();
        self.force_blink();
    }
}
