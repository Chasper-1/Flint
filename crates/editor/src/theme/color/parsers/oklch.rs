use crate::theme::color::rgba::Rgba;
use crate::theme::color::parsers::helpers::parse_0_100;

pub fn parse_oklch(s: &str) -> Result<Rgba, String> {
    let inner = s.trim_start_matches("oklch(").trim_end_matches(')').trim();

    let parts: Vec<&str> = inner
        .split(|c| c == ' ' || c == ',')
        .filter(|p| !p.is_empty())
        .collect();
    if parts.len() != 3 {
        return Err(format!(
            "oklch: нужно 3 аргумента (l c h), получено {}",
            parts.len()
        ));
    }

    let l = parse_0_100(parts[0])?;
    let c = parts[1]
        .parse::<f32>()
        .map_err(|e| format!("oklch: насыщенность не число: {}", e))?
        .max(0.0);
    let h = parts[2]
        .parse::<f32>()
        .map_err(|e| format!("oklch: оттенок не число: {}", e))?;

    Ok(oklch_to_rgb(l, c, h))
}

pub fn oklch_to_rgb(l: f32, c: f32, h: f32) -> Rgba {
    let h_rad = h.to_radians();
    let a = c * h_rad.cos();
    let b_val = c * h_rad.sin();

    let l_ = l + 0.3963377774 * a + 0.2158037573 * b_val;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b_val;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b_val;

    let l3 = l_ * l_ * l_;
    let m3 = m_ * m_ * m_;
    let s3 = s_ * s_ * s_;

    let r_lin = 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
    let g_lin = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
    let b_lin = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3;

    fn srgb_gamma(c: f32) -> f32 {
        let c = c.max(0.0).min(1.0);
        if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    }

    Rgba::new(srgb_gamma(r_lin), srgb_gamma(g_lin), srgb_gamma(b_lin))
}
