use cgmath::*;
use color::*;
use layer::*;
use utility::unordered_tracker::*;

/// A default font reference for a basic white square.
pub const DEFAULT_FONT: FontReference = FontReference {
    key: 0,
};

/// Handle to reference an uploaded font with.
#[derive(Copy, Clone, Debug)]
pub struct FontReference {
    key: usize,
}

impl FontReference {
    pub(crate) fn new(key: usize) -> FontReference {
        FontReference {
            key: key,
        }
    }

    pub(crate) fn key(&self) -> usize {
        self.key
    }
}

/// Handle to reference an text with.
#[derive(Copy, Clone, Debug)]
pub struct TextReference {
    key: Key<TextReference>,
    layer: LayerReference,
}

impl TextReference {
    pub(crate) fn new(key: Key<TextReference>, layer: LayerReference) -> TextReference {
        TextReference {
            key: key,
            layer: layer,
        }
    }

    pub(crate) fn key(&self) -> Key<TextReference> {
        self.key
    }

    pub(crate) fn layer(&self) -> &LayerReference {
        &self.layer
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

impl TextDescription {
    pub fn pos(&mut self, pos: Vector3<f32>) -> &mut TextDescription {
        self.pos = pos;
        self
    }

    pub fn max_width(&mut self, max_width: Option<f32>) -> &mut TextDescription {
        self.max_width = max_width;
        self
    }

    pub fn scale(&mut self, scale: u32) -> &mut TextDescription {
        self.scale = scale;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut TextDescription {
        self.color = color;
        self
    }

    pub fn font(&mut self, font: FontReference) -> &mut TextDescription {
        self.font = font;
        self
    }
}
