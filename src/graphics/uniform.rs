use crate::ctx;
use crate::graphics::{resource, AsStd140, BufferBindingTarget, BufferBlockBindingTarget, BufferUsage};
use core::marker::PhantomData;
use crevice::std140::Std140;

/// Stores a uniform on the device.
pub struct Uniform<T: AsStd140> {
    vbo: resource::Buffer,
    phantom: PhantomData<T>,
}

impl<T: AsStd140> Uniform<T> {
    /// Creates a new uniform.
    pub fn new<Z: Into<T>>(uniform: Z) -> Uniform<T> {
        let gl = ctx().graphics().gl();

        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(vbo));
        gl.buffer_data_u8_slice(
            BufferBindingTarget::UniformBuffer,
            uniform.into().as_std140().as_bytes(),
            BufferUsage::StaticDraw,
        );

        Uniform {
            vbo,
            phantom: PhantomData,
        }
    }

    pub(crate) fn new_internal() -> Uniform<T> {
        let gl = ctx().graphics().gl();

        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(vbo));
        gl.buffer_data_empty(
            BufferBindingTarget::UniformBuffer,
            T::std140_size_static() as i32,
            BufferUsage::StaticDraw,
        );

        Uniform {
            vbo,
            phantom: PhantomData,
        }
    }

    /// Sets the value of the uniform.
    pub fn set<Z: Into<T>>(&mut self, uniform: Z) {
        let gl = ctx().graphics().gl();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(self.vbo));
        gl.buffer_data_u8_slice(
            BufferBindingTarget::UniformBuffer,
            uniform.into().as_std140().as_bytes(),
            BufferUsage::StaticDraw,
        );
    }

    pub(crate) fn bind(&self, block: u32) {
        let gl = ctx().graphics().gl();
        gl.bind_buffer_base(BufferBlockBindingTarget::UniformBuffer, block, Some(self.vbo));
    }
}

impl<T: AsStd140> Drop for Uniform<T> {
    fn drop(&mut self) {
        let gl = ctx().graphics().gl();
        gl.delete_buffer(self.vbo);
    }
}
