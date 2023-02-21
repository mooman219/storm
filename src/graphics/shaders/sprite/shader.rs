use crate::graphics::{shaders::sprite::Sprite, Shader, ShaderDescriptor};

impl ShaderDescriptor for SpriteShaderDescriptor {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: &'static [&'static str] = &["tex"];
    const UNIFORM_NAMES: &'static [&'static str] = &["vertex"];
    type VertexDescriptor = Sprite;
}

/// Describes the SpriteShader.
pub struct SpriteShaderDescriptor();

/// Shader object for sprites. This holds no mutable state, so it's recommended to reuse this as
/// much as possible.
pub type SpriteShader = Shader<SpriteShaderDescriptor>;
