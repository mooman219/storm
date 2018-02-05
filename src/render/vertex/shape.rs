use cgmath::*;
use gl;
use render::color::*;
use render::vertex::*;

#[repr(C)]
pub struct ShapeVertex {
    position: Vector2<f32>,
    rotation: f32,
    color: Color,
}

impl ShapeVertex {
    pub fn new(x: f32, y: f32, color: Color) -> ShapeVertex {
        ShapeVertex {
            position: Vector2 { x: x, y: y },
            rotation: 0f32,
            color: color,
        }
    }
}

impl Vertex for ShapeVertex {
    const VERTEX_SIZE: usize = mem::size_of::<Self>();

    fn configure_vertex_attribute() {
        unsafe {
            // Position 2D
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,                        // Index
                2,                        // Count
                gl::FLOAT,                // Type
                gl::FALSE,                // Normalized
                Self::VERTEX_SIZE as i32, // Stride
                (0) as *const _,          // Offset
            );
            // Rotation
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,                        // Index
                1,                        // Count
                gl::FLOAT,                // Type
                gl::FALSE,                // Normalized
                Self::VERTEX_SIZE as i32, // Stride
                (2 * 4) as *const _,      // Offset
            );
            // Color
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,                               // Index
                gl::BGRA as i32,                 // Count
                gl::UNSIGNED_INT_2_10_10_10_REV, // Type
                gl::TRUE,                        // Normalized
                Self::VERTEX_SIZE as i32,        // Stride
                (3 * 4) as *const _,             // Offset
            );
        }
    }
}
