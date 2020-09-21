use super::colors::BLACK;
use super::RGBA8;
use cgmath::*;

/// Token to reference a font with.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FontToken {
    key: usize,
}

impl FontToken {
    pub(crate) fn new(key: usize) -> FontToken {
        FontToken {
            key,
        }
    }

    pub(crate) fn key(&self) -> usize {
        self.key
    }
}

impl Default for FontToken {
    // The engine default font.
    fn default() -> FontToken {
        FontToken {
            key: 0,
        }
    }
}

/// Configuration settings for text.
#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    /// Text that's being drawn.
    pub string: String,
    /// Position of the text. The X and Y coordinates represent the bottom left corner of the text.
    /// The Z coordinate represents sprite depth. Units are measured in pixels.
    pub pos: Vector3<f32>,
    /// Max width of the text before it's pushed to a new line. If this is set to None, it will
    /// not wrap text to a new line. Units are measured in pixels. The default is None.
    pub max_width: Option<f32>,
    /// Amount to scale the text by as defined by the font. The default is 24.
    pub scale: u32,
    /// Color to use for the text. The default is black.
    pub color: RGBA8,
    /// Font to use for the text. This is the engine font by default.
    pub font: FontToken,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            string: String::new(),
            pos: Vector3::new(0f32, 0f32, 0f32),
            max_width: None,
            scale: 24,
            color: BLACK,
            font: FontToken::default(),
        }
    }
}

impl Text {
    pub fn new(
        string: String,
        pos: Vector3<f32>,
        max_width: Option<f32>,
        scale: u32,
        color: RGBA8,
        font: FontToken,
    ) -> Text {
        Text {
            string,
            pos,
            max_width,
            scale,
            color,
            font,
        }
    }

    pub fn set_string(&mut self, string: &str) {
        self.string.clear();
        self.string.push_str(&string);
    }
}
