use cgmath::*;
use gl;
use std::mem;

use render::vertex::*;
use render::color::*;

#[repr(C)]
pub struct Pos2Vertex {
    position: Vector2<f32>,
    color: Color,
}

impl Pos2Vertex {
    const VERTEX_SIZE: usize = mem::size_of::<Pos2Vertex>();

    pub fn new(x: f32, y: f32, red: f32, green: f32, blue: f32, alpha: f32) -> Pos2Vertex {
        Pos2Vertex {
            position: Vector2 { x: x, y: y },
            color: Color::new(red, green, blue, alpha),
        }
    }
}

impl Vertex for Pos2Vertex {
    fn configure_vertex_attribute() {
        unsafe {
            // Position 2D
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,                                   // Index
                2,                                   // Count
                gl::FLOAT,                           // Type
                gl::FALSE,                           // Normalized
                Pos2Vertex::VERTEX_SIZE as i32, // Stride
                (0) as *const _,                     // Offset
            );
            // Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,                                   // Index
                gl::BGRA as i32,                     // Count
                gl::UNSIGNED_INT_2_10_10_10_REV,     // Type
                gl::TRUE,                            // Normalized
                Pos2Vertex::VERTEX_SIZE as i32, // Stride
                (2 * 4) as *const _,                 // Offset
            );
        }
    }
}
