use std::collections::HashMap;

use peniko::Font;

pub(super) struct FontCache {
    fonts: HashMap<String, Font>,
}

impl FontCache {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    pub fn set(&mut self, name: String, font: Font) {
        self.fonts.insert(name, font);
    }
}
