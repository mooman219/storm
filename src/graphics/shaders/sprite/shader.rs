use crate::{
    graphics::{
        shaders::sprite::Sprite, AsStd140, Buffer, DrawMode, Shader, ShaderDescriptor, Texture, Uniform,
    },
    math::Transform,
};
use cgmath::Matrix4;

impl ShaderDescriptor<1> for SpriteShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = SpriteUniform;
    type VertexDescriptor = Sprite;
}

/// The uniform for sprites is a simple transformation matrix.
#[derive(AsStd140)]
pub struct SpriteUniform {
    /// The matrix.
    pub ortho: Matrix4<f32>,
}

impl SpriteUniform {
    /// Helper function to create a new SpriteUniform.
    pub fn new(ortho: Matrix4<f32>) -> SpriteUniform {
        SpriteUniform {
            ortho,
        }
    }
}

impl From<&mut Transform> for SpriteUniform {
    fn from(item: &mut Transform) -> Self {
        SpriteUniform::new(item.matrix())
    }
}

/// Shader object for sprites. This holds no mutable state, so it's recommended to reuse this as
/// much as possible.
pub struct SpriteShader {
    shader: Shader<SpriteShader, 1>,
}

impl SpriteShader {
    /// Creates a new sprite shader.
    pub fn new() -> SpriteShader {
        SpriteShader {
            shader: Shader::new(),
        }
    }

    /// Helper function to draw sprites to the screen.
    pub fn draw(&self, uniform: &Uniform<SpriteUniform>, atlas: &Texture, buffers: &[&Buffer<Sprite>]) {
        self.shader.draw_instanced(DrawMode::TriangleStrip, uniform, [atlas], buffers, 4);
    }
}
