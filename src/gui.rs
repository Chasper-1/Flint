use crate::editor::renderer::Renderer;
use crate::editor::{state::EditorState, theme};

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, EventControllerKey, gdk};

use rhai::Engine;

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Flint Notes")
        .default_width(1200)
        .default_height(800)
        .build();

    // ---------- Theme ----------

    let src = fs::read_to_string("theme.rhai").expect("theme.rhai not found");

    let engine = Engine::new();

    let ast = engine.compile(&src).expect("Rhai compile error");

    let rhai_map: rhai::Map = engine.eval_ast(&ast).expect("Rhai runtime error");

    let theme = theme::parse_theme(rhai_map);

    // ---------- Document ----------

    let text = fs::read_to_string("notes.md").unwrap_or_else(|_| String::new());

    let state = Rc::new(RefCell::new(EditorState::new(theme, text)));

    // ---------- Drawing ----------

    let area = DrawingArea::new();

    area.set_focusable(true);
    area.set_hexpand(true);
    area.set_vexpand(true);

    {
        let state = state.clone();

        area.set_draw_func(move |_, cr, width, height| {
            Renderer::draw(cr, width, height, &state.borrow());
        });
    }

    // ---------- Keyboard ----------

    {
        let state = state.clone();
        let area_clone = area.clone();

        let controller = EventControllerKey::new();

        controller.connect_key_pressed(move |_, key, _, _| {
            let mut state = state.borrow_mut();

            fn char_to_byte(s: &str, column: usize) -> usize {
                s.char_indices()
                    .nth(column)
                    .map(|(i, _)| i)
                    .unwrap_or(s.len())
            }

            match key {
                gdk::Key::Left => {
                    if state.cursor.column > 0 {
                        state.cursor.column -= 1;
                    }
                }

                gdk::Key::Right => {
                    if let Some(line) = state.lines.get(state.cursor.line) {
                        let len = line.chars().count();

                        if state.cursor.column < len {
                            state.cursor.column += 1;
                        }
                    }
                }

                gdk::Key::Up => {
                    if state.cursor.line > 0 {
                        state.cursor.line -= 1;

                        if let Some(line) = state.lines.get(state.cursor.line) {
                            state.cursor.column =
                                state.cursor.column.min(line.chars().count());
                        }
                    }
                }

                gdk::Key::Down => {
                    if state.cursor.line + 1 < state.lines.len() {
                        state.cursor.line += 1;

                        if let Some(line) = state.lines.get(state.cursor.line) {
                            state.cursor.column =
                                state.cursor.column.min(line.chars().count());
                        }
                    }
                }

                gdk::Key::BackSpace => {
                    let line = state.cursor.line;
                    let col = state.cursor.column;

                    if col > 0 {
                        if let Some(text) = state.lines.get_mut(line) {
                            let byte = char_to_byte(text, col - 1);
                            text.remove(byte);
                            state.cursor.column -= 1;
                        }
                    }
                }

                gdk::Key::Return => {
                    let line = state.cursor.line;
                    let col = state.cursor.column;

                    if let Some(text) = state.lines.get_mut(line) {
                        let byte = char_to_byte(text, col);
                        let tail = text.split_off(byte);

                        state.lines.insert(line + 1, tail);

                        state.cursor.line += 1;
                        state.cursor.column = 0;
                    }
                }

                _ => {
                    if let Some(ch) = key.to_unicode() {
                        if !ch.is_control() {
                            let line = state.cursor.line;
                            let col = state.cursor.column;

                            if let Some(text) = state.lines.get_mut(line) {
                                let byte = char_to_byte(text, col);

                                text.insert(byte, ch);

                                state.cursor.column += 1;
                            }
                        }
                    }
                }
            }

            area_clone.queue_draw();

            gtk4::glib::Propagation::Stop
        });

        area.add_controller(controller);
    }

    window.set_child(Some(&area));

    window.present();

    area.grab_focus();
}
