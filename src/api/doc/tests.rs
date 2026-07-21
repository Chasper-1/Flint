use super::*;

#[test]
fn doc_create_with_text() {
    let d = doc_create("hello world");
    assert_eq!(d.content, "hello world");
}

#[test]
fn doc_create_empty() {
    let d = doc_create("");
    assert!(doc_is_empty(&d));
}

#[test]
fn doc_text_returns_content() {
    let d = doc_create("abc");
    assert_eq!(doc_text(&d), "abc");
}

#[test]
fn doc_line_basic() {
    let d = doc_create("first\nsecond\nthird");
    assert_eq!(doc_line(&d, 0), Some("first"));
    assert_eq!(doc_line(&d, 1), Some("second"));
    assert_eq!(doc_line(&d, 2), Some("third"));
    assert_eq!(doc_line(&d, 3), None);
}

#[test]
fn doc_line_single_line() {
    let d = doc_create("only one");
    assert_eq!(doc_line(&d, 0), Some("only one"));
    assert_eq!(doc_line(&d, 1), None);
}

#[test]
fn doc_line_count_basic() {
    let d = doc_create("a\nb\nc");
    assert_eq!(doc_line_count(&d), 3);
}

#[test]
fn doc_line_count_empty() {
    let d = doc_create("");
    // Пустая строка — 1 строка (пустая)
    assert_eq!(doc_line_count(&d), 1);
}

#[test]
fn doc_line_count_single() {
    let d = doc_create("hello");
    assert_eq!(doc_line_count(&d), 1);
}

#[test]
fn doc_len_basic() {
    let d = doc_create("hello");
    assert_eq!(doc_len(&d), 5);
}

#[test]
fn doc_len_empty() {
    let d = doc_create("");
    assert_eq!(doc_len(&d), 0);
}

#[test]
fn doc_is_empty_true() {
    let d = doc_create("");
    assert!(doc_is_empty(&d));
}

#[test]
fn doc_is_empty_false() {
    let d = doc_create("x");
    assert!(!doc_is_empty(&d));
}

#[test]
fn doc_new_is_dirty() {
    let d = doc_create("abc");
    assert!(d.dirty);
}

#[test]
fn doc_with_unicode() {
    let text = "Привет мир 👋";
    let d = doc_create(text);
    assert_eq!(doc_text(&d), text);
    assert_eq!(doc_len(&d), text.len());
}

#[test]
fn doc_line_with_unicode() {
    let d = doc_create("строка1\nстрока2");
    assert_eq!(doc_line(&d, 0), Some("строка1"));
    assert_eq!(doc_line(&d, 1), Some("строка2"));
}
