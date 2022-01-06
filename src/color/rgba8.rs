use super::{ColorComponentType, ColorDescriptor, ColorLayoutFormat};

/// Simple RGBA8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RGBA8 {
    /// Represents the red color channel.
    pub r: u8,
    /// Represents the green color channel.
    pub g: u8,
    /// Represents the blue color channel.
    pub b: u8,
    /// Represents the alpha color channel.
    pub a: u8,
}

impl RGBA8 {
    pub const RED: RGBA8 = RGBA8::new(255, 0, 0, 255);
    pub const PURPLE: RGBA8 = RGBA8::new(128, 0, 128, 255);
    pub const BLUE: RGBA8 = RGBA8::new(0, 0, 255, 255);
    pub const GREEN: RGBA8 = RGBA8::new(0, 255, 0, 255);
    pub const YELLOW: RGBA8 = RGBA8::new(255, 255, 0, 255);
    pub const ORANGE: RGBA8 = RGBA8::new(255, 164, 0, 255);
    pub const MAGENTA: RGBA8 = RGBA8::new(255, 0, 255, 255);
    pub const WHITE: RGBA8 = RGBA8::new(255, 255, 255, 255);
    pub const BLACK: RGBA8 = RGBA8::new(0, 0, 0, 255);
    pub const TRANSPARENT: RGBA8 = RGBA8::new(0, 0, 0, 0);

    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> RGBA8 {
        RGBA8 {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    /// Helper function to create this color from f32s.
    pub fn from_f32(red: f32, green: f32, blue: f32, alpha: f32) -> RGBA8 {
        RGBA8 {
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
            b: (blue * 255.0) as u8,
            a: (alpha * 255.0) as u8,
        }
    }
}

impl Into<(f32, f32, f32, f32)> for RGBA8 {
    fn into(self) -> (f32, f32, f32, f32) {
        ((self.r as f32) / 255.0, (self.g as f32) / 255.0, (self.b as f32) / 255.0, (self.a as f32) / 255.0)
    }
}

impl ColorDescriptor for RGBA8 {
    fn component_type() -> ColorComponentType {
        ColorComponentType::U8
    }
    fn layout() -> ColorLayoutFormat {
        ColorLayoutFormat::RGBA
    }
}
