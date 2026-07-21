use crate::editor::cache::DocumentCache;
use crate::editor::cursor::Cursor;
use crate::editor::render::ShapedDocument;

/// Состояние редактора: контент, курсор, кеш разметки, shaped-документ.
///
/// GUI-независимая структура. Используется API-слоем (`api::cursor`, `api::text`)
/// и оборачивается в [`crate::gui::iced_editor::EditorInner`] для Iced.
pub struct EditorWidget {
    pub(crate) content: String,
    pub(crate) cursor: Cursor,
    pub(crate) document_cache: DocumentCache,
    pub(crate) shaped_doc: ShapedDocument,
    pub(crate) dirty: bool,
}

impl EditorWidget {
    pub fn new(text: &str) -> Self {
        let content = text.to_string();
        let cursor = Cursor::new();
        let document_cache = crate::editor::markup::parse_document(&content);
        let metrics = cosmic_text::Metrics::new(14.0, 19.6);
        let empty_buffer = cosmic_text::Buffer::new_empty(metrics);
        let shaped_doc = ShapedDocument::new(empty_buffer);

        Self {
            content,
            cursor,
            document_cache,
            shaped_doc,
            dirty: true,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    #[allow(dead_code)]
    pub fn set_content(&mut self, text: &str) {
        self.content = text.to_string();
        self.cursor = Cursor::new();
        self.document_cache = crate::editor::markup::parse_document(&self.content);
        self.dirty = true;
    }
}
