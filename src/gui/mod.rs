// src/gui/mod.rs
use gtk4::Application;
use gtk4::prelude::*;
use crate::editor::EditorWidget;

pub fn build_ui(app: &Application) {
    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title("Flint Notes")
        .default_width(1024)
        .default_height(768)
        .build();

    let main_paned = gtk4::Paned::new(gtk4::Orientation::Horizontal);
    main_paned.set_position(250);

    let sidebar_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    sidebar_container.set_width_request(150);

    let preview_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    preview_container.set_width_request(400);
    preview_container.set_vexpand(true);
    preview_container.set_hexpand(true);

    let text_view = EditorWidget::new();

    let scrolled_window = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .child(&text_view)
        .build();

    text_view.set_css_classes(&["editor"]);
    
    preview_container.append(&scrolled_window);

    main_paned.set_start_child(Some(&sidebar_container));
    main_paned.set_end_child(Some(&preview_container));

    window.set_child(Some(&main_paned));
    window.present();
}
