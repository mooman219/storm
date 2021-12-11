use super::{ColorComponentType, ColorDescriptor, ColorLayoutFormat};

/// Simple RGB8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB8 {
    pub const RED: RGB8 = RGB8::new(255, 0, 0);
    pub const PURPLE: RGB8 = RGB8::new(128, 0, 128);
    pub const BLUE: RGB8 = RGB8::new(0, 0, 255);
    pub const GREEN: RGB8 = RGB8::new(0, 255, 0);
    pub const YELLOW: RGB8 = RGB8::new(255, 255, 0);
    pub const ORANGE: RGB8 = RGB8::new(255, 164, 0);
    pub const MAGENTA: RGB8 = RGB8::new(255, 0, 255);
    pub const WHITE: RGB8 = RGB8::new(255, 255, 255);
    pub const BLACK: RGB8 = RGB8::new(0, 0, 0);

    pub const fn new(red: u8, green: u8, blue: u8) -> RGB8 {
        RGB8 {
            r: red,
            g: green,
            b: blue,
        }
    }

    pub fn from_f32(red: f32, green: f32, blue: f32) -> RGB8 {
        RGB8 {
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
            b: (blue * 255.0) as u8,
        }
    }
}

impl Into<(f32, f32, f32)> for RGB8 {
    fn into(self) -> (f32, f32, f32) {
        ((self.r as f32) / 255.0, (self.g as f32) / 255.0, (self.b as f32) / 255.0)
    }
}

impl ColorDescriptor for RGB8 {
    fn component_type() -> ColorComponentType {
        ColorComponentType::U8
    }
    fn layout() -> ColorLayoutFormat {
        ColorLayoutFormat::RGB
    }
}
