use cgmath::*;
use render::color::*;
use render::raw::*;
use render::vertex::*;
use std::mem;

#[repr(C)]
pub struct ColorVertex {
    position: Vector3<f32>,
    color: Color,
}

impl ColorVertex {
    pub fn new(x: f32, y: f32, z: f32, color: Color) -> ColorVertex {
        ColorVertex {
            position: Vector3::new(x, y, z),
            color: color,
        }
    }
}

impl Vertex for ColorVertex {
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
        // Color
        enable_vertex_attrib_array(1);
        vertex_attrib_pointer(
            1,                                        // Index
            0x80E1,                                   // Count (gl::BGRA = 0x80E1)
            AttributeType::UnsignedInt2_10_10_10_Rev, // Type
            true,                                     // Normalized
            Self::VERTEX_SIZE as i32,                 // Stride
            (3 * 4) as *const _,                      // Offset
        );
    }
}
