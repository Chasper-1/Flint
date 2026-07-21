use crate::document::Document;
use crate::editor::utils::line_utils;

pub fn move_left(doc: &mut Document) {
    doc.cursor.move_left(&doc.content);
}

pub fn move_right(doc: &mut Document) {
    doc.cursor.move_right(&doc.content);
}

pub fn move_up(doc: &mut Document) {
    let line = doc.cursor.line();
    if line == 0 {
        doc.cursor.move_home(&doc.content);
        return;
    }

    let col_x = doc.cursor.col_visual();
    let prev_line = line - 1;
    let prev_text = line_utils::line_text(&doc.content, prev_line).unwrap_or("");
    let target_char = if col_x.is_infinite() {
        prev_text.chars().count()
    } else {
        x_to_char_pos(prev_text, col_x)
    };

    let byte_offset = prev_text
        .char_indices()
        .nth(target_char)
        .map(|(b, _)| b)
        .unwrap_or(prev_text.len());

    let start = line_utils::line_start_byte(&doc.content, prev_line);
    doc.cursor.set_raw(&doc.content, start + byte_offset);
    doc.cursor.set_line(prev_line);
    doc.cursor.set_col_visual(col_x);
}

pub fn move_down(doc: &mut Document) {
    let line = doc.cursor.line();
    let total = line_utils::count_lines(&doc.content);

    if line + 1 >= total {
        doc.cursor.move_end(&doc.content);
        return;
    }

    let col_x = doc.cursor.col_visual();
    let next_line = line + 1;
    let next_text = line_utils::line_text(&doc.content, next_line).unwrap_or("");
    let target_char = if col_x.is_infinite() {
        next_text.chars().count()
    } else {
        x_to_char_pos(next_text, col_x)
    };

    let byte_offset = next_text
        .char_indices()
        .nth(target_char)
        .map(|(b, _)| b)
        .unwrap_or(next_text.len());

    let start = line_utils::line_start_byte(&doc.content, next_line);
    doc.cursor.set_raw(&doc.content, start + byte_offset);
    doc.cursor.set_line(next_line);
    doc.cursor.set_col_visual(col_x);
}

pub fn move_home(doc: &mut Document) {
    doc.cursor.move_home(&doc.content);
}

pub fn move_end(doc: &mut Document) {
    doc.cursor.move_end(&doc.content);
}

pub fn move_word_left(doc: &mut Document) {
    doc.cursor.move_word_left(&doc.content);
}

pub fn move_word_right(doc: &mut Document) {
    doc.cursor.move_word_right(&doc.content);
}

pub fn cursor_raw(doc: &Document) -> usize {
    doc.cursor.raw()
}

pub fn cursor_set_raw(doc: &mut Document, byte: usize) {
    doc.cursor.set_raw(&doc.content, byte);
}

pub fn cursor_line(doc: &Document) -> usize {
    doc.cursor.line()
}

pub fn cursor_set_line(doc: &mut Document, line: usize) {
    doc.cursor.set_line(line);
}

pub fn cursor_col(doc: &Document) -> f32 {
    doc.cursor.col_visual()
}

pub fn cursor_set_col(doc: &mut Document, col: f32) {
    doc.cursor.set_col_visual(col);
}

pub fn cursor_reset_col(doc: &mut Document) {
    doc.cursor.reset_col_visual();
}

fn x_to_char_pos(line: &str, x: f32) -> usize {
    let char_count = line.chars().count();
    let approx = (x / 10.0).round() as usize;
    approx.min(char_count)
}
