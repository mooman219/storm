use super::TrianglePoint;
use storm::graphics::{std140, Shader, ShaderDescriptor};

impl ShaderDescriptor<0> for TriangleShaderDescriptor {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 0] = [];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = std140::mat4;
    type VertexDescriptor = TrianglePoint;
}

pub struct TriangleShaderDescriptor();

pub type TriangleShader = Shader<TriangleShaderDescriptor, 0>;
