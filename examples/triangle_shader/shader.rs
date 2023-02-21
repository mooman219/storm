use super::TrianglePoint;
use storm::graphics::{Shader, ShaderDescriptor};

impl ShaderDescriptor for TriangleShaderDescriptor {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: &'static [&'static str] = &[];
    const UNIFORM_NAMES: &'static [&'static str] = &["vertex"];
    type VertexDescriptor = TrianglePoint;
}

pub struct TriangleShaderDescriptor();

pub type TriangleShader = Shader<TriangleShaderDescriptor>;
