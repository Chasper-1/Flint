use cosmic_text::*;

pub struct EditorState {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
    pub buffer: Buffer,
}

impl EditorState {
    pub fn new() -> Self {
        let mut font_system = FontSystem::new();
        let metrics = Metrics::new(20.0, 28.0);

        let buffer = Buffer::new(&mut font_system, metrics);

        Self {
            font_system,
            swash_cache: SwashCache::new(),
            buffer,
        }
    }
}