use crate::cursor::types::Cursor;
use crate::utils;

impl Cursor {
    /// На строку вверх, сохраняя пиксельную X-позицию.
    pub fn move_up(&mut self, content: &str) {
        if self.line == 0 {
            self.move_home(content);
            return;
        }
        let col_x = self.col_visual;
        let prev_line = self.line - 1;
        let prev_text = utils::line_text(content, prev_line).unwrap_or("");
        let target_char = if col_x.is_infinite() {
            prev_text.chars().count()
        } else {
            let char_count = prev_text.chars().count();
            let approx = (col_x / 10.0).round() as usize;
            approx.min(char_count)
        };

        let byte_offset = prev_text
            .char_indices()
            .nth(target_char)
            .map(|(b, _)| b)
            .unwrap_or(prev_text.len());

        let start = utils::line_start_byte(content, prev_line);
        self.raw = (start + byte_offset).min(content.len());
        self.line = prev_line;
        self.col_visual = col_x;
        self.force_blink();
    }

    /// На строку вниз, сохраняя пиксельную X-позицию.
    pub fn move_down(&mut self, content: &str) {
        let total = utils::count_lines(content);
        let next_line = self.line + 1;
        if next_line >= total {
            self.move_end(content);
            return;
        }

        let col_x = self.col_visual;
        let next_text = utils::line_text(content, next_line).unwrap_or("");
        let target_char = if col_x.is_infinite() {
            next_text.chars().count()
        } else {
            let char_count = next_text.chars().count();
            let approx = (col_x / 10.0).round() as usize;
            approx.min(char_count)
        };

        let byte_offset = next_text
            .char_indices()
            .nth(target_char)
            .map(|(b, _)| b)
            .unwrap_or(next_text.len());

        let start = utils::line_start_byte(content, next_line);
        self.raw = (start + byte_offset).min(content.len());
        self.line = next_line;
        self.col_visual = col_x;
        self.force_blink();
    }
}
