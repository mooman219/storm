use cgmath::*;
use gl;
use render::vertex::*;

#[repr(C)]
pub struct TextureVertex {
    position: Vector3<f32>,
    uv: Vector2<f32>,
}

impl TextureVertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> TextureVertex {
        TextureVertex {
            position: Vector3::new(x, y, z),
            uv: Vector2::new(u, v),
        }
    }
}

impl Vertex for TextureVertex {
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
            // UV
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1, // Index
                2, // Count
                gl::FLOAT, // Type
                gl::FALSE, // Normalized
                Self::VERTEX_SIZE as i32, // Stride
                (3 * 4) as *const _, // Offset
            );
        }
    }
}
