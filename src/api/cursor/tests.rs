use crate::document::Document;
use super::*;

fn make_doc(text: &str) -> Document {
    Document::new(text)
}

// ── move_left / move_right ───────────────────────────────────────────

#[test]
fn move_left_basic() {
    let mut d = make_doc("abc");
    d.cursor.set_raw(&d.content, 2);
    move_left(&mut d);
    assert_eq!(d.cursor.raw(), 1);
    assert_eq!(d.cursor.line(), 0);
}

#[test]
fn move_left_at_start() {
    let mut d = make_doc("abc");
    move_left(&mut d);
    assert_eq!(d.cursor.raw(), 0);
}

#[test]
fn move_right_basic() {
    let mut d = make_doc("abc");
    move_right(&mut d);
    assert_eq!(d.cursor.raw(), 1);
}

#[test]
fn move_right_at_end() {
    let mut d = make_doc("abc");
    d.cursor.set_raw(&d.content, 3);
    move_right(&mut d);
    assert_eq!(d.cursor.raw(), 3);
}

#[test]
fn move_left_grapheme() {
    let mut d = make_doc("e\u{0301}x");
    d.cursor.set_raw(&d.content, 3);
    move_left(&mut d);
    assert_eq!(d.cursor.raw(), 0);
}

#[test]
fn move_right_grapheme() {
    let mut d = make_doc("e\u{0301}x");
    move_right(&mut d);
    assert_eq!(d.cursor.raw(), 3);
}

// ── move_up / move_down ──────────────────────────────────────────────

#[test]
fn move_up_simple() {
    let mut d = make_doc("first\nsecond");
    d.cursor.set_raw(&d.content, 10);
    move_up(&mut d);
    assert_eq!(d.cursor.line(), 0);
}

#[test]
fn move_up_at_first_line_goes_home() {
    let mut d = make_doc("only one line");
    d.cursor.set_raw(&d.content, 5);
    move_up(&mut d);
    assert_eq!(d.cursor.raw(), 0);
}

#[test]
fn move_down_simple() {
    let mut d = make_doc("first\nsecond");
    move_down(&mut d);
    assert_eq!(d.cursor.line(), 1);
}

#[test]
fn move_down_at_last_line_goes_end() {
    let mut d = make_doc("first\nsecond");
    d.cursor.set_raw(&d.content, 6);
    d.cursor.set_line(1);
    let len = d.content.len();
    move_down(&mut d);
    assert_eq!(d.cursor.raw(), len);
}

// ── move_home / move_end ─────────────────────────────────────────────

#[test]
fn move_home_works() {
    let mut d = make_doc("hello world");
    d.cursor.set_raw(&d.content, 5);
    move_home(&mut d);
    assert_eq!(d.cursor.raw(), 0);
}

#[test]
fn move_end_works() {
    let mut d = make_doc("hello world");
    move_end(&mut d);
    assert_eq!(d.cursor.raw(), 11);
}

// ── move_word_left / move_word_right ─────────────────────────────────

#[test]
fn move_word_left_works() {
    let mut d = make_doc("hello world foo");
    d.cursor.set_raw(&d.content, 16);
    move_word_left(&mut d);
    assert_eq!(d.cursor.raw(), 12);
}

#[test]
fn move_word_right_works() {
    let mut d = make_doc("hello world");
    move_word_right(&mut d);
    assert_eq!(d.cursor.raw(), 6);
}

// ── Прямые доступоры ─────────────────────────────────────────────────

#[test]
fn cursor_raw_getter() {
    let d = make_doc("abc");
    assert_eq!(cursor_raw(&d), 0);
}

#[test]
fn cursor_raw_setter() {
    let mut d = make_doc("abc\ndef");
    cursor_set_raw(&mut d, 5);
    assert_eq!(cursor_raw(&d), 5);
}

#[test]
fn cursor_line_getter() {
    let mut d = make_doc("a\nb\nc");
    d.cursor.set_raw(&d.content, 3);
    assert_eq!(cursor_line(&d), 1);
}

#[test]
fn cursor_line_setter() {
    let mut d = make_doc("a\nb\nc");
    cursor_set_line(&mut d, 2);
    assert_eq!(cursor_line(&d), 2);
}

#[test]
fn cursor_col_getter() {
    let mut d = make_doc("hello");
    cursor_set_raw(&mut d, 3);
    assert_eq!(cursor_col(&d), 0.0); // col_visual сброшен set_raw
}

#[test]
fn cursor_col_setter() {
    let mut d = make_doc("hello");
    cursor_set_col(&mut d, 42.0);
    assert_eq!(cursor_col(&d), 42.0);
}

#[test]
fn cursor_reset_col_works() {
    let mut d = make_doc("hello");
    cursor_set_col(&mut d, 99.0);
    cursor_reset_col(&mut d);
    assert_eq!(cursor_col(&d), 0.0);
}

// ── unicode ──────────────────────────────────────────────────────────

#[test]
fn unicode_move_left_right() {
    let mut d = make_doc("Привет");
    d.cursor.set_raw(&d.content, 12);
    move_left(&mut d);
    assert_eq!(d.cursor.raw(), 10);
    move_right(&mut d);
    assert_eq!(d.cursor.raw(), 12);
}

#[test]
fn unicode_word_nav() {
    let mut d = make_doc("hello\u{A0}world");
    move_word_right(&mut d);
    assert_eq!(d.cursor.raw(), 7);
}

#[test]
fn word_nav_tab() {
    let mut d = make_doc("a\tb");
    move_word_right(&mut d);
    assert_eq!(d.cursor.raw(), 2);
}
