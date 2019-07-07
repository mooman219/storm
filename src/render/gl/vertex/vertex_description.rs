use crate::render::gl::raw::*;
use crate::types::*;
use std::mem;

pub trait VertexDescription {
    const VERTEX_SIZE: usize;

    fn configure_vertex_attribute();
}

impl VertexDescription for SpriteDescription {
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
            index,                        // Index
            1,                            // Count
            AttributeType::UnsignedShort, // Type
            true,                         // Normalized
            Self::VERTEX_SIZE as i32,     // Stride
            size as *const _,             // Offset
        );
        //index += 1;
        // size += 1 * 2; // Count * Bytes
        // warn!("{}, {}", size, std::mem::size_of::<SpriteDescription>()); // DEBUG
    }
}
