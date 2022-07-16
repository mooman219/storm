use crate::graphics::{shaders::sprite::Sprite, std140, Shader, ShaderDescriptor};

impl ShaderDescriptor<1> for SpriteShaderDescriptor {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = std140::mat4;
    type VertexDescriptor = Sprite;
}

/// Describes the SpriteShader.
pub struct SpriteShaderDescriptor();

/// Shader object for sprites. This holds no mutable state, so it's recommended to reuse this as
/// much as possible.
pub type SpriteShader = Shader<SpriteShaderDescriptor, 1>;
