use cgmath::*;
use color::*;
use render::gl::raw::*;
use render::gl::vertex::*;
use std::mem;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TextureVertex {
    pos: Vector3<f32>,
    size: Vector2<u16>,
    uv: Vector4<u16>,
    color: RGBA8,
    rotation: u8,
}

impl TextureVertex {
    /// The Vector4's are in the order of (left, right, bottom, top).
    pub fn new(
        pos: Vector3<f32>,
        size: Vector2<f32>,
        uv: Vector4<u16>,
        color: RGBA8,
        rotation: f32,
    ) -> TextureVertex {
        TextureVertex {
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

impl Vertex for TextureVertex {
    const VERTEX_SIZE: usize = mem::size_of::<Self>();

    fn configure_vertex_attribute() {
        let mut index = 0;
        let mut size = 0;

        // Position
        enable_vertex_attrib_array(index);
        vertex_attrib_divisor(index, 1);
        vertex_attrib_pointer(
            index,                    // Index
            3,                        // Count
            AttributeType::Float,     // Type
            false,                    // Normalized
            Self::VERTEX_SIZE as i32, // Stride
            size as *const _,         // Offset
        );
        index += 1;
        size += 3 * 4; // Count * Bytes

        // Size
        enable_vertex_attrib_array(index);
        vertex_attrib_divisor(index, 1);
        vertex_attrib_pointer(
            index,                        // Index
            2,                            // Count
            AttributeType::UnsignedShort, // Type
            true,                         // Normalized
            Self::VERTEX_SIZE as i32,     // Stride
            size as *const _,             // Offset
        );
        index += 1;
        size += 2 * 2; // Count * Bytes

        // UV
        enable_vertex_attrib_array(index);
        vertex_attrib_divisor(index, 1);
        vertex_attrib_pointer(
            index,                        // Index
            4,                            // Count
            AttributeType::UnsignedShort, // Type
            true,                         // Normalized
            Self::VERTEX_SIZE as i32,     // Stride
            size as *const _,             // Offset
        );
        index += 1;
        size += 4 * 2; // Count * Bytes

        // RGBA8
        enable_vertex_attrib_array(index);
        vertex_attrib_divisor(index, 1);
        vertex_attrib_pointer(
            index,                       // Index
            4,                           // Count
            AttributeType::UnsignedByte, // Type
            true,                        // Normalized
            Self::VERTEX_SIZE as i32,    // Stride
            size as *const _,            // Offset
        );
        index += 1;
        size += 4 * 1; // Count * Bytes

        // Rotation
        enable_vertex_attrib_array(index);
        vertex_attrib_divisor(index, 1);
        vertex_attrib_pointer(
            index,                       // Index
            1,                           // Count
            AttributeType::UnsignedByte, // Type
            true,                        // Normalized
            Self::VERTEX_SIZE as i32,    // Stride
            size as *const _,            // Offset
        );
        //index += 1;
        //size += 1 * 1; // Count * Bytes
    }
}
