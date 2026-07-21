use crate::zoll as zoll_core;

pub fn zoll_tokenize(text: &str) -> Vec<zoll_core::token::Token> {
    zoll_core::token::tokenize(text)
}

pub fn zoll_parse(text: &str) -> zoll_core::ast::MarkupDoc {
    let tokens = zoll_core::token::tokenize(text);
    zoll_core::parser::parse(&tokens)
}

pub fn zoll_parse_cache(text: &str) -> crate::editor::cache::DocumentCache {
    zoll_core::parse_document(text)
}
