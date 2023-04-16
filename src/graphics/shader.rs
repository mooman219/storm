use crate::graphics::{graphics, resource, Texture, Uniform};
use crate::{App, Context};
use alloc::{format, vec::Vec};

/// A struct to describe a shader's inputs and outputs so they can be represented without using the
/// heap.
pub struct ShaderDescription<'a> {
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
    pub texture_names: &'a [&'a str],
    pub uniform_names: &'a [&'a str],
}

/// Represents the runtime metadata required to configure and draw with a shader.
pub struct Shader {
    // This type is !Send + !Sync.
    _unsend: core::marker::PhantomData<*const ()>,
    program: resource::Program,
    uniform_len: usize,
    texture_locations: Vec<resource::UniformLocation>,
}

impl Shader {
    /// Creates a new shader. Shaders hold no mutable state and should be reused as often as
    /// possible.
    pub fn new(_ctx: &Context<impl App>, description: ShaderDescription) -> Shader {
        let gl = graphics().gl();

        let program = gl.shader_program(description.vertex_shader, description.fragment_shader);

        for (i, name) in description.uniform_names.iter().enumerate() {
            let idx = gl
                .get_uniform_block_index(program, name)
                .expect(&format!("Failed to find uniform block named '{}' in your shader.", name));
            gl.uniform_block_binding(program, idx, i as u32);
        }

        let texture_locations = description
            .texture_names
            .iter()
            .map(|name| {
                gl.get_uniform_location(program, name)
                    .expect(&format!("Failed to find texture named '{}' in your shader.", name))
            })
            .collect();

        Shader {
            _unsend: core::marker::PhantomData,
            program,
            uniform_len: description.uniform_names.len(),
            texture_locations,
        }
    }

    /// Binds this shader for future draw calls. The order of uniforms and textures are as they're
    /// defined in the `ShaderDescription` used to create this `Shader`.
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
        if uniforms.len() != self.uniform_len {
            panic!(
                "Uniforms length ({}) must equal ShaderDescriptor::UNIFORM_NAMES length ({})",
                textures.len(),
                self.texture_locations.len()
            );
        }

        let gl = graphics().gl();
        gl.use_program(Some(self.program));

        for i in 0..self.uniform_len {
            uniforms[i].bind(i as u32);
        }

        for i in 0..self.texture_locations.len() {
            textures[i].bind(i as u32);
            gl.uniform_1_i32(Some(&self.texture_locations[i]), i as i32);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        let gl = graphics().gl();
        gl.delete_program(self.program);
    }
}
