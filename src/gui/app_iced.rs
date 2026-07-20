//! Iced-приложение Flint Notes (v0.14).
//!
//! Постепенно заменяет egui-версию в `app.rs` / `run.rs`.

use iced::widget::{container, scrollable, text};
use iced::{Element, Task, Theme};

use crate::editor::state::EditorState;

/// Сообщения приложения.
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

/// Состояние приложения.
struct AppState {
    state: EditorState,
}

fn boot() -> (AppState, Task<Message>) {
    let app = AppState {
        state: EditorState::new(
            crate::editor::theme::EditorTheme::default(),
            String::new(),
        ),
    };
    (app, Task::none())
}

fn update(app_state: &mut AppState, _message: Message) {
    // TODO: обработка команд редактора
}

fn view(app_state: &AppState) -> Element<'_, Message, Theme, iced::Renderer> {
    let content = if app_state.state.content.is_empty() {
        text("Flint Notes — откройте или создайте файл")
    } else {
        text(&app_state.state.content)
    };

    let editor = container(content)
        .padding(10)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill);

    container(scrollable(container(editor)))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

/// Запустить Iced-приложение.
pub fn run() -> iced::Result {
    iced::application(boot, update, view)
        .window_size(iced::Size::new(1200.0, 800.0))
        .run()
}
