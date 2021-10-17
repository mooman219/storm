use super::{ColorComponentType, ColorDescription, ColorLayoutFormat};

/// Simple R8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct R8 {
    pub r: u8,
}

impl R8 {
    pub const fn new(red: u8) -> R8 {
        R8 {
            r: red,
        }
    }

    pub fn from_f32(red: f32) -> R8 {
        R8 {
            r: (red * 255.0) as u8,
        }
    }
}

impl Into<f32> for R8 {
    fn into(self) -> f32 {
        (self.r as f32) / 255.0
    }
}

impl ColorDescription for R8 {
    fn component_type() -> ColorComponentType {
        ColorComponentType::U8
    }
    fn layout() -> ColorLayoutFormat {
        ColorLayoutFormat::R
    }
}
