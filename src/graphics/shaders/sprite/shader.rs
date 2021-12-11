use crate::graphics::shaders::sprite::Sprite;
use crate::graphics::{AsStd140, Shader, ShaderDescriptor, ShaderInstance};
use crate::math::Transform;
use crate::render::OpenGLState;
use crate::{Context, Texture};
use cgmath::Matrix4;

impl ShaderDescriptor<1> for SpriteShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    const VERTEX_UNIFORM_DEFAULT: SpriteUniform = SpriteUniform {
        ortho: Matrix4::new(
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ),
    };
    type VertexUniformType = SpriteUniform;
    type VertexDescriptor = Sprite;
}

#[derive(AsStd140)]
pub struct SpriteUniform {
    ortho: Matrix4<f32>,
}

pub struct SpriteShader {
    shader: Shader<SpriteShader, 1>,
}

impl SpriteShader {
    pub fn new(ctx: &mut Context) -> SpriteShader {
        SpriteShader {
            shader: Shader::new(ctx),
        }
    }

    pub fn new_instance(&mut self) -> SpriteShaderInstance {
        SpriteShaderInstance {
            instance: self.shader.new_instance(),
            transform: Transform::new(OpenGLState::ctx().logical_size()),
        }
    }
}

pub struct SpriteShaderInstance {
    instance: ShaderInstance<SpriteShader, 1>,
    transform: Transform,
}

impl SpriteShaderInstance {
    /// Sets the texture atlas to use during the draw.
    pub fn set_atlas(&mut self, handle: &Texture) {
        self.instance.set_textures([handle]);
    }

    /// Sets the sprites that will be drawn.
    pub fn set_sprites(&mut self, sprites: &[Sprite]) {
        self.instance.set_vertices(sprites);
    }

    /// Clears all the sprites, resulting in nothing being drawable.
    pub fn clear_sprites(&mut self) {
        self.instance.clear_vertices();
    }

    /// Gets a mutable reference the transform settings for this instance.
    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    /// Draws the instance to the screen.
    pub fn draw(&mut self) {
        if let Some(transform) = self.transform.matrix() {
            self.instance.set_vertex_uniform(SpriteUniform {
                ortho: transform,
            });
        }
        self.instance.draw_instanced();
    }
}
