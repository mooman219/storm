use crate::graphics::{
    configure_vertex, graphics, resource, BufferBindingTarget, BufferUsage, DrawMode, IndiceType,
    VertexDescriptor,
};
use crate::{math::UnsignedInteger, App, Context};
use core::marker::PhantomData;

/// Buffers a set of elements on the device.
pub struct Buffer<T: VertexDescriptor + Copy> {
    // This type is !Send + !Sync.
    _unsend: core::marker::PhantomData<*const ()>,
    vbo: resource::Buffer,
    ebo: Option<resource::Buffer>,
    vao: resource::VertexArray,
    vertices: usize,
    indices: usize,
    indice_type: IndiceType,
    phantom: PhantomData<T>,
}

impl<T: VertexDescriptor + Copy> Buffer<T> {
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
            ebo: None,
            vao,
            vertices: 0,
            indices: 0,
            indice_type: IndiceType::UnsignedShort,
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
    pub fn set_indices<U: UnsignedInteger>(&mut self, indices: &[U]) {
        self.indices = indices.len();
        let gl = graphics().gl();
        gl.bind_vertex_array(Some(self.vao));
        if self.ebo.is_none() {
            let ebo = gl.create_buffer();
            gl.bind_buffer(BufferBindingTarget::ElementArrayBuffer, Some(ebo));
            self.ebo = Some(ebo);
        };
        gl.buffer_data(BufferBindingTarget::ElementArrayBuffer, indices, BufferUsage::StaticDraw);
        self.indice_type = U::INDICE_TYPE;
    }

    pub(crate) fn draw(&self, mode: DrawMode) {
        let gl = graphics().gl();
        gl.bind_vertex_array(Some(self.vao));
        if T::INSTANCING.is_instanced() {
            if self.ebo.is_some() {
                gl.draw_elements_instanced(mode, self.indice_type, self.indices as i32, T::INSTANCING.count)
            } else {
                gl.draw_arrays_instanced(mode, 0, T::INSTANCING.count, self.vertices as i32);
            }
        } else {
            if self.ebo.is_some() {
                gl.draw_elements(mode, self.indice_type, self.indices as i32)
            } else {
                gl.draw_arrays(mode, 0, self.vertices as i32);
            }
        }
    }
}

impl<T: VertexDescriptor + Copy> Drop for Buffer<T> {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_buffer(self.vbo);
        if let Some(ebo) = self.ebo {
            gl.delete_buffer(ebo);
        }
        gl.delete_vertex_array(self.vao);
    }
}

impl<T: VertexDescriptor + Copy> AsRef<Buffer<T>> for Buffer<T> {
    fn as_ref(&self) -> &Buffer<T> {
        self
    }
}
