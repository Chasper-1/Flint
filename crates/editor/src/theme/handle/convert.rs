//! Трейты конвертации значений в/из [`HandleValue`].

use super::HandleValue;

/// Преобразование значения в [`HandleValue`].
pub trait IntoHandleValue {
    fn into_handle_value(self) -> HandleValue;
}

/// Обратное преобразование из [`HandleValue`].
pub trait FromHandleValue: Sized {
    fn from_handle_value(v: &HandleValue) -> Option<Self>;
}

// float
impl IntoHandleValue for f32 {
    fn into_handle_value(self) -> HandleValue {
        HandleValue::Float(self)
    }
}
impl FromHandleValue for f32 {
    fn from_handle_value(v: &HandleValue) -> Option<Self> {
        match v {
            HandleValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

// Rgba
impl IntoHandleValue for crate::theme::color::Rgba {
    fn into_handle_value(self) -> HandleValue {
        HandleValue::Rgba(self)
    }
}
impl FromHandleValue for crate::theme::color::Rgba {
    fn from_handle_value(v: &HandleValue) -> Option<Self> {
        match v {
            HandleValue::Rgba(c) => Some(*c),
            _ => None,
        }
    }
}

// String
impl IntoHandleValue for String {
    fn into_handle_value(self) -> HandleValue {
        HandleValue::String(self)
    }
}
impl FromHandleValue for String {
    fn from_handle_value(v: &HandleValue) -> Option<Self> {
        match v {
            HandleValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}
