use super::{ColorComponentType, ColorDescriptor, ColorLayoutFormat};

/// Simple RG8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RG8 {
    /// Represents the red color channel.
    pub r: u8,
    /// Represents the green color channel.
    pub g: u8,
}

impl RG8 {
    pub const fn new(red: u8, green: u8) -> RG8 {
        RG8 {
            r: red,
            g: green,
        }
    }

    /// Helper function to create this color from f32s.
    pub fn from_f32(red: f32, green: f32) -> RG8 {
        RG8 {
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
        }
    }
}

impl From<RG8> for (f32, f32) {
    fn from(x: RG8) -> Self {
        let r = (x.r as f32) / 255.0;
        let g = (x.g as f32) / 255.0;
        (r, g)
    }
}

impl From<RG8> for [f32; 2] {
    fn from(x: RG8) -> Self {
        let r = (x.r as f32) / 255.0;
        let g = (x.g as f32) / 255.0;
        [r, g]
    }
}

impl From<(f32, f32)> for RG8 {
    fn from(x: (f32, f32)) -> Self {
        let (r, g) = x;
        Self::from_f32(r, g)
    }
}

impl From<[f32; 2]> for RG8 {
    fn from(x: [f32; 2]) -> Self {
        let [r, g] = x;
        Self::from_f32(r, g)
    }
}

impl Default for RG8 {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
        }
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
