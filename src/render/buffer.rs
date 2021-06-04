use crate::render::raw::{resource, BufferBindingTarget, BufferUsage, DrawMode, OpenGL};
use crate::render::vertex::*;
use core::marker::PhantomData;

pub struct Buffer<T: VertexDescription + Copy> {
    gl: OpenGL,
    vbo: resource::Buffer,
    vao: resource::VertexArray,
    vertices: usize,
    buffer_type: BufferBindingTarget,
    phantom: PhantomData<T>,
}

impl<T: VertexDescription + Copy> Buffer<T> {
    pub fn new(gl: OpenGL, buffer_type: BufferBindingTarget) -> Buffer<T> {
        let vbo = gl.create_buffer();
        gl.bind_buffer(buffer_type, Some(vbo));
        gl.buffer_data_empty(buffer_type, 0, BufferUsage::StaticDraw);

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(Some(vao));
        T::configure_vertex_attribute(&gl);

        Buffer {
            gl,
            vbo,
            vao,
            vertices: 0,
            buffer_type,
            phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.vertices
    }

    pub fn clear(&mut self) {
        self.vertices = 0;
    }

    pub fn set(&mut self, items: &Vec<T>) {
        self.vertices = items.len();
        if self.vertices > 0 {
            self.gl.bind_buffer(self.buffer_type, Some(self.vbo));
            self.gl.buffer_data(self.buffer_type, items, BufferUsage::StaticDraw);
        }
    }

    pub fn draw(&self) {
        if self.vertices > 0 {
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, self.vertices as i32);
        }
    }
}

impl<T: VertexDescription + Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        self.gl.delete_buffer(self.vbo);
        self.gl.delete_vertex_array(self.vao);
    }
}
