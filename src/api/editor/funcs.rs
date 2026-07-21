use crate::editor::state::{EditMode, EditorState};

pub fn editor_set_mode(state: &mut EditorState, mode: EditMode) {
    state.mode = mode;
}

pub fn editor_get_mode(state: &EditorState) -> EditMode {
    state.mode
}

pub fn editor_mode_name(mode: EditMode) -> &'static str {
    match mode {
        EditMode::Preview => "preview",
        EditMode::LivePreview => "live_preview",
        EditMode::Source => "source",
    }
}

pub fn editor_state_create(text: &str) -> EditorState {
    EditorState::new(crate::editor::theme::EditorTheme::default(), text.to_string())
}
