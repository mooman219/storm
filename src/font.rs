/// A default font reference for a basic white square.
pub const DEFAULT_FONT: FontReference = FontReference { key: 0 };

/// Handle to reference an uploaded font with.
#[derive(Copy, Clone, Debug)]
pub struct FontReference {
    key: usize,
}

impl FontReference {
    pub(crate) fn new(key: usize) -> FontReference {
        FontReference { key: key }
    }

    pub(crate) fn key(&self) -> usize {
        self.key
    }
}
