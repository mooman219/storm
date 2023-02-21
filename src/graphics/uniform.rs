use crate::graphics::{
    graphics, resource, std140::IntoStd140, BufferBindingTarget, BufferBlockBindingTarget, BufferUsage,
};
use crate::{App, Context};

/// Stores a uniform on the device.
pub struct Uniform {
    _unsend: core::marker::PhantomData<*const ()>,
    vbo: resource::Buffer,
}

impl Uniform {
    /// Creates a new uniform.
    pub fn new<T: IntoStd140>(_ctx: &Context<impl App>, uniform: T) -> Uniform {
        let gl = graphics().gl();

        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(vbo));
        gl.buffer_data(BufferBindingTarget::UniformBuffer, &[uniform.std140()], BufferUsage::StaticDraw);

        Uniform {
            _unsend: core::marker::PhantomData,
            vbo,
        }
    }

    /// Sets the value of the uniform.
    pub fn set<T: IntoStd140>(&mut self, uniform: T) {
        let gl = graphics().gl();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(self.vbo));
        gl.buffer_data(BufferBindingTarget::UniformBuffer, &[uniform.std140()], BufferUsage::StaticDraw);
    }

    pub(crate) fn bind(&self, block: u32) {
        let gl = graphics().gl();
        gl.bind_buffer_base(BufferBlockBindingTarget::UniformBuffer, block, Some(self.vbo));
    }
}

impl Drop for Uniform {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_buffer(self.vbo);
    }
}
