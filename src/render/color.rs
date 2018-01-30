const RED: Color = Color { color: 0xC000_03FF };
const GREEN: Color = Color { color: 0xC00F_FC00 };
const BLUE: Color = Color { color: 0xFFF0_0000 };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    color: u32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
        let mut val = 0u32;
        val = val | (((alpha * 3f32) as u32) << 30u32);
        val = val | (((blue * 1023f32) as u32) << 20u32);
        val = val | (((green * 1023f32) as u32) << 10u32);
        val = val | (((red * 1023f32) as u32) << 0u32);
        Color { color: val }
    }
}
