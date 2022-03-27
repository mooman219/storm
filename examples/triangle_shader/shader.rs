use super::TrianglePoint;
use crate::TriangleApp;

use storm::graphics::{std140, Buffer, DrawMode, Shader, ShaderDescriptor, Uniform};
use storm::Context;

impl ShaderDescriptor<0> for TriangleShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 0] = [];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = std140::mat4;
    type VertexDescriptor = TrianglePoint;
}

pub struct TriangleShader {
    shader: Shader<TriangleShader, 0>,
}

impl TriangleShader {
    pub fn new(ctx: &mut Context<TriangleApp>) -> TriangleShader {
        TriangleShader {
            shader: Shader::new(ctx),
        }
    }

    pub fn draw(&self, uniform: &Uniform<std140::mat4>, buffers: &[impl AsRef<Buffer<TrianglePoint>>]) {
        self.shader.draw(DrawMode::Triangles, uniform, [], buffers);
    }
}
