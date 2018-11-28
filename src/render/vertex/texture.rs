use cgmath::*;
use render::color::*;
use render::raw::*;
use render::vertex::*;
use std::mem;

#[repr(C)]
pub struct TextureVertex {
    position: Vector3<f32>,
    uv: Vector2<f32>,
    color: Color,
}

impl TextureVertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32, color: Color) -> TextureVertex {
        TextureVertex {
            position: Vector3::new(x, y, z),
            uv: Vector2::new(u, v),
            color: color,
        }
    }
}

impl Vertex for TextureVertex {
    const VERTEX_SIZE: usize = mem::size_of::<Self>();

    fn configure_vertex_attribute() {
        // Position 2D
        enable_vertex_attrib_array(0);
        vertex_attrib_pointer(
            0,                        // Index
            3,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            (0) as *const _,          // Offset
        );
        // UV
        enable_vertex_attrib_array(1);
        vertex_attrib_pointer(
            1,                        // Index
            2,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            (3 * 4) as *const _,      // Offset
        );
        // Color
        enable_vertex_attrib_array(2);
        vertex_attrib_pointer(
            2,                           // Index
            4,                           // Count
            AttributeType::UnsignedByte, // Type
            true,                        // Normalized
            Self::VERTEX_SIZE as i32,    // Stride
            (5 * 4) as *const _,         // Offset
        );
    }
}
