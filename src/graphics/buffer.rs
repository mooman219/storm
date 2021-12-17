use crate::graphics::{configure_vertex, VertexDescriptor};
use crate::render::{resource, BufferBindingTarget, BufferUsage, OpenGLState};
use core::marker::PhantomData;

/// Buffers a set of elements on the device.
pub struct Buffer<T: VertexDescriptor + Copy> {
    vbo: resource::Buffer,
    vao: resource::VertexArray,
    vertices: usize,
    phantom: PhantomData<T>,
}

impl<T: VertexDescriptor + Copy> Buffer<T> {
    /// Creates a new buffer.
    pub fn new() -> Buffer<T> {
        let gl = &mut OpenGLState::ctx().gl;

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(Some(vao));
        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::ArrayBuffer, Some(vbo));
        configure_vertex::<T>(&T::ATTRIBUTES, gl);
        gl.bind_vertex_array(None);

        Buffer {
            vbo,
            vao,
            vertices: 0,
            phantom: PhantomData,
        }
    }

    /// Gets the number of elements in the buffer.
    pub fn len(&self) -> usize {
        self.vertices
    }

    /// Clears all elements from the buffer.
    pub fn clear(&mut self) {
        self.vertices = 0;
    }

    /// Sets the elements in the buffer.
    pub fn set(&mut self, items: &[T]) {
        self.vertices = items.len();
        if self.vertices > 0 {
            let gl = &OpenGLState::ctx().gl;
            gl.bind_buffer(BufferBindingTarget::ArrayBuffer, Some(self.vbo));
            gl.buffer_data(BufferBindingTarget::ArrayBuffer, items, BufferUsage::StaticDraw);
        }
    }

    pub(crate) fn bind(&self) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.bind_vertex_array(Some(self.vao));
    }
}

impl<T: VertexDescriptor + Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.delete_buffer(self.vbo);
        gl.delete_vertex_array(self.vao);
    }
}
