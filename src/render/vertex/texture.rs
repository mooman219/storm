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
    uv: Vector4<u16>,
    color: Color,
    rotation: u8,
}

impl TextureVertex {
    /// The Vector4's are in the order of (left, right, bottom, top).
    pub fn new(
        pos: Vector3<f32>,
        size: Vector2<f32>,
        uv: Vector4<u16>,
        color: Color,
        rotation: f32,
    ) -> TextureVertex {
        TextureVertex {
            pos_z: pos.z,
            pos: Vector4::new(pos.x, pos.x + size.x, pos.y, pos.y + size.y),
            uv: uv,
            color: color,
            rotation: (rotation.fract() * 255f32) as u8,
        }
    }
}

impl Vertex for TextureVertex {
    const VERTEX_SIZE: usize = mem::size_of::<Self>();

    fn configure_vertex_attribute() {
        let mut size = 0;

        // Position Z
        enable_vertex_attrib_array(0);
        vertex_attrib_divisor(0, 1);
        vertex_attrib_pointer(
            0,                        // Index
            1,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            size as *const _,         // Offset
        );
        size += 1 * 4; // Count * Bytes

        // Position
        enable_vertex_attrib_array(1);
        vertex_attrib_divisor(1, 1);
        vertex_attrib_pointer(
            1,                        // Index
            4,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            size as *const _,         // Offset
        );
        size += 4 * 4; // Count * Bytes

        // UV
        enable_vertex_attrib_array(2);
        vertex_attrib_divisor(2, 1);
        vertex_attrib_pointer(
            2,                            // Index
            4,                            // Count
            AttributeType::UnsignedShort, // Type
            true,                         // Normalized
            Self::VERTEX_SIZE as i32,     // Stride
            size as *const _,             // Offset
        );
        size += 4 * 2; // Count * Bytes

        // Color
        enable_vertex_attrib_array(3);
        vertex_attrib_divisor(3, 1);
        vertex_attrib_pointer(
            3,                           // Index
            4,                           // Count
            AttributeType::UnsignedByte, // Type
            true,                        // Normalized
            Self::VERTEX_SIZE as i32,    // Stride
            size as *const _,            // Offset
        );
        size += 4 * 1; // Count * Bytes

        // Rotation
        enable_vertex_attrib_array(4);
        vertex_attrib_divisor(4, 1);
        vertex_attrib_pointer(
            4,                           // Index
            1,                           // Count
            AttributeType::UnsignedByte, // Type
            true,                        // Normalized
            Self::VERTEX_SIZE as i32,    // Stride
            size as *const _,            // Offset
        );
        //size += 1 * 1; // Count * Bytes
    }
}
