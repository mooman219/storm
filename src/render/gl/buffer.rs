use crate::render::gl::raw::{resource, BufferBindingTarget, BufferUsage, DrawMode, OpenGL};
use crate::render::gl::vertex::*;

pub struct Buffer<T: VertexDescription + Copy> {
    gl: OpenGL,
    vbo: resource::Buffer,
    vertices: usize,
    buffer_type: BufferBindingTarget,
    vertex_array: VertexArray<T>,
}

impl<T: VertexDescription + Copy> Buffer<T> {
    pub fn new(gl: OpenGL, buffer_type: BufferBindingTarget) -> Buffer<T> {
        let vbo = gl.create_buffer();
        gl.bind_buffer(buffer_type, Some(vbo));
        gl.buffer_data_empty(buffer_type, 0, BufferUsage::StaticDraw);
        let vertex_array = VertexArray::new(gl.clone());
        Buffer {
            gl,
            vbo,
            vertices: 0,
            buffer_type,
            vertex_array,
        }
    }

    pub fn len(&self) -> usize {
        self.vertices
    }

    pub fn set(&mut self, items: &Vec<T>) {
        self.vertices = items.len();
        if self.vertices > 0 {
            self.gl.bind_buffer(self.buffer_type, Some(self.vbo));
            self.gl.buffer_data(self.buffer_type, items, BufferUsage::StaticDraw);
        }
    }

    pub fn draw(&self) {
        self.vertex_array.bind();
        self.gl.draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, self.vertices as i32);
    }
}
