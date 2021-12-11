use crate::graphics::AsStd140;
use crate::render::raw::{resource, BufferBindingTarget, BufferBlockBindingTarget, BufferUsage};
use crate::render::OpenGLState;
use core::marker::PhantomData;
use crevice::std140::Std140;

pub(crate) struct UniformBuffer<T: AsStd140> {
    vbo: resource::Buffer,
    phantom: PhantomData<T>,
}

impl<T: AsStd140> UniformBuffer<T> {
    pub fn new(uniform: T) -> UniformBuffer<T> {
        let gl = &mut OpenGLState::ctx().gl;

        let vbo = gl.create_buffer();
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(vbo));
        gl.buffer_data_u8_slice(
            BufferBindingTarget::UniformBuffer,
            uniform.as_std140().as_bytes(),
            BufferUsage::StaticDraw,
        );

        UniformBuffer {
            vbo,
            phantom: PhantomData,
        }
    }

    pub fn set(&mut self, uniform: T) {
        let gl = &OpenGLState::ctx().gl;
        gl.bind_buffer(BufferBindingTarget::UniformBuffer, Some(self.vbo));
        gl.buffer_data_u8_slice(
            BufferBindingTarget::UniformBuffer,
            uniform.as_std140().as_bytes(),
            BufferUsage::StaticDraw,
        );
    }

    pub fn bind(&self, block: u32) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.bind_buffer_base(BufferBlockBindingTarget::UniformBuffer, block, Some(self.vbo));
    }
}

impl<T: AsStd140> Drop for UniformBuffer<T> {
    fn drop(&mut self) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.delete_buffer(self.vbo);
    }
}
