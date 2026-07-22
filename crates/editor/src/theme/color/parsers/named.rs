use crate::theme::color::named::NAMED_COLORS;
use crate::theme::color::rgba::Rgba;

pub fn parse_named(s: &str) -> Result<Rgba, String> {
    let s = s.trim().to_lowercase();

    if let Ok(idx) = NAMED_COLORS.binary_search_by_key(&s.as_str(), |(name, _)| name) {
        Ok(NAMED_COLORS[idx].1)
    } else {
        Err(format!("неизвестный цвет: «{}»", s))
    }
}
