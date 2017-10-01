use cgmath::*;
use gl;
use std::mem;

use render::vertex::*;
use render::color::*;

#[repr(C)]
pub struct Pos3ColorVertex {
    position: Vector3<f32>,
    color: Color,
}

impl Pos3ColorVertex {
    const VERTEX_SIZE: usize = mem::size_of::<Pos3ColorVertex>();

    pub fn new(x: f32, y: f32, z: f32, red: f32, green: f32, blue: f32, alpha: f32) -> Pos3ColorVertex {
        Pos3ColorVertex {
            position: Vector3 { x: x, y: y, z: z },
            color: Color::new(red, green, blue, alpha),
        }
    }
}

impl Vertex for Pos3ColorVertex {
    fn configure_vertex_attribute() {
        unsafe {
            // Position 3D
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,                                   // Index
                3,                                   // Count
                gl::FLOAT,                           // Type
                gl::FALSE,                           // Normalized
                Pos3ColorVertex::VERTEX_SIZE as i32, // Stride
                (0) as *const _,                     // Offset
            );
            // Color
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,                                   // Index
                gl::BGRA as i32,                     // Count
                gl::UNSIGNED_INT_2_10_10_10_REV,     // Type
                gl::TRUE,                            // Normalized
                Pos3ColorVertex::VERTEX_SIZE as i32, // Stride
                (3 * 4) as *const _,                 // Offset
            );
        }
    }
}
