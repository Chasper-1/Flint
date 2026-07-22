use crate::theme::color::rgba::Rgba;

pub fn parse_hex(s: &str) -> Result<Rgba, String> {
    let hex = &s[1..]; // убираем #
    let len = hex.len();

    let expanded: String = match len {
        3 => hex.chars().flat_map(|c| [c, c]).collect(),
        4 => hex.chars().flat_map(|c| [c, c]).collect(),
        _ => hex.to_string(),
    };

    let expanded_len = expanded.len();
    if expanded_len != 6 && expanded_len != 8 {
        return Err(format!(
            "неверная длина hex-цвета: #{} ({} символов)",
            hex, len
        ));
    }

    let ch = |start: usize| -> Result<u8, String> {
        u8::from_str_radix(&expanded[start..start + 2], 16)
            .map_err(|_| format!("неверный hex: #{}", hex))
    };

    let r = ch(0)?;
    let g = ch(2)?;
    let b = ch(4)?;
    let a = if expanded_len == 8 { ch(6)? } else { 255 };

    Ok(Rgba::from_rgba8(r, g, b, a))
}
