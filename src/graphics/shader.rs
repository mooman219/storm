use crate::graphics::{AsStd140, Buffer, Texture, Uniform, VertexDescriptor};
use crate::render::raw::{resource, DrawMode};
use crate::render::OpenGLState;
use core::marker::PhantomData;

/// A trait to describe a shader's inputs and outputs.
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
    /// Creates a new shader.
    pub fn new() -> Shader<T, TEXTURES> {
        let gl = &mut OpenGLState::ctx().gl;

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

    /// Draws the instance to the screen.
    pub fn draw_instanced(
        &self,
        uniform: &Uniform<T::VertexUniformType>,
        textures: [&Texture; TEXTURES],
        buffer: &Buffer<T::VertexDescriptor>,
        count: i32,
    ) {
        if buffer.len() > 0 {
            let gl = &mut OpenGLState::ctx().gl;
            gl.use_program(Some(self.program));
            uniform.bind(0);
            for i in 0..TEXTURES {
                textures[i].bind(i as u32);
                gl.uniform_1_i32(Some(&self.texture_locations[i]), i as i32);
            }
            buffer.bind();
            gl.draw_arrays_instanced(DrawMode::TriangleStrip, 0, count, buffer.len() as i32);
        }
    }
}

impl<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> Drop for Shader<T, TEXTURES> {
    fn drop(&mut self) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.delete_program(self.program);
    }
}
