use super::segment::SegmentStyle;

// TODO(perf):
// Сейчас используются несколько `find()` для поиска ближайшего маркера.
// Для текущих объёмов текста этого достаточно, но в будущем стоит заменить
// на один линейный проход по байтам/символам без повторных поисков.
// Это уберёт лишние проходы по строке и уменьшит количество сравнений.
pub const MARKERS: [&str; 7] = [
    "**",
    "~~",
    "^",
    "~",
    "`",
    "*",
    "_",
];

pub fn marker_style(marker: &str) -> SegmentStyle {
    match marker {
        "**" => SegmentStyle::Bold,
        "*" | "_" => SegmentStyle::Italic,
        "~~" => SegmentStyle::Strikethrough,
        "^" => SegmentStyle::Superscript,
        "~" => SegmentStyle::Subscript,
        "`" => SegmentStyle::Code,
        _ => unreachable!(),
    }
}