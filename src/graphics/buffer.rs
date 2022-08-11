use crate::graphics::{
    configure_vertex, graphics, resource, BufferBindingTarget, BufferUsage, VertexDescriptor,
};
use crate::{App, Context};
use core::marker::PhantomData;

/// Buffers a set of elements on the device.
pub struct Buffer<T: VertexDescriptor> {
    // This type is !Send + !Sync.
    _unsend: core::marker::PhantomData<*const ()>,
    vbo: resource::Buffer,
    vao: resource::VertexArray,
    vertices: usize,
    phantom: PhantomData<T>,
}

impl<T: VertexDescriptor> Buffer<T> {
    /// Creates a new array buffer.
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

    /// Sets the data in the buffer.
    pub fn set_data(&mut self, items: &[T]) {
        self.vertices = items.len();
        let gl = graphics().gl();
        gl.bind_buffer(BufferBindingTarget::ArrayBuffer, Some(self.vbo));
        gl.buffer_data(BufferBindingTarget::ArrayBuffer, items, BufferUsage::StaticDraw);
    }

    /// Perfroms a draw call for the buffer. The most recently bound shader will dictate how this
    /// data is processed.
    pub fn draw(&self) {
        let gl = graphics().gl();
        gl.bind_vertex_array(Some(self.vao));
        if T::INSTANCING.is_instanced() {
            gl.draw_arrays_instanced(T::DRAW_MODE, 0, T::INSTANCING.count, self.vertices as i32);
        } else {
            gl.draw_arrays(T::DRAW_MODE, 0, self.vertices as i32);
        }
    }
}

impl<T: VertexDescriptor> Drop for Buffer<T> {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_buffer(self.vbo);
        gl.delete_vertex_array(self.vao);
    }
}

impl<T: VertexDescriptor> AsRef<Buffer<T>> for Buffer<T> {
    fn as_ref(&self) -> &Buffer<T> {
        self
    }
}
