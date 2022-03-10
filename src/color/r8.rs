use super::{ColorComponentType, ColorDescriptor, ColorLayoutFormat};

/// Simple R8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct R8 {
    /// Represents the red color channel.
    pub r: u8,
}

impl R8 {
    pub const fn new(red: u8) -> R8 {
        R8 {
            r: red,
        }
    }

    /// Helper function to create this color from an f32.
    pub fn from_f32(red: f32) -> R8 {
        R8 {
            r: (red * 255.0) as u8,
        }
    }
}

impl From<R8> for f32 {
    fn from(x: R8) -> Self {
        (x.r as f32) / 255.0
    }
}

impl From<f32> for R8 {
    fn from(r: f32) -> Self {
        Self::from_f32(r)
    }
}

impl Default for R8 {
    fn default() -> Self {
        Self {
            r: 255,
        }
    }
}

impl ColorDescriptor for R8 {
    fn component_type() -> ColorComponentType {
        ColorComponentType::U8
    }
    fn layout() -> ColorLayoutFormat {
        ColorLayoutFormat::R
    }
}
