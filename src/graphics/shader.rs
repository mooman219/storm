use crate::graphics::{graphics, resource, Texture, Uniform, VertexDescriptor};
use crate::{App, Context};
use alloc::{format, vec::Vec};
use core::marker::PhantomData;

/// A trait to describe a shader's inputs and outputs so they can be represented without using the
/// heap.
pub trait ShaderDescriptor {
    const VERTEX_SHADER: &'static str;
    const FRAGMENT_SHADER: &'static str;
    const TEXTURE_NAMES: &'static [&'static str];
    const UNIFORM_NAMES: &'static [&'static str];
    type VertexDescriptor: VertexDescriptor;
}

/// Represents the runtime metadata required to configure and draw with a shader.
pub struct Shader<T: ShaderDescriptor> {
    // This type is !Send + !Sync.
    _unsend: core::marker::PhantomData<*const ()>,
    program: resource::Program,
    texture_locations: Vec<resource::UniformLocation>,
    phantom: PhantomData<T>,
}

impl<T: ShaderDescriptor> Shader<T> {
    /// Creates a new shader. Shaders hold no mutable state and should be reused as often as
    /// possible.
    pub fn new(_ctx: &Context<impl App>) -> Shader<T> {
        let gl = graphics().gl();

        let program = gl.shader_program(T::VERTEX_SHADER, T::FRAGMENT_SHADER);

        for (i, name) in T::UNIFORM_NAMES.iter().enumerate() {
            let idx = gl
                .get_uniform_block_index(program, name)
                .expect(&format!("Failed to find uniform block named '{}' in your shader.", name));
            gl.uniform_block_binding(program, idx, i as u32);
        }

        let texture_locations = T::TEXTURE_NAMES
            .iter()
            .map(|name| {
                gl.get_uniform_location(program, name)
                    .expect(&format!("Failed to find texture named '{}' in your shader.", name))
            })
            .collect();

        Shader {
            _unsend: core::marker::PhantomData,
            program,
            texture_locations,
            phantom: PhantomData,
        }
    }

    /// Binds this shader for future draw calls.
    /// # Arguments
    ///
    /// * `uniform` - The uniform to use for the shader invocation.
    /// * `textures` - The set of textures to use in the fragment shader.
    pub fn bind(&self, uniforms: &[&Uniform], textures: &[&Texture]) {
        if textures.len() != self.texture_locations.len() {
            panic!(
                "Textures length ({}) must equal ShaderDescriptor::TEXTURE_NAMES length ({})",
                textures.len(),
                self.texture_locations.len()
            );
        }
        if uniforms.len() != T::UNIFORM_NAMES.len() {
            panic!(
                "Uniforms length ({}) must equal ShaderDescriptor::UNIFORM_NAMES length ({})",
                textures.len(),
                self.texture_locations.len()
            );
        }

        let gl = graphics().gl();
        gl.use_program(Some(self.program));

        for i in 0..T::UNIFORM_NAMES.len() {
            uniforms[i].bind(i as u32);
        }

        for i in 0..self.texture_locations.len() {
            textures[i].bind(i as u32);
            gl.uniform_1_i32(Some(&self.texture_locations[i]), i as i32);
        }
    }
}

impl<T: ShaderDescriptor> Drop for Shader<T> {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_program(self.program);
    }
}
