use cgmath::*;
use gl;
use render::color::*;
use render::vertex::*;

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
        unsafe {
            // Position 2D
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, // Index
                3, // Count
                gl::FLOAT, // Type
                gl::FALSE, // Normalized
                Self::VERTEX_SIZE as i32, // Stride
                (0) as *const _, // Offset
            );
            // Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, // Index
                gl::BGRA as i32, // Count
                gl::UNSIGNED_INT_2_10_10_10_REV, // Type
                gl::TRUE, // Normalized
                Self::VERTEX_SIZE as i32, // Stride
                (3 * 4) as *const _, // Offset
            );
        }
    }
}
