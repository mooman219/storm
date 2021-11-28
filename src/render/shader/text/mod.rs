use crate::render::raw::{resource, OpenGL, TextureUnit};
use crate::render::OpenGLState;
use cgmath::Matrix4;

const VERTEX: &str = include_str!("../sprite/vertex.glsl");
const FRAGMENT: &str = include_str!("fragment.glsl");

pub struct TextShader {
    program: resource::Program,
    uniform_ortho: resource::UniformLocation,
    uniform_texture: resource::UniformLocation,
}

impl TextShader {
    pub fn new(gl: &mut OpenGL) -> TextShader {
        let program = gl.shader_program(VERTEX, FRAGMENT);
        let uniform_ortho = gl.get_uniform_location(program, "ortho").unwrap();
        let uniform_texture = gl.get_uniform_location(program, "tex[0]").unwrap();

        TextShader {
            program,
            uniform_ortho,
            uniform_texture,
        }
    }

    pub fn bind(&self) {
        let ctx = OpenGLState::ctx();
        ctx.gl.use_program(Some(self.program));
    }

    pub fn set_ortho(&mut self, ortho: &Matrix4<f32>) {
        let ctx = OpenGLState::ctx();
        ctx.gl.uniform_matrix_4fv(Some(&self.uniform_ortho), false, ortho.as_ref());
    }

    pub fn set_texture(&mut self, unit: TextureUnit) {
        let ctx = OpenGLState::ctx();
        ctx.gl.uniform_1i(Some(&self.uniform_texture), unit.index());
    }
}

impl Drop for TextShader {
    fn drop(&mut self) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.delete_program(self.program);
    }
}
