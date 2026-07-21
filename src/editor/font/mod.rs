//! Управление шрифтами: системные + встроенные.
//!
//! Единая точка доступа к `cosmic_text::FontSystem` и `SwashCache`.
//! Инициализируется один раз при старте, грузит системные шрифты через fontdb.
//!
//! # Пример
//! ```ignore
//! font::init();
//! font::with_font_system(|fs| { /* шейпинг */ });
//! ```

use std::sync::{Mutex, OnceLock, PoisonError};

use cosmic_text::fontdb;

// ---------------------------------------------------------------------------
// Глобальное состояние
// ---------------------------------------------------------------------------

struct FontGlobal {
    font_system: cosmic_text::FontSystem,
    swash_cache: cosmic_text::SwashCache,
}

/// Глобальный синглтон. Инициализируется один раз в [`init()`].
static GLOBAL: OnceLock<Mutex<FontGlobal>> = OnceLock::new();

// ---------------------------------------------------------------------------
// Публичное API
// ---------------------------------------------------------------------------

/// Проинициализировать глобальный `FontSystem` (системные + встроенные шрифты).
///
/// Безопасно вызывать многократно — второй вызов игнорируется.
pub fn init() {
    let _ = GLOBAL.get_or_init(|| {
        let mut db = fontdb::Database::new();
        db.load_system_fonts(); // все шрифты ОС
        // TODO: загрузить встроенные .ttf из ресурсов

        let font_system = cosmic_text::FontSystem::new_with_locale_and_db(
            // locale можно брать из окружения, пока en
            "en".to_string(),
            db,
        );

        Mutex::new(FontGlobal {
            font_system,
            swash_cache: cosmic_text::SwashCache::new(),
        })
    });
}

/// Доступ к `FontSystem` для шейпинга.
pub fn with_font_system<F, T>(f: F) -> T
where
    F: FnOnce(&mut cosmic_text::FontSystem) -> T,
{
    let lock = GLOBAL
        .get()
        .expect("font::init() must be called before with_font_system()");
    let mut guard = lock.lock().unwrap_or_else(PoisonError::into_inner);
    f(&mut guard.font_system)
}

/// Доступ к `SwashCache` для растрирования.
pub fn with_swash_cache<F, T>(f: F) -> T
where
    F: FnOnce(&mut cosmic_text::SwashCache) -> T,
{
    let lock = GLOBAL
        .get()
        .expect("font::init() must be called before with_swash_cache()");
    let mut guard = lock.lock().unwrap_or_else(PoisonError::into_inner);
    f(&mut guard.swash_cache)
}

/// Доступ к `FontSystem` и `SwashCache` одновременно.
///
/// Нужен для растеризации глифов: `buffer.draw()` требует оба ресурса
/// сразу, а вложенный вызов `with_font_system` + `with_swash_cache`
/// привёл бы к взаимной блокировке одного и того же `Mutex`.
pub fn with_font_and_cache<F, T>(f: F) -> T
where
    F: FnOnce(&mut cosmic_text::FontSystem, &mut cosmic_text::SwashCache) -> T,
{
    let lock = GLOBAL
        .get()
        .expect("font::init() must be called before with_font_and_cache()");
    let mut guard = lock.lock().unwrap_or_else(PoisonError::into_inner);
    let FontGlobal { font_system, swash_cache } = &mut *guard;
    f(font_system, swash_cache)
}

/// Список всех доступных семейств шрифтов (системные + встроенные).
pub fn list_families() -> Vec<String> {
    let lock = GLOBAL
        .get()
        .expect("font::init() must be called before list_families()");
    let guard = lock.lock().unwrap_or_else(PoisonError::into_inner);
    let mut families: Vec<String> = guard
        .font_system
        .db()
        .faces()
        .flat_map(|f| f.families.iter().map(|(name, _)| name.clone()))
        .collect();
    families.sort();
    families.dedup();
    families
}

/// Пересканировать системные шрифты (полезно после установки нового шрифта).
pub fn reload_system_fonts() {
    let lock = GLOBAL
        .get()
        .expect("font::init() must be called before reload_system_fonts()");
    let mut guard = lock.lock().unwrap_or_else(PoisonError::into_inner);
    guard.font_system.db_mut().load_system_fonts();
}

// ---------------------------------------------------------------------------
// Тесты
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_and_list_families() {
        init();
        let families = list_families();
        assert!(!families.is_empty(), "Должен быть хотя бы один шрифт");
        assert!(
            families.iter().any(|f| f.to_lowercase().contains("sans")
                || f.to_lowercase().contains("serif")
                || f.to_lowercase().contains("mono")),
            "Ожидаются стандартные семейства, получено: {families:?}"
        );
    }

    #[test]
    fn with_font_system_works() {
        init();
        let metrics = with_font_system(|fs| {
            let buf = cosmic_text::Buffer::new_empty(cosmic_text::Metrics::new(14.0, 19.6));
            buf.metrics()
        });
        assert_eq!(metrics.font_size, 14.0);
    }
}
