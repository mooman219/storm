use crate::ctx;
use crate::graphics::{resource, AsStd140, Buffer, DrawMode, Texture, Uniform, VertexDescriptor};
use alloc::format;
use core::marker::PhantomData;

/// A trait to describe a shader's inputs and outputs so they can be represented without using the
/// heap.
pub trait ShaderDescriptor<const TEXTURES: usize> {
    const VERTEX_SHADER: &'static str;
    const FRAGMENT_SHADER: &'static str;
    const TEXTURE_NAMES: [&'static str; TEXTURES];
    const VERTEX_UNIFORM_NAME: &'static str;
    type VertexUniformType: AsStd140;
    type VertexDescriptor: VertexDescriptor + Copy;
}

/// Represents the runtime metadata required to configure and draw with a shader.
pub struct Shader<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> {
    program: resource::Program,
    vertex_uniform_location: u32,
    texture_locations: [resource::UniformLocation; TEXTURES],
    phantom: PhantomData<T>,
}

impl<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> Shader<T, TEXTURES> {
    /// Creates a new shader. Shaders hold no mutable state and should be reused as often as
    /// possible.
    pub fn new() -> Shader<T, TEXTURES> {
        let gl = ctx().graphics().gl();

        let program = gl.shader_program(T::VERTEX_SHADER, T::FRAGMENT_SHADER);
        let vertex_uniform_location = gl.get_uniform_block_index(program, T::VERTEX_UNIFORM_NAME).expect(
            &format!("Failed to find uniform block named '{}' in vertex shader.", T::VERTEX_UNIFORM_NAME),
        );
        gl.uniform_block_binding(program, vertex_uniform_location, 0);

        let texture_locations = T::TEXTURE_NAMES.map(|name| {
            gl.get_uniform_location(program, name)
                .expect(&format!("Failed to find texture named '{}' in fragment shader.", name))
        });

        Shader {
            program,
            vertex_uniform_location,
            texture_locations,
            phantom: PhantomData,
        }
    }

    fn bind(&self, uniform: &Uniform<T::VertexUniformType>, textures: [&Texture; TEXTURES]) {
        let gl = ctx().graphics().gl();
        gl.use_program(Some(self.program));
        uniform.bind(0);
        for i in 0..TEXTURES {
            textures[i].bind(i as u32);
            gl.uniform_1_i32(Some(&self.texture_locations[i]), i as i32);
        }
    }

    /// Performs an instanced draw to the screen.
    /// # Arguments
    ///
    /// * `mode` - Specifies what kind of primitives to render.
    /// * `uniform` - The uniform to use for the shader invocation.
    /// * `textures` - The set of textures to use in the fragment shader.
    /// * `buffers` - The set of buffers of vertices to draw, reusing the uniform and textures for
    /// each buffer draw.
    /// * `count` - Specifies the number of instances to be rendered.
    pub fn draw_instanced(
        &self,
        mode: DrawMode,
        uniform: &Uniform<T::VertexUniformType>,
        textures: [&Texture; TEXTURES],
        buffers: &[&Buffer<T::VertexDescriptor>],
        count: i32,
    ) {
        self.bind(uniform, textures);
        for buffer in buffers {
            if buffer.len() > 0 {
                buffer.bind();
                ctx().graphics().gl().draw_arrays_instanced(mode, 0, count, buffer.len() as i32);
            }
        }
    }

    /// Performs a draw to the screen.
    /// # Arguments
    ///
    /// * `mode` - Specifies what kind of primitives to render.
    /// * `uniform` - The uniform to use for the shader invocation.
    /// * `textures` - The set of textures to use in the fragment shader.
    /// * `buffers` - The set of buffers of vertices to draw, reusing the uniform and textures for
    /// each buffer draw.
    pub fn draw(
        &self,
        mode: DrawMode,
        uniform: &Uniform<T::VertexUniformType>,
        textures: [&Texture; TEXTURES],
        buffers: &[&Buffer<T::VertexDescriptor>],
    ) {
        self.bind(uniform, textures);
        for buffer in buffers {
            if buffer.len() > 0 {
                buffer.bind();
                ctx().graphics().gl().draw_arrays(mode, 0, buffer.len() as i32);
            }
        }
    }
}

impl<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> Drop for Shader<T, TEXTURES> {
    fn drop(&mut self) {
        let gl = ctx().graphics().gl();
        gl.delete_program(self.program);
    }
}
