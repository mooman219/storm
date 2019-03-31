pub const DEFAULT_TEXTURE: TextureReference = TextureReference { key: 0 };

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
