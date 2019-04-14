use cgmath::*;
use color::*;

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

/// Configuration description for text.
#[derive(Copy, Clone, Debug)]
pub struct TextDescription {
    pub pos: Vector3<f32>,
    pub max_width: Option<f32>,
    pub scale: u32,
    pub color: Color,
    pub font: FontReference,
}

impl Default for TextDescription {
    fn default() -> TextDescription {
        TextDescription {
            pos: Vector3::new(0f32, 0f32, 0f32),
            max_width: None,
            scale: 24,
            color: BLACK,
            font: DEFAULT_FONT,
        }
    }
}
