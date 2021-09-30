use cgmath::*;

pub mod colors {
    use super::*;
    pub const RED: RGBA8 = RGBA8::new_raw(255, 0, 0, 255);
    pub const PURPLE: RGBA8 = RGBA8::new_raw(128, 0, 128, 255);
    pub const BLUE: RGBA8 = RGBA8::new_raw(0, 0, 255, 255);
    pub const GREEN: RGBA8 = RGBA8::new_raw(0, 255, 0, 255);
    pub const YELLOW: RGBA8 = RGBA8::new_raw(255, 255, 0, 255);
    pub const ORANGE: RGBA8 = RGBA8::new_raw(255, 164, 0, 255);
    pub const MAGENTA: RGBA8 = RGBA8::new_raw(255, 0, 255, 255);
    pub const WHITE: RGBA8 = RGBA8::new_raw(255, 255, 255, 255);
    pub const BLACK: RGBA8 = RGBA8::new_raw(0, 0, 0, 255);
    pub const TRANSPARENT: RGBA8 = RGBA8::new_raw(0, 0, 0, 0);
}

/// Simple RGBA8 color type to represent colors.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA8 {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> RGBA8 {
        RGBA8 {
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
            b: (blue * 255.0) as u8,
            a: (alpha * 255.0) as u8,
        }
    }

    pub const fn new_raw(red: u8, green: u8, blue: u8, alpha: u8) -> RGBA8 {
        RGBA8 {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }
}

impl Into<Vector4<f32>> for RGBA8 {
    fn into(self) -> Vector4<f32> {
        Vector4::new(
            (self.r as f32) / 255.0,
            (self.g as f32) / 255.0,
            (self.b as f32) / 255.0,
            (self.a as f32) / 255.0,
        )
    }
}

impl Into<(f32, f32, f32, f32)> for RGBA8 {
    fn into(self) -> (f32, f32, f32, f32) {
        ((self.r as f32) / 255.0, (self.g as f32) / 255.0, (self.b as f32) / 255.0, (self.a as f32) / 255.0)
    }
}
