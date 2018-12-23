pub const RED: Color = Color::new_raw(255, 0, 0, 255);
pub const PURPLE: Color = Color::new_raw(128, 0, 128, 255);
pub const BLUE: Color = Color::new_raw(0, 0, 255, 255);
pub const GREEN: Color = Color::new_raw(0, 255, 0, 255);
pub const YELLOW: Color = Color::new_raw(255, 255, 0, 255);
pub const ORANGE: Color = Color::new_raw(255, 164, 0, 255);
pub const MAGENTA: Color = Color::new_raw(255, 0, 255, 255);
pub const WHITE: Color = Color::new_raw(255, 255, 255, 255);
pub const BLACK: Color = Color::new_raw(0, 0, 0, 255);
pub const TRANSPARENT: Color = Color::new_raw(0, 0, 0, 0);


use std::cmp::{PartialEq, Eq};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        Color {
            r: (red * 255f32) as u8,
            g: (green * 255f32) as u8,
            b: (blue * 255f32) as u8,
            a: (alpha * 255f32) as u8,
        }
    }

    pub const fn new_raw(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }
}

impl Eq for Color {}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        return self.r == other.r && self.b == other.b && self.g == other.g && self.a == other.a;
    }
}