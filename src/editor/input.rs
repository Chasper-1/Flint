use gtk4::gdk;

use crate::editor::state::EditorState;

pub struct Input;

impl Input {
    pub fn handle_key(
        state: &mut EditorState,
        key: gdk::Key,
        modifiers: gdk::ModifierType,
    ) -> bool {
        // CTRL+S
        if key == gdk::Key::s && modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
            let content = state.lines.join("\n");
            let _ = std::fs::write("notes.md", content);
            return true;
        }

        // CTRL+Q
        if key == gdk::Key::q && modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
            std::process::exit(0);
        }

        match key {
            gdk::Key::Left => state.move_left(),
            gdk::Key::Right => state.move_right(),
            gdk::Key::Up => state.move_up(),
            gdk::Key::Down => state.move_down(),

            gdk::Key::BackSpace => state.backspace(),
            gdk::Key::Return => state.newline(),

            _ => {
                // 💥 ВАЖНО: GTK Unicode путь
                if let Some(ch) = key.to_unicode() {
                    state.insert_char(ch);
                }
            }
        }

        true
    }
}
