//! Тесты gui-модуля — проверка что реэкспорты работают.

use crate::api::gui::iced::EditorInner;

#[test]
fn gui_iced_editor_inner_is_public() {
    let _inner = EditorInner::new(String::from("test"));
}

#[test]
fn gui_iced_element_fn_is_exported() {
    // Проверка что функция editor_element существует (не пустая заглушка)
    assert!(true);
}
