pub mod convert;

/// Описание одной ручки темы.
#[derive(Debug, Clone)]
pub struct Handle<T> {
    pub category: &'static str,
    pub name: &'static str,
    pub default: T,
}

impl<T> Handle<T> {
    /// Строковый ключ для использования в `ThemeSystem`.
    pub fn key(&self) -> String {
        format!("{}.{}", self.category, self.name)
    }
}

/// Типизированное значение ручки внутри [`ThemeSystem`].
#[derive(Debug, Clone)]
pub enum HandleValue {
    Float(f32),
    Rgba(crate::theme::color::Rgba),
    String(String),
}

/// Система управления ручками темы.
#[derive(Debug, Clone)]
pub struct ThemeSystem {
    values: std::collections::HashMap<String, HandleValue>,
}

impl ThemeSystem {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }

    pub fn set<T: convert::IntoHandleValue>(&mut self, handle: &Handle<T>, value: T) {
        self.values
            .insert(handle.key(), value.into_handle_value());
    }

    pub fn get<T: convert::FromHandleValue>(&self, handle: &Handle<T>) -> Option<T> {
        self.values
            .get(&handle.key())
            .and_then(|v| T::from_handle_value(v))
    }

    pub fn get_or_default<T: convert::FromHandleValue + Clone + convert::IntoHandleValue>(&self, handle: &Handle<T>) -> T {
        self.get(handle).unwrap_or_else(|| {
            T::from_handle_value(&handle.default.clone().into_handle_value())
                .expect("Handle::default должен конвертироваться")
        })
    }

    pub fn set_raw(&mut self, path: &str, value: HandleValue) {
        self.values.insert(path.to_string(), value);
    }

    pub fn get_raw(&self, path: &str) -> Option<HandleValue> {
        self.values.get(path).cloned()
    }

    pub fn reset(&mut self) {
        self.values.clear();
    }
}

impl Default for ThemeSystem {
    fn default() -> Self {
        Self::new()
    }
}
