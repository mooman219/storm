use crate::graphics::{
    configure_vertex, graphics, resource, BufferBindingTarget, BufferUsage, VertexDescriptor,
};
use crate::{math::UnsignedInteger, App, Context};
use core::marker::PhantomData;

/// Buffers a set of elements on the device.
pub struct IndexBuffer<T: VertexDescriptor, U: UnsignedInteger> {
    // This type is !Send + !Sync.
    _unsend: core::marker::PhantomData<*const ()>,
    vbo: resource::Buffer,
    ebo: resource::Buffer,
    vao: resource::VertexArray,
    vertices: usize,
    indices: usize,
    phantom: PhantomData<(T, U)>,
}

impl<T: VertexDescriptor, U: UnsignedInteger> IndexBuffer<T, U> {
    /// Creates a new array buffer.
    pub fn new(_ctx: &Context<impl App>) -> IndexBuffer<T, U> {
        let gl = graphics().gl();

        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(Some(vao));

        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::ArrayBuffer, Some(vbo));
        configure_vertex::<T>(&T::ATTRIBUTES, gl);

        let ebo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::ElementArrayBuffer, Some(ebo));

        gl.bind_vertex_array(None);

        IndexBuffer {
            _unsend: core::marker::PhantomData,
            vbo,
            ebo,
            vao,
            vertices: 0,
            indices: 0,
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

    /// Attaches indices to the buffer.
    pub fn set_indices(&mut self, indices: &[U]) {
        self.indices = indices.len();
        let gl = graphics().gl();
        gl.bind_vertex_array(Some(self.vao));
        gl.buffer_data(BufferBindingTarget::ElementArrayBuffer, indices, BufferUsage::StaticDraw);
    }

    /// Perfroms a draw call for the buffer. The most recently bound shader will dictate how this
    /// data is processed.
    pub fn draw(&self) {
        let gl = graphics().gl();
        gl.bind_vertex_array(Some(self.vao));
        if T::INSTANCING.is_instanced() {
            gl.draw_elements_instanced(T::DRAW_MODE, U::INDICE_TYPE, self.indices as i32, T::INSTANCING.count)
        } else {
            gl.draw_elements(T::DRAW_MODE, U::INDICE_TYPE, self.indices as i32)
        }
    }
}

impl<T: VertexDescriptor, U: UnsignedInteger> Drop for IndexBuffer<T, U> {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_buffer(self.vbo);
        gl.delete_buffer(self.ebo);
        gl.delete_vertex_array(self.vao);
    }
}

impl<T: VertexDescriptor, U: UnsignedInteger> AsRef<IndexBuffer<T, U>> for IndexBuffer<T, U> {
    fn as_ref(&self) -> &IndexBuffer<T, U> {
        self
    }
}
