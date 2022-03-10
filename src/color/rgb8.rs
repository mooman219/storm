use super::{ColorComponentType, ColorDescriptor, ColorLayoutFormat};

/// Simple RGB8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RGB8 {
    /// Represents the red color channel.
    pub r: u8,
    /// Represents the green color channel.
    pub g: u8,
    /// Represents the blue color channel.
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

    /// Helper function to create this color from f32s.
    pub fn from_f32(red: f32, green: f32, blue: f32) -> RGB8 {
        RGB8 {
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
            b: (blue * 255.0) as u8,
        }
    }
}

impl From<RGB8> for (f32, f32, f32) {
    fn from(x: RGB8) -> Self {
        let r = (x.r as f32) / 255.0;
        let g = (x.g as f32) / 255.0;
        let b = (x.b as f32) / 255.0;
        (r, g, b)
    }
}

impl From<RGB8> for [f32; 3] {
    fn from(x: RGB8) -> Self {
        let r = (x.r as f32) / 255.0;
        let g = (x.g as f32) / 255.0;
        let b = (x.b as f32) / 255.0;
        [r, g, b]
    }
}

impl From<(f32, f32, f32)> for RGB8 {
    fn from(x: (f32, f32, f32)) -> Self {
        let (r, g, b) = x;
        Self::from_f32(r, g, b)
    }
}

impl From<[f32; 3]> for RGB8 {
    fn from(x: [f32; 3]) -> Self {
        let [r, g, b] = x;
        Self::from_f32(r, g, b)
    }
}

impl Default for RGB8 {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
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
