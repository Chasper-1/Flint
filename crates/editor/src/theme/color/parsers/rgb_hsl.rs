use crate::theme::color::rgba::Rgba;
use crate::theme::color::parsers::helpers::{parse_0_100, split_func_args, parse_alpha, parse_0_255};

pub fn parse_rgb(s: &str) -> Result<Rgba, String> {
    let parts = split_func_args(s, "rgb/rgba")?;
    let r = parse_0_255(parts[0])?;
    let g = parse_0_255(parts[1])?;
    let b = parse_0_255(parts[2])?;
    let a = parse_alpha(&parts, "rgb/rgba")?;
    Ok(Rgba::from_rgb8_a(r, g, b, a))
}

pub fn parse_hsl(s: &str) -> Result<Rgba, String> {
    let parts = split_func_args(s, "hsl/hsla")?;
    let h = parts[0]
        .parse::<f32>()
        .map_err(|e| format!("hsl: оттенок не число: {}", e))?;
    let s = parse_0_100(parts[1])?;
    let l = parse_0_100(parts[2])?;
    let a = parse_alpha(&parts, "hsl/hsla")?;
    Ok(hsl_to_rgb(h, s, l).with_alpha(a))
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Rgba {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h as i32).rem_euclid(360) {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Rgba::new(r + m, g + m, b + m)
}
