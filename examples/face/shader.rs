use super::Face;
use crate::CubeApp;
use storm::cgmath::Matrix4;
use storm::Context;
use storm::{
    graphics::{std140, Buffer, DrawMode, Shader, ShaderDescriptor, Texture, Uniform},
    math::{OrthographicCamera, PerspectiveCamera},
};

impl ShaderDescriptor<1> for FaceShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = FaceUniform;
    type VertexDescriptor = Face;
}

#[std140::uniform]
#[derive(Copy, Clone)]
pub struct FaceUniform {
    pub ortho: std140::mat4,
}

impl FaceUniform {
    pub fn new(ortho: Matrix4<f32>) -> FaceUniform {
        FaceUniform {
            ortho: ortho.into(),
        }
    }
}

impl From<&mut OrthographicCamera> for FaceUniform {
    fn from(item: &mut OrthographicCamera) -> Self {
        FaceUniform::new(item.matrix())
    }
}

impl From<&mut PerspectiveCamera> for FaceUniform {
    fn from(item: &mut PerspectiveCamera) -> Self {
        FaceUniform::new(item.matrix())
    }
}

pub struct FaceShader {
    shader: Shader<FaceShader, 1>,
}

impl FaceShader {
    pub fn new(ctx: &mut Context<CubeApp>) -> FaceShader {
        FaceShader {
            shader: Shader::new(ctx),
        }
    }

    pub fn draw(
        &self,
        uniform: &Uniform<FaceUniform>,
        atlas: &Texture,
        buffers: &[impl AsRef<Buffer<Face>>],
    ) {
        self.shader.draw_instanced(DrawMode::TriangleStrip, uniform, [atlas], buffers, 4);
    }
}
