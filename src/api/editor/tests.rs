use crate::editor::state::{EditMode, EditorState};
use crate::editor::theme::EditorTheme;
use super::*;

fn make_state() -> EditorState {
    EditorState::new(EditorTheme::default(), String::new())
}

#[test]
fn set_mode_changes_state() {
    let mut s = make_state();
    assert_eq!(s.mode, EditMode::LivePreview);
    editor_set_mode(&mut s, EditMode::Preview);
    assert_eq!(s.mode, EditMode::Preview);
}

#[test]
fn get_mode_returns_current() {
    let mut s = make_state();
    editor_set_mode(&mut s, EditMode::Source);
    assert_eq!(editor_get_mode(&s), EditMode::Source);
}

#[test]
fn mode_name_preview() {
    assert_eq!(editor_mode_name(EditMode::Preview), "preview");
}

#[test]
fn mode_name_live_preview() {
    assert_eq!(editor_mode_name(EditMode::LivePreview), "live_preview");
}

#[test]
fn mode_name_source() {
    assert_eq!(editor_mode_name(EditMode::Source), "source");
}

#[test]
fn editor_state_create_default_mode() {
    let s = editor_state_create("hello");
    assert_eq!(s.mode, EditMode::LivePreview);
    assert_eq!(s.content, "hello");
}

#[test]
fn editor_state_create_empty() {
    let s = editor_state_create("");
    assert_eq!(s.content, "");
}

#[test]
fn editor_state_create_unicode() {
    let s = editor_state_create("Привет");
    assert_eq!(s.content, "Привет");
}
