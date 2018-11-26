pub const RED: Color = Color { color: 0xFFF0_0000 };
pub const PURPLE: Color = Color { color: 0xE812_03C2 };
pub const BLUE: Color = Color { color: 0xC000_03FF };
pub const GREEN: Color = Color { color: 0xC00F_FC00 };
pub const YELLOW: Color = Color { color: 0xFFFF_FC00 };
pub const ORANGE: Color = Color { color: 0xFFFA_5400 };
pub const MAGENTA: Color = Color { color: 0xC0FF_00FF };
pub const WHITE: Color = Color{color: 0xFFFF_FFFF};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    color: u32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        let mut val = 0u32;
        val = val | (((alpha * 3f32) as u32) << 30u32);
        val = val | (((red * 1023f32) as u32) << 20u32);
        val = val | (((green * 1023f32) as u32) << 10u32);
        val = val | (((blue * 1023f32) as u32) << 0u32);
        Color { color: val }
    }
}
