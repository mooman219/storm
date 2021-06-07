use crate::render::raw::{resource, BufferBindingTarget, BufferUsage, DrawMode};
use crate::render::vertex::*;
use crate::render::OpenGLState;
use crate::utility::bad::UnsafeShared;
use core::marker::PhantomData;

pub struct Buffer<T: VertexDescription + Copy> {
    state: UnsafeShared<OpenGLState>,
    vbo: resource::Buffer,
    vao: resource::VertexArray,
    vertices: usize,
    buffer_type: BufferBindingTarget,
    phantom: PhantomData<T>,
}

impl<T: VertexDescription + Copy> Buffer<T> {
    pub fn new(state: UnsafeShared<OpenGLState>, buffer_type: BufferBindingTarget) -> Buffer<T> {
        let vbo = state.gl.create_buffer();
        state.gl.bind_buffer(buffer_type, Some(vbo));
        state.gl.buffer_data_empty(buffer_type, 0, BufferUsage::StaticDraw);

        let vao = state.gl.create_vertex_array();
        state.gl.bind_vertex_array(Some(vao));
        T::configure_vertex_attribute(&state.gl);

        Buffer {
            state,
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
            self.state.gl.bind_buffer(self.buffer_type, Some(self.vbo));
            self.state.gl.buffer_data(self.buffer_type, items, BufferUsage::StaticDraw);
        }
    }

    pub fn draw(&self) {
        if self.vertices > 0 {
            self.state.gl.bind_vertex_array(Some(self.vao));
            self.state.gl.draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, self.vertices as i32);
        }
    }
}

impl<T: VertexDescription + Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        self.state.gl.delete_buffer(self.vbo);
        self.state.gl.delete_vertex_array(self.vao);
    }
}
