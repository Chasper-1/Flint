//! Парсер цветов для тем оформления.
//!
//! Поддерживаемые форматы:
//! - `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`
//! - `rgb(r, g, b)`, `rgba(r, g, b, a)`
//! - `hsl(h, s, l)`, `hsla(h, s, l, a)`
//! - `oklch(l, c, h)`
//! - именованные цвета: `red`, `blue`, `transparent`, …
//!
//! Все форматы возвращают единый тип [`Rgba`] с компонентами в диапазоне `0.0..=1.0`.

/// Цвет в формате RGBA (все компоненты нормированы в `0.0..=1.0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    /// Создать цвет с полностью непрозрачной альфой.
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Создать цвет с явной альфой.
    pub const fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    /// Создать цвет из целых компонентов 0–255.
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Создать цвет из целых компонентов 0–255 с float-альфой 0..1.
    pub fn from_rgb8_a(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    /// Строковое представление `rgba(r, g, b, a)` для вывода.
    pub fn to_string_rgba(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            self.a,
        )
    }
}

/// Конвертирует [`Rgba`] в кортеж из четырёх целых чисел (u8) для UI-фреймворков.
pub fn rgba_to_u8(c: &Rgba) -> (u8, u8, u8, u8) {
    (
        (c.r * 255.0) as u8,
        (c.g * 255.0) as u8,
        (c.b * 255.0) as u8,
        (c.a * 255.0) as u8,
    )
}

// —————— публичный API ——————

/// Формат записи цвета.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorFormat {
    /// `#RGB` / `#RGBA` / `#RRGGBB` / `#RRGGBBAA`
    Hex,
    /// `rgb(r,g,b)` / `rgba(r,g,b,a)`
    Rgb,
    /// `hsl(h,s,l)` / `hsla(h,s,l,a)`
    Hsl,
    /// `oklch(l,c,h)`
    Oklch,
    /// Именованный цвет (`red`, `blue`, …)
    Named,
}

/// Парсит строку в цвет.
///
/// # Errors
/// Возвращает `Err` с описанием, если строка не является корректным цветом.
pub fn parse_color(s: &str) -> Result<Rgba, String> {
    let s = s.trim();

    if s.starts_with('#') {
        parse_hex(s)
    } else if s.starts_with("rgba(") || s.starts_with("rgb(") {
        parse_rgb(s)
    } else if s.starts_with("hsla(") || s.starts_with("hsl(") {
        parse_hsl(s)
    } else if s.starts_with("oklch(") {
        parse_oklch(s)
    } else {
        parse_named(s)
    }
}

/// Определяет формат цвета по первому цвету.
pub fn detect_format(s: &str) -> Option<ColorFormat> {
    let s = s.trim();
    if s.starts_with('#') {
        Some(ColorFormat::Hex)
    } else if s.starts_with("rgba(") || s.starts_with("rgb(") {
        Some(ColorFormat::Rgb)
    } else if s.starts_with("hsla(") || s.starts_with("hsl(") {
        Some(ColorFormat::Hsl)
    } else if s.starts_with("oklch(") {
        Some(ColorFormat::Oklch)
    } else if NAMED_COLORS
        .binary_search_by_key(&s, |(name, _)| name)
        .is_ok()
    {
        Some(ColorFormat::Named)
    } else {
        None
    }
}

/// Проверяет, что все цвета в списке имеют один и тот же формат.
///
/// Возвращает формат, если всё согласовано, или ошибку с описанием.
pub fn enforce_consistency(colors: &[&str]) -> Result<ColorFormat, String> {
    let first = match colors.first() {
        Some(c) => c,
        None => return Err("список цветов пуст".into()),
    };

    let fmt =
        detect_format(first).ok_or_else(|| format!("неизвестный формат цвета: «{}»", first))?;

    for (i, c) in colors.iter().enumerate().skip(1) {
        let other = detect_format(c)
            .ok_or_else(|| format!("строка {}: неизвестный формат цвета «{}»", i + 1, c))?;
        if other != fmt {
            let fmt_name = |f: ColorFormat| -> &'static str {
                match f {
                    ColorFormat::Hex => "hex",
                    ColorFormat::Rgb => "rgb/rgba",
                    ColorFormat::Hsl => "hsl/hsla",
                    ColorFormat::Oklch => "oklch",
                    ColorFormat::Named => "именованный",
                }
            };
            return Err(format!(
                "строка {}: формат «{}» не совпадает с «{}» (строка 1)",
                i + 1,
                fmt_name(other),
                fmt_name(fmt),
            ));
        }
    }

    Ok(fmt)
}

// —————— парсеры форматов ——————

fn parse_hex(s: &str) -> Result<Rgba, String> {
    let hex = &s[1..]; // убираем #
    let len = hex.len();

    let (r, g, b, a) = match len {
        3 => {
            // #RGB → #RRGGBB
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| "неверный hex")?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| "неверный hex")?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| "неверный hex")?;
            (r, g, b, 255)
        }
        4 => {
            // #RGBA → #RRGGBBAA
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|_| "неверный hex")?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|_| "неверный hex")?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|_| "неверный hex")?;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).map_err(|_| "неверный hex")?;
            (r, g, b, a)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "неверный hex")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "неверный hex")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "неверный hex")?;
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "неверный hex")?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "неверный hex")?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "неверный hex")?;
            let a = u8::from_str_radix(&hex[6..8], 16).map_err(|_| "неверный hex")?;
            (r, g, b, a)
        }
        _ => {
            return Err(format!(
                "неверная длина hex-цвета: #{} ({} символов)",
                hex, len
            ));
        }
    };

    Ok(Rgba::from_rgba8(r, g, b, a))
}

fn parse_rgb(s: &str) -> Result<Rgba, String> {
    let inner = s
        .trim_start_matches("rgba(")
        .trim_start_matches("rgb(")
        .trim_end_matches(')')
        .trim();

    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
    if parts.len() < 3 || parts.len() > 4 {
        return Err(format!(
            "rgb/rgba: нужно 3 или 4 аргумента, получено {}",
            parts.len()
        ));
    }

    let r = parse_0_255(parts[0])?;
    let g = parse_0_255(parts[1])?;
    let b = parse_0_255(parts[2])?;
    let a = if parts.len() == 4 {
        parts[3]
            .parse::<f32>()
            .map_err(|e| format!("rgb/rgba: альфа не число: {}", e))?
            .clamp(0.0, 1.0)
    } else {
        1.0
    };

    Ok(Rgba::from_rgb8_a(r, g, b, a))
}

fn parse_hsl(s: &str) -> Result<Rgba, String> {
    let inner = s
        .trim_start_matches("hsla(")
        .trim_start_matches("hsl(")
        .trim_end_matches(')')
        .trim();

    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
    if parts.len() < 3 || parts.len() > 4 {
        return Err(format!(
            "hsl/hsla: нужно 3 или 4 аргумента, получено {}",
            parts.len()
        ));
    }

    let h = parts[0]
        .parse::<f32>()
        .map_err(|e| format!("hsl: оттенок не число: {}", e))?;
    let s = parse_0_100(parts[1])?;
    let l = parse_0_100(parts[2])?;
    let a = if parts.len() == 4 {
        parts[3]
            .parse::<f32>()
            .map_err(|e| format!("hsla: альфа не число: {}", e))?
            .clamp(0.0, 1.0)
    } else {
        1.0
    };

    Ok(hsl_to_rgb(h, s, l).with_alpha(a))
}

fn parse_oklch(s: &str) -> Result<Rgba, String> {
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

    let l = parse_0_100(parts[0])?; // уже нормировано в 0..1
    let c = parts[1]
        .parse::<f32>()
        .map_err(|e| format!("oklch: насыщенность не число: {}", e))?
        .max(0.0);
    let h = parts[2]
        .parse::<f32>()
        .map_err(|e| format!("oklch: оттенок не число: {}", e))?;

    Ok(oklch_to_rgb(l, c, h))
}

fn parse_named(s: &str) -> Result<Rgba, String> {
    let s = s.trim().to_lowercase();

    if let Ok(idx) = NAMED_COLORS.binary_search_by_key(&s.as_str(), |(name, _)| name) {
        Ok(NAMED_COLORS[idx].1)
    } else {
        Err(format!("неизвестный цвет: «{}»", s))
    }
}

// —————— вспомогательные парсеры ——————

fn parse_0_255(s: &str) -> Result<u8, String> {
    let v: f32 = s
        .parse()
        .map_err(|e| format!("ожидалось число 0–255: {}", e))?;
    if v < 0.0 || v > 255.0 {
        return Err(format!("значение {} вне диапазона 0–255", v));
    }
    Ok(v as u8)
}

fn parse_0_100(s: &str) -> Result<f32, String> {
    let cleaned = s.trim_end_matches('%').trim();
    let v: f32 = cleaned
        .parse()
        .map_err(|e| format!("ожидалось число 0–100, получено «{}»: {}", s, e))?;
    if v < 0.0 || v > 100.0 {
        return Err(format!("значение {} вне диапазона 0–100", v));
    }
    Ok(v / 100.0) // нормируем в 0..1
}

// —————— конвертеры цветовых пространств ——————

/// HSL → sRGB по стандартной формуле.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Rgba {
    // h — градусы 0..360, s и l — нормированные 0..1
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

/// oklch → sRGB.
///
/// Аппроксимация по OKLCH → OKLab → линейный sRGB → гамма-коррекция.
fn oklch_to_rgb(l: f32, c: f32, h: f32) -> Rgba {
    // oklch → oklab
    let h_rad = h.to_radians();
    let a = c * h_rad.cos();
    let b_val = c * h_rad.sin();

    // oklab → линейный sRGB (матрица 3×3)
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b_val;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b_val;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b_val;

    let l3 = l_ * l_ * l_;
    let m3 = m_ * m_ * m_;
    let s3 = s_ * s_ * s_;

    // Преобразование LMS → sRGB
    let r_lin = 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
    let g_lin = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
    let b_lin = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3;

    // Гамма-коррекция (sRGB transfer function)
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

// —————— именованные цвета ——————

/// Базовый набор именованных цветов (алфавитный порядок для `binary_search`).
const NAMED_COLORS: &[(&str, Rgba)] = &[
    ("aqua", Rgba::new(0.0, 1.0, 1.0)),
    ("black", Rgba::new(0.0, 0.0, 0.0)),
    ("blue", Rgba::new(0.0, 0.0, 1.0)),
    ("cyan", Rgba::new(0.0, 1.0, 1.0)),
    ("fuchsia", Rgba::new(1.0, 0.0, 1.0)),
    ("gray", Rgba::new(0.5, 0.5, 0.5)),
    ("green", Rgba::new(0.0, 0.5, 0.0)),
    ("grey", Rgba::new(0.5, 0.5, 0.5)),
    ("lime", Rgba::new(0.0, 1.0, 0.0)),
    ("maroon", Rgba::new(0.5, 0.0, 0.0)),
    ("navy", Rgba::new(0.0, 0.0, 0.5)),
    ("olive", Rgba::new(0.5, 0.5, 0.0)),
    ("orange", Rgba::new(1.0, 0.647, 0.0)),
    ("pink", Rgba::new(1.0, 0.753, 0.796)),
    ("purple", Rgba::new(0.5, 0.0, 0.5)),
    ("red", Rgba::new(1.0, 0.0, 0.0)),
    ("silver", Rgba::new(0.753, 0.753, 0.753)),
    ("tan", Rgba::new(0.824, 0.706, 0.549)),
    ("teal", Rgba::new(0.0, 0.5, 0.5)),
    ("transparent", Rgba::new(0.0, 0.0, 0.0).with_alpha(0.0)),
    ("white", Rgba::new(1.0, 1.0, 1.0)),
    ("yellow", Rgba::new(1.0, 1.0, 0.0)),
];

// —————— тесты ——————

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_rgba {
        ($left:expr, $right:expr $(,)?) => {
            let l = $left;
            let r = $right;
            let eps = 0.005;
            assert!(
                (l.r - r.r).abs() < eps
                    && (l.g - r.g).abs() < eps
                    && (l.b - r.b).abs() < eps
                    && (l.a - r.a).abs() < eps,
                "left: {:?}, right: {:?}",
                l,
                r,
            );
        };
    }

    #[test]
    fn hex_3() {
        let c = parse_color("#F00").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn hex_4() {
        let c = parse_color("#F00F").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn hex_6() {
        let c = parse_color("#FF0000").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn hex_8() {
        let c = parse_color("#FF000080").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0).with_alpha(0.502));
    }

    #[test]
    fn hex_invalid_length() {
        assert!(parse_color("#FF").is_err());
    }

    #[test]
    fn rgb() {
        let c = parse_color("rgb(255, 0, 0)").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn rgba() {
        let c = parse_color("rgba(255, 0, 0, 0.5)").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0).with_alpha(0.5));
    }

    #[test]
    fn hsl() {
        let c = parse_color("hsl(0, 100%, 50%)").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn hsla() {
        let c = parse_color("hsla(0, 100%, 50%, 0.5)").unwrap();
        assert_rgba!(c, Rgba::new(1.0, 0.0, 0.0).with_alpha(0.5));
    }

    #[test]
    fn oklch() {
        // OKLCH(0.5 0.1 30) примерно соответствует sRGB (0.5, 0.3, 0.2)
        let c = parse_color("oklch(50% 0.1 30)").unwrap();
        // Проверка, что не паника и результат в 0..1
        assert!(c.r >= 0.0 && c.r <= 1.0);
        assert!(c.g >= 0.0 && c.g <= 1.0);
        assert!(c.b >= 0.0 && c.b <= 1.0);
    }

    #[test]
    fn named_colors() {
        assert_rgba!(parse_color("red").unwrap(), Rgba::new(1.0, 0.0, 0.0));
        assert_rgba!(
            parse_color("transparent").unwrap(),
            Rgba::new(0.0, 0.0, 0.0).with_alpha(0.0)
        );
        assert_rgba!(parse_color("white").unwrap(), Rgba::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn named_case_insensitive() {
        assert_rgba!(parse_color("RED").unwrap(), Rgba::new(1.0, 0.0, 0.0));
        assert_rgba!(
            parse_color("Transparent").unwrap(),
            Rgba::new(0.0, 0.0, 0.0).with_alpha(0.0)
        );
    }

    #[test]
    fn unknown_named() {
        assert!(parse_color("ultramarine").is_err());
    }

    #[test]
    fn detect_hex() {
        assert_eq!(detect_format("#FF0000"), Some(ColorFormat::Hex));
    }

    #[test]
    fn detect_rgb() {
        assert_eq!(detect_format("rgb(255,0,0)"), Some(ColorFormat::Rgb));
    }

    #[test]
    fn detect_hsl() {
        assert_eq!(detect_format("hsl(0,100%,50%)"), Some(ColorFormat::Hsl));
    }

    #[test]
    fn detect_oklch() {
        assert_eq!(detect_format("oklch(50% 0.1 30)"), Some(ColorFormat::Oklch));
    }

    #[test]
    fn detect_named() {
        assert_eq!(detect_format("red"), Some(ColorFormat::Named));
    }

    #[test]
    fn detect_unknown() {
        assert_eq!(detect_format("not-a-color"), None);
    }

    #[test]
    fn consistency_pass() {
        let colors = ["#FF0000", "#00FF00", "#0000FF"];
        assert_eq!(enforce_consistency(&colors).unwrap(), ColorFormat::Hex);
    }

    #[test]
    fn consistency_fail() {
        let colors = ["#FF0000", "rgb(0,255,0)"];
        assert!(enforce_consistency(&colors).is_err());
    }

    #[test]
    fn hsl_known_colors() {
        // Красный: hsl(0, 100%, 50%) → rgb(255, 0, 0)
        assert_rgba!(hsl_to_rgb(0.0, 1.0, 0.5), Rgba::new(1.0, 0.0, 0.0));
        // Зелёный: hsl(120, 100%, 50%) → rgb(0, 255, 0)
        assert_rgba!(hsl_to_rgb(120.0, 1.0, 0.5), Rgba::new(0.0, 1.0, 0.0));
        // Синий: hsl(240, 100%, 50%) → rgb(0, 0, 255)
        assert_rgba!(hsl_to_rgb(240.0, 1.0, 0.5), Rgba::new(0.0, 0.0, 1.0));
        // Белый: hsl(0, 0%, 100%)
        assert_rgba!(hsl_to_rgb(0.0, 0.0, 1.0), Rgba::new(1.0, 1.0, 1.0));
        // Чёрный: hsl(0, 0%, 0%)
        assert_rgba!(hsl_to_rgb(0.0, 0.0, 0.0), Rgba::new(0.0, 0.0, 0.0));
    }
}
