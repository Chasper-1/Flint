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

mod parsers;
mod named;

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
        parsers::parse_hex(s)
    } else if s.starts_with("rgba(") || s.starts_with("rgb(") {
        parsers::parse_rgb(s)
    } else if s.starts_with("hsla(") || s.starts_with("hsl(") {
        parsers::parse_hsl(s)
    } else if s.starts_with("oklch(") {
        parsers::parse_oklch(s)
    } else {
        parsers::parse_named(s)
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
    } else if named::NAMED_COLORS
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

#[cfg(test)]
mod tests;
