use crate::document::Document;
use crate::editor::utils::line_utils;

pub fn doc_create(text: &str) -> Document {
    Document::new(text)
}

pub fn doc_text(doc: &Document) -> &str {
    &doc.content
}

pub fn doc_line(doc: &Document, idx: usize) -> Option<&str> {
    line_utils::line_text(&doc.content, idx)
}

pub fn doc_line_count(doc: &Document) -> usize {
    line_utils::count_lines(&doc.content)
}

pub fn doc_len(doc: &Document) -> usize {
    doc.content.len()
}

pub fn doc_is_empty(doc: &Document) -> bool {
    doc.content.is_empty()
}
