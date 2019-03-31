pub const DEFAULT_TEXTURE: TextureReference = TextureReference { key: 0 };

#[derive(Copy, Clone, Debug)]
pub struct TextureReference {
    pub(crate) key: usize,
}
