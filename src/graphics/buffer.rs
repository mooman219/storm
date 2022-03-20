use crate::graphics::{
    configure_vertex, graphics, resource, BufferBindingTarget, BufferUsage, VertexDescriptor,
};
use crate::{App, Context};
use core::marker::PhantomData;

/// Buffers a set of elements on the device.
pub struct Buffer<T: VertexDescriptor + Copy> {
    // This type is !Send + !Sync.
    _unsend: core::marker::PhantomData<*const ()>,
    vbo: resource::Buffer,
    vao: resource::VertexArray,
    vertices: usize,
    phantom: PhantomData<T>,
}

impl<T: VertexDescriptor + Copy> Buffer<T> {
    /// Creates a new buffer.
    pub fn new(_ctx: &Context<impl App>) -> Buffer<T> {
        let gl = graphics().gl();

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(Some(vao));
        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::ArrayBuffer, Some(vbo));
        configure_vertex::<T>(&T::ATTRIBUTES, gl);
        gl.bind_vertex_array(None);

        Buffer {
            _unsend: core::marker::PhantomData,
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
            let gl = graphics().gl();
            gl.bind_buffer(BufferBindingTarget::ArrayBuffer, Some(self.vbo));
            gl.buffer_data(BufferBindingTarget::ArrayBuffer, items, BufferUsage::StaticDraw);
        }
    }

    pub(crate) fn bind(&self) {
        let gl = graphics().gl();
        gl.bind_vertex_array(Some(self.vao));
    }
}

impl<T: VertexDescriptor + Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_buffer(self.vbo);
        gl.delete_vertex_array(self.vao);
    }
}

impl<T: VertexDescriptor + Copy> AsRef<Buffer<T>> for Buffer<T> {
    fn as_ref(&self) -> &Buffer<T> {
        self
    }
}
