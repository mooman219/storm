use super::TrianglePoint;
use crate::TriangleApp;
use storm::cgmath::Matrix4;
use storm::Context;
use storm::{
    graphics::{std140, Buffer, DrawMode, Shader, ShaderDescriptor, Uniform},
    math::{OrthographicCamera, PerspectiveCamera},
};

impl ShaderDescriptor<0> for TriangleShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 0] = [];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = TriangleUniform;
    type VertexDescriptor = TrianglePoint;
}

#[std140::uniform]
#[derive(Copy, Clone)]
pub struct TriangleUniform {
    pub ortho: std140::mat4,
}

impl TriangleUniform {
    pub fn new(ortho: Matrix4<f32>) -> TriangleUniform {
        TriangleUniform {
            ortho: ortho.into(),
        }
    }
}

impl From<&mut OrthographicCamera> for TriangleUniform {
    fn from(item: &mut OrthographicCamera) -> Self {
        TriangleUniform::new(item.matrix())
    }
}

impl From<&mut PerspectiveCamera> for TriangleUniform {
    fn from(item: &mut PerspectiveCamera) -> Self {
        TriangleUniform::new(item.matrix())
    }
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

    pub fn draw(&self, uniform: &Uniform<TriangleUniform>, buffers: &[impl AsRef<Buffer<TrianglePoint>>]) {
        self.shader.draw(DrawMode::Triangles, uniform, [], buffers);
    }
}
