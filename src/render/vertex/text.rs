use crate::prelude::TextSprite;
use crate::render::raw::{AttributeType, OpenGL};
use crate::render::vertex::VertexDescription;
use core::mem;

impl VertexDescription for TextSprite {
    const VERTEX_SIZE: usize = mem::size_of::<Self>();

    fn configure_vertex_attribute(gl: &OpenGL) {
        let mut index = 0;
        let mut size = 0;

        // Position
        gl.enable_vertex_attrib_array(index);
        gl.vertex_attrib_divisor(index, 1);
        gl.vertex_attrib_pointer_f32(index, 3, AttributeType::Float, false, Self::VERTEX_SIZE as i32, size);
        index += 1;
        size += 3 * 4;

        // Size
        gl.enable_vertex_attrib_array(index);
        gl.vertex_attrib_divisor(index, 1);
        gl.vertex_attrib_pointer_f32(
            index,
            2,
            AttributeType::UnsignedShort,
            false,
            Self::VERTEX_SIZE as i32,
            size,
        );
        index += 1;
        size += 2 * 2;

        // UV
        gl.enable_vertex_attrib_array(index);
        gl.vertex_attrib_divisor(index, 1);
        gl.vertex_attrib_pointer_f32(
            index,
            4,
            AttributeType::UnsignedShort,
            true,
            Self::VERTEX_SIZE as i32,
            size,
        );
        index += 1;
        size += 4 * 2;

        // RGBA8
        gl.enable_vertex_attrib_array(index);
        gl.vertex_attrib_divisor(index, 1);
        gl.vertex_attrib_pointer_f32(
            index,
            4,
            AttributeType::UnsignedByte,
            true,
            Self::VERTEX_SIZE as i32,
            size,
        );

        // index += 1;
        // size += 4 * 1;
        // warn!("Bytes {}, Size {}", size, core::mem::size_of::<TextSprite>()); // DEBUG
    }
}
