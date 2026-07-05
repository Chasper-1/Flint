use crate::editor::cache::MarkupCache;

/// Переводит позицию в отображаемом тексте
/// в позицию исходного Markdown.
pub fn visible_to_raw(cache: &MarkupCache, visible: usize) -> usize {
    for seg in &cache.segments {
        if visible >= seg.visible_start && visible <= seg.visible_end {
            return seg.raw_start
                + seg.left_marker_len
                + (visible - seg.visible_start);
        }
    }

    visible
}

/// Переводит позицию исходного Markdown
/// в позицию отображаемого текста.
pub fn raw_to_visible(cache: &MarkupCache, raw: usize) -> usize {
    for seg in &cache.segments {
        let content_start = seg.raw_start + seg.left_marker_len;
        let content_end = seg.raw_end.saturating_sub(seg.right_marker_len);

        if raw < content_start {
            return seg.visible_start;
        }

        if raw > content_end {
            return seg.visible_end;
        }

        if raw >= content_start && raw <= content_end {
            return seg.visible_start + (raw - content_start);
        }
    }

    raw
}

/// Находит сегмент по видимой позиции.
pub fn segment_at_visible(
    cache: &MarkupCache,
    visible: usize,
) -> Option<&crate::editor::markup::Segment> {
    cache
        .segments
        .iter()
        .find(|seg| visible >= seg.visible_start && visible <= seg.visible_end)
}

/// Находит сегмент по позиции в Markdown.
pub fn segment_at_raw(
    cache: &MarkupCache,
    raw: usize,
) -> Option<&crate::editor::markup::Segment> {
    cache
        .segments
        .iter()
        .find(|seg| raw >= seg.raw_start && raw <= seg.raw_end)
}