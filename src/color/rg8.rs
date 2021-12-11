use super::{ColorComponentType, ColorDescriptor, ColorLayoutFormat};

/// Simple RG8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RG8 {
    pub r: u8,
    pub g: u8,
}

impl RG8 {
    pub const fn new(red: u8, green: u8) -> RG8 {
        RG8 {
            r: red,
            g: green,
        }
    }

    pub fn from_f32(red: f32, green: f32) -> RG8 {
        RG8 {
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
        }
    }
}

impl Into<(f32, f32)> for RG8 {
    fn into(self) -> (f32, f32) {
        ((self.r as f32) / 255.0, (self.g as f32) / 255.0)
    }
}

impl ColorDescriptor for RG8 {
    fn component_type() -> ColorComponentType {
        ColorComponentType::U8
    }
    fn layout() -> ColorLayoutFormat {
        ColorLayoutFormat::RG
    }
}
