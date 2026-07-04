#[derive(Clone, Debug)]
pub enum SegmentStyle {
    Plain,
    Bold,
    Italic,
    Strikethrough,
    Superscript,
    Subscript,
    Code,
}

pub struct Segment {
    pub text: String,
    pub style: SegmentStyle,
    pub left_marker_len: usize,
    pub right_marker_len: usize,
}

/// Парсит строку и возвращает все сегменты с разметкой
pub fn parse_line(line: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut remaining = line;
    let markers = ["**", "~~", "^", "~", "`", "*", "_"];

    while !remaining.is_empty() {
        let mut found: Option<(usize, &str, usize)> = None;

        // Ищем самый левый маркер
        for &m in &markers {
            if let Some(pos) = remaining.find(m) {
                // Проверяем, что это не часть более длинного маркера
                if m == "*" && remaining[pos..].starts_with("**") {
                    continue;
                }

                let rest = &remaining[pos + m.len()..];

                if let Some(end) = rest.find(m) {
                    let end_pos = pos + m.len() + end;

                    if found.map_or(true, |(p, _, _)| pos < p) {
                        found = Some((pos, m, end_pos));
                    }
                }
            }
        }

        if let Some((start, marker, end)) = found {
            if start > 0 {
                segments.push(Segment {
                    text: remaining[..start].to_string(),
                    style: SegmentStyle::Plain,
                    left_marker_len: 0,
                    right_marker_len: 0,
                });
            }

            let content = &remaining[start + marker.len()..end];
            let style = match marker {
                "**" => SegmentStyle::Bold,
                "*" | "_" => SegmentStyle::Italic,
                "~~" => SegmentStyle::Strikethrough,
                "^" => SegmentStyle::Superscript,
                "~" => SegmentStyle::Subscript,
                "`" => SegmentStyle::Code,
                _ => unreachable!(),
            };

            segments.push(Segment {
                text: content.to_string(),
                style,
                left_marker_len: marker.len(),
                right_marker_len: marker.len(),
            });

            remaining = &remaining[end + marker.len()..];
        } else {
            segments.push(Segment {
                text: remaining.to_string(),
                style: SegmentStyle::Plain,
                left_marker_len: 0,
                right_marker_len: 0,
            });
            break;
        }
    }

    segments
}
