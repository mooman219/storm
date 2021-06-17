use crate::render::raw::{resource, BufferBindingTarget, BufferUsage, DrawMode};
use crate::render::vertex::VertexDescription;
use crate::render::OpenGLState;
use core::marker::PhantomData;

pub struct Buffer<T: VertexDescription + Copy> {
    vbo: resource::Buffer,
    vao: resource::VertexArray,
    vertices: usize,
    buffer_type: BufferBindingTarget,
    phantom: PhantomData<T>,
}

impl<T: VertexDescription + Copy> Buffer<T> {
    pub fn new(buffer_type: BufferBindingTarget) -> Buffer<T> {
        let gl = &mut OpenGLState::ctx().gl;

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(Some(vao));
        let vbo = gl.create_buffer();
        gl.bind_buffer(buffer_type, Some(vbo));
        T::configure_vertex_attribute(gl);
        gl.bind_vertex_array(None);

        Buffer {
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
            let gl = &OpenGLState::ctx().gl;
            gl.bind_buffer(self.buffer_type, Some(self.vbo));
            gl.buffer_data(self.buffer_type, items, BufferUsage::StaticDraw);
        }
    }

    pub fn draw(&self) {
        if self.vertices > 0 {
            let gl = &mut OpenGLState::ctx().gl;
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, self.vertices as i32);
        }
    }
}

impl<T: VertexDescription + Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        let gl = &OpenGLState::ctx().gl;
        gl.delete_buffer(self.vbo);
        gl.delete_vertex_array(self.vao);
    }
}
