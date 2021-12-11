use crate::graphics::shaders::text::{Text, TextSprite, TextUserData};
use crate::graphics::{AsStd140, Shader, ShaderDescriptor, ShaderInstance, TextureSection};
use crate::image::{Image, Packer};
use crate::math::Transform;
use crate::render::OpenGLState;
use crate::{Context, Texture};
use cgmath::Matrix4;
use cgmath::*;
use fontdue::layout::{CoordinateSystem, GlyphRasterConfig, Layout, LayoutSettings};
use fontdue::Font;
use hashbrown::HashMap;

impl ShaderDescriptor<1> for TextShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    const VERTEX_UNIFORM_DEFAULT: TextUniform = TextUniform {
        ortho: Matrix4::new(
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ),
    };
    type VertexUniformType = TextUniform;
    type VertexDescriptor = TextSprite;
}

#[derive(AsStd140)]
pub struct TextUniform {
    ortho: Matrix4<f32>,
}

pub struct TextShader {
    shader: Shader<TextShader, 1>,
    max: u32,
}

impl TextShader {
    pub fn new(ctx: &mut Context) -> TextShader {
        TextShader {
            shader: Shader::new(ctx),
            max: OpenGLState::ctx().max_texture_size().min(4096) as u32,
        }
    }

    pub fn new_instance(&mut self) -> TextShaderInstance {
        let mut instance = self.shader.new_instance();
        let atlas = Texture::from_image(&Image::from_color(0u8, self.max, self.max));
        instance.set_textures([&atlas]);
        TextShaderInstance {
            transform: Transform::new(OpenGLState::ctx().logical_size()),
            sprites: Vec::new(),
            instance,
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            packer: Packer::new(self.max, self.max),
            cache: HashMap::new(),
            atlas,
            dirty: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct CharCacheValue {
    uv: TextureSection,
    size: Vector2<f32>,
}

pub struct TextShaderInstance {
    transform: Transform,
    sprites: Vec<TextSprite>,
    instance: ShaderInstance<TextShader, 1>,
    layout: Layout<TextUserData>,
    packer: Packer,
    cache: HashMap<GlyphRasterConfig, CharCacheValue>,
    atlas: Texture,
    dirty: bool,
}

impl TextShaderInstance {
    /// Draws the instance to the screen.
    pub fn draw(&mut self) {
        if self.sprites.len() > 0 {
            if self.dirty {
                self.dirty = false;
                self.instance.set_vertices(&self.sprites);
            }
            if let Some(transform) = self.transform.matrix() {
                self.instance.set_vertex_uniform(TextUniform {
                    ortho: transform,
                });
            }
            self.instance.draw_instanced();
        }
    }

    /// Appends text to the instance.
    pub fn append(&mut self, fonts: &[Font], layout: &LayoutSettings, styles: &[Text]) {
        self.layout.reset(layout);
        for style in styles {
            self.layout.append(fonts, &style.into());
        }
        for glyph in self.layout.glyphs() {
            if glyph.width == 0 {
                continue;
            }
            let value = match self.cache.get(&glyph.key).copied() {
                Some(value) => value,
                None => {
                    let font = &fonts[glyph.font_index];
                    let (metrics, bitmap) = font.rasterize_config(glyph.key);
                    let rect = self
                        .packer
                        .pack(metrics.width as u32, metrics.height as u32)
                        .expect("Text packer is full.");
                    self.atlas.set(
                        rect.x,
                        rect.y,
                        &Image::from_vec(bitmap, metrics.width as u32, metrics.height as u32),
                    );
                    let value = CharCacheValue {
                        uv: self.atlas.subsection(rect.x, rect.x + rect.w, rect.y, rect.y + rect.h),
                        size: Vector2::new(metrics.width as f32, metrics.height as f32),
                    };
                    self.cache.insert(glyph.key, value);
                    value
                }
            };
            self.sprites.push(TextSprite::new(
                Vector3::new(glyph.x, glyph.y, glyph.user_data.depth),
                value.size,
                value.uv,
                glyph.user_data.color,
            ));
            self.dirty = true;
        }
    }

    /// Clears all the text, drawing nothing.
    pub fn clear_text(&mut self) {
        self.sprites.clear();
        self.dirty = true;
    }

    /// Gets a mutable reference the transform settings for this instance.
    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
