use crate::graphics::{AsStd140, Buffer, Texture, UniformBuffer, VertexDescriptor};
use crate::render::raw::{resource, BufferBindingTarget, DrawMode};
use crate::render::OpenGLState;
use crate::Context;
use alloc::rc::Rc;
use core::marker::PhantomData;

pub trait ShaderDescriptor<const TEXTURES: usize> {
    const VERTEX_SHADER: &'static str;
    const FRAGMENT_SHADER: &'static str;
    const TEXTURE_NAMES: [&'static str; TEXTURES];
    const VERTEX_UNIFORM_NAME: &'static str;
    const VERTEX_UNIFORM_DEFAULT: Self::VertexUniformType;
    type VertexUniformType: AsStd140;
    type VertexDescriptor: VertexDescriptor + Copy;
}

pub struct Shader<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> {
    rc: Rc<()>,
    program: resource::Program,
    vertex_uniform_location: u32,
    texture_locations: [resource::UniformLocation; TEXTURES],
    phantom: PhantomData<T>,
}

impl<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> Clone for Shader<T, TEXTURES> {
    fn clone(&self) -> Self {
        Shader {
            rc: self.rc.clone(),
            program: self.program,
            vertex_uniform_location: self.vertex_uniform_location,
            texture_locations: self.texture_locations,
            phantom: PhantomData,
        }
    }
}

impl<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> Shader<T, TEXTURES> {
    /// Creates a new shader.
    pub fn new(_ctx: &mut Context) -> Shader<T, TEXTURES> {
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
            rc: Rc::new(()),
            program,
            vertex_uniform_location,
            texture_locations,
            phantom: PhantomData,
        }
    }

    /// Creates a new parameterized instance of this shader that you can store state on and draw
    /// with.
    pub fn new_instance(&mut self) -> ShaderInstance<T, TEXTURES> {
        let ctx = &mut OpenGLState::ctx();
        let default_texture = ctx.default_texture();
        ShaderInstance {
            shader: self.clone(),
            vertex_uniform: UniformBuffer::new(T::VERTEX_UNIFORM_DEFAULT),
            textures: self.texture_locations.map(|_| default_texture.clone()),
            vertices: Buffer::new(BufferBindingTarget::ArrayBuffer),
        }
    }
}

impl<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> Drop for Shader<T, TEXTURES> {
    fn drop(&mut self) {
        if Rc::<()>::strong_count(&self.rc) == 1 {
            let gl = &mut OpenGLState::ctx().gl;
            gl.delete_program(self.program);
        }
    }
}

pub struct ShaderInstance<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> {
    shader: Shader<T, TEXTURES>,
    vertex_uniform: UniformBuffer<T::VertexUniformType>,
    textures: [Texture; TEXTURES],
    vertices: Buffer<T::VertexDescriptor>,
}

impl<T: ShaderDescriptor<TEXTURES>, const TEXTURES: usize> ShaderInstance<T, TEXTURES> {
    /// Sets the vertex uniforms to use during the draw.
    pub fn set_vertex_uniform(&mut self, uniform: T::VertexUniformType) {
        self.vertex_uniform.set(uniform);
    }

    /// Sets the textures to use during the draw.
    pub fn set_textures(&mut self, textures: [&Texture; TEXTURES]) {
        self.textures = textures.map(|texture| texture.clone());
    }

    /// Sets the vertices that will be drawn.
    pub fn set_vertices(&mut self, vertices: &[T::VertexDescriptor]) {
        self.vertices.set(vertices);
    }

    /// Clears all the vertices, resulting in nothing being drawable.
    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
    }

    fn bind(&self) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.use_program(Some(self.shader.program));
        self.vertex_uniform.bind(0);
        for i in 0..TEXTURES {
            self.textures[i].bind(i as u32);
            gl.uniform_1_i32(Some(&self.shader.texture_locations[i]), i as i32);
        }
        self.vertices.bind();
    }

    /// Draws the instance to the screen.
    pub fn draw_instanced(&mut self) {
        if self.vertices.len() > 0 {
            self.bind();
            let gl = &mut OpenGLState::ctx().gl;
            gl.draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, self.vertices.len() as i32);
        }
    }
}
