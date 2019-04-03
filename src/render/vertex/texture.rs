use cgmath::*;
use color::*;
use render::raw::*;
use render::vertex::*;
use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextureVertex {
    pos_z: f32,
    pos: Vector4<f32>,
    uv: Vector4<f32>,
    color: Color,
}

impl TextureVertex {
    /// The Vector4's are in the order of (left, right, bottom, top).
    pub fn new(pos: Vector3<f32>, size: Vector2<f32>, uv: Vector4<f32>, color: Color) -> TextureVertex {
        TextureVertex {
            pos_z: pos.z,
            pos: Vector4::new(pos.x, pos.x + size.x, pos.y, pos.y + size.y),
            uv: uv,
            color: color,
        }
    }

    /// The Vector4's are in the order of (left, right, bottom, top).
    pub fn new_raw(pos_z: f32, pos: Vector4<f32>, uv: Vector4<f32>, color: Color) -> TextureVertex {
        TextureVertex {
            pos_z: pos_z,
            pos: pos,
            uv: uv,
            color: color,
        }
    }
}

impl Vertex for TextureVertex {
    const VERTEX_SIZE: usize = mem::size_of::<Self>();

    fn configure_vertex_attribute() {
        // Position Z
        enable_vertex_attrib_array(0);
        vertex_attrib_divisor(0, 1);
        vertex_attrib_pointer(
            0,                        // Index
            1,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            (0) as *const _,          // Offset
        );
        // Position
        enable_vertex_attrib_array(1);
        vertex_attrib_divisor(1, 1);
        vertex_attrib_pointer(
            1,                        // Index
            4,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            (1 * 4) as *const _,      // Offset
        );
        // UV
        enable_vertex_attrib_array(2);
        vertex_attrib_divisor(2, 1);
        vertex_attrib_pointer(
            2,                        // Index
            4,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            (5 * 4) as *const _,      // Offset
        );
        // Color
        enable_vertex_attrib_array(3);
        vertex_attrib_divisor(3, 1);
        vertex_attrib_pointer(
            3,                           // Index
            4,                           // Count
            AttributeType::UnsignedByte, // Type
            true,                        // Normalized
            Self::VERTEX_SIZE as i32,    // Stride
            (9 * 4) as *const _,         // Offset
        );
    }
}
