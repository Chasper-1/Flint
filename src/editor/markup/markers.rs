use super::segment::{StyleFlags, STYLE_BOLD, STYLE_CODE, STYLE_ITALIC, STYLE_STRIKETHROUGH, STYLE_SUBSCRIPT, STYLE_SUPERSCRIPT};

pub const MARKERS: [&str; 7] = [
    "**",
    "~~",
    "^",
    "~",
    "`",
    "*",
    "_",
];

pub fn marker_style(marker: &str) -> StyleFlags {
    match marker {
        "**" => STYLE_BOLD,
        "*" | "_" => STYLE_ITALIC,
        "~~" => STYLE_STRIKETHROUGH,
        "^" => STYLE_SUPERSCRIPT,
        "~" => STYLE_SUBSCRIPT,
        "`" => STYLE_CODE,
        _ => unreachable!(),
    }
}