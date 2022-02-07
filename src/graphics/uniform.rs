use crate::graphics::{
    graphics, resource, std140::Std140Struct, BufferBindingTarget, BufferBlockBindingTarget, BufferUsage,
};
use crate::{App, Context};
use core::marker::PhantomData;

/// Stores a uniform on the device.
pub struct Uniform<T: Std140Struct> {
    _unsend: core::marker::PhantomData<*const ()>,
    vbo: resource::Buffer,
    phantom: PhantomData<T>,
}

impl<T: Std140Struct> Uniform<T> {
    /// Creates a new uniform.
    pub fn new<Z: Into<T>>(_ctx: &Context<impl App>, uniform: Z) -> Uniform<T> {
        let gl = graphics().gl();

        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(vbo));
        gl.buffer_data(BufferBindingTarget::UniformBuffer, &[uniform.into()], BufferUsage::StaticDraw);

        Uniform {
            _unsend: core::marker::PhantomData,
            vbo,
            phantom: PhantomData,
        }
    }

    /// Sets the value of the uniform.
    pub fn set<Z: Into<T>>(&mut self, uniform: Z) {
        let gl = graphics().gl();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(self.vbo));
        gl.buffer_data(BufferBindingTarget::UniformBuffer, &[uniform.into()], BufferUsage::StaticDraw);
    }

    pub(crate) fn bind(&self, block: u32) {
        let gl = graphics().gl();
        gl.bind_buffer_base(BufferBlockBindingTarget::UniformBuffer, block, Some(self.vbo));
    }
}

impl<T: Std140Struct> Drop for Uniform<T> {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_buffer(self.vbo);
    }
}
