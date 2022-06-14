use crate::color::R8;
use crate::graphics::{
    shaders::text::{Text, TextSprite, TextUserData},
    std140,
    texture_atlas::TextureAtlas,
    Buffer, DrawMode, Shader, ShaderDescriptor, Texture, TextureFiltering, TextureSection, Uniform,
};
use crate::image::Image;
use crate::{App, Context};
use alloc::vec::Vec;
use cgmath::*;
use fontdue::{
    layout::{CoordinateSystem, GlyphRasterConfig, Layout, LayoutSettings},
    Font,
};
use hashbrown::HashMap;

impl ShaderDescriptor<1> for TextShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = std140::mat4;
    type VertexDescriptor = TextSprite;
}

/// Shader object for sprites. This holds no mutable state, so it's recommended to reuse this as
/// much as possible.
pub struct TextShader {
    shader: Shader<TextShader, 1>,
}

impl TextShader {
    /// Creates a new text shader.
    pub fn new(ctx: &Context<impl App>) -> TextShader {
        TextShader {
            shader: Shader::new(ctx),
        }
    }

    /// Draws to the screen.
    pub fn draw(&self, uniform: &Uniform<std140::mat4>, atlas: &Texture, buffer: &Buffer<TextSprite>) {
        self.shader.draw(DrawMode::TriangleStrip, uniform, [atlas], &[buffer]);
    }
}

#[derive(Debug, Copy, Clone)]
struct CharCacheValue {
    uv: TextureSection,
    size: Vector2<f32>,
}

/// Holds the state required to cache and draw text to the screen.
pub struct TextShaderPass {
    uniform: Uniform<std140::mat4>,
    atlas: TextureAtlas,
    buffer: Buffer<TextSprite>,

    sprites: Vec<TextSprite>,
    layout: Layout<TextUserData>,
    cache: HashMap<GlyphRasterConfig, CharCacheValue>,
    dirty: bool,
}

impl TextShaderPass {
    pub fn new(ctx: &Context<impl App>, ortho: Matrix4<f32>) -> TextShaderPass {
        let max = ctx.max_texture_size().min(4096) as u32;
        TextShaderPass {
            uniform: Uniform::new(ctx, ortho),
            atlas: TextureAtlas::new::<R8, _>(ctx, max, TextureFiltering::none()),
            buffer: Buffer::new(ctx),

            sprites: Vec::new(),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            cache: HashMap::new(),
            dirty: false,
        }
    }

    /// Sets the orthographic projection used to draw this pass. If none is passed, this function
    /// does nothing.
    pub fn set_ortho(&mut self, ortho: Matrix4<f32>) {
        self.uniform.set(ortho);
    }

    /// Draws the pass to the screen.
    pub fn draw(&mut self, shader: &TextShader) {
        if self.sprites.len() > 0 {
            if self.dirty {
                self.dirty = false;
                self.buffer.set_data(&self.sprites);
            }
            shader.draw(&self.uniform, self.atlas.get(), &self.buffer);
        }
    }

    /// Appends text to the instance.
    pub fn append(&mut self, fonts: &[Font], layout: &LayoutSettings, styles: &[Text]) {
        self.layout.reset(layout);
        for style in styles {
            self.layout.append(fonts, &style.into());
        }
        // log::info!("{:?}", self.layout.glyphs());
        // log::info!("{:?}", self.layout.lines());
        for glyph in self.layout.glyphs() {
            if glyph.width == 0 {
                continue;
            }
            let value = match self.cache.get(&glyph.key).copied() {
                Some(value) => value,
                None => {
                    let font = &fonts[glyph.font_index];
                    let (metrics, bitmap) = font.rasterize_config(glyph.key);
                    // info!("{:?}", metrics); // Debug
                    let image = Image::from_vec(bitmap, metrics.width as u32, metrics.height as u32);
                    let uv = self.atlas.pack(&image).expect("Text packer is full.");
                    let value = CharCacheValue {
                        uv,
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
}
