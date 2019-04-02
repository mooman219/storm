/// A default texture reference for a basic white square.
pub const DEFAULT_TEXTURE: TextureReference = TextureReference { key: 0 };

/// Handle to reference an uploaded texture with.
#[derive(Copy, Clone, Debug)]
pub struct TextureReference {
    key: usize,
}

impl TextureReference {
    pub fn new(key: usize) -> TextureReference {
        TextureReference { key: key }
    }

    pub fn key(&self) -> usize {
        self.key
    }
}
