use cgmath::*;
use color::*;
use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pos: Vector3<f32>,
    size: Vector2<u16>,
    uv: Vector4<u16>,
    color: RGBA8,
    rotation: u8,
}

impl Quad {
    /// The Vector4's are in the order of (left, right, bottom, top).
    pub fn new(pos: Vector3<f32>, size: Vector2<f32>, uv: Vector4<u16>, color: RGBA8, rotation: f32) -> Quad {
        Quad {
            pos: pos,
            size: {
                let x = (size.x as u32) & 0xFFFF;
                let y = (size.y as u32) & 0xFFFF;
                Vector2::new(x as u16, y as u16)
            },
            uv: uv,
            color: color,
            rotation: (rotation.fract() * 255f32) as u8,
        }
    }
}
