use crate::graphics::{
    max_texture_size,
    shaders::text::{Text, TextSprite, TextUserData},
    AsStd140, Buffer, DrawMode, Shader, ShaderDescriptor, Texture, TextureSection, Uniform,
};
use crate::image::{Image, Packer};
use crate::*;
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
    type VertexUniformType = TextUniform;
    type VertexDescriptor = TextSprite;
}

#[derive(AsStd140)]
pub struct TextUniform {
    ortho: Matrix4<f32>,
}

impl TextUniform {
    pub fn new(ortho: Matrix4<f32>) -> TextUniform {
        TextUniform {
            ortho,
        }
    }
}

pub struct TextShader {
    shader: Shader<TextShader, 1>,
}

impl TextShader {
    pub fn new() -> TextShader {
        TextShader {
            shader: Shader::new(),
        }
    }

    /// Draws to the screen.
    pub fn draw(&self, uniform: &Uniform<TextUniform>, atlas: &Texture, buffer: &Buffer<TextSprite>) {
        self.shader.draw_instanced(DrawMode::TriangleStrip, uniform, [atlas], &[buffer], 4);
    }
}

#[derive(Debug, Copy, Clone)]
struct CharCacheValue {
    uv: TextureSection,
    size: Vector2<f32>,
}

/// Holds the state required to cache and draw text to the screen.
pub struct TextShaderPass {
    uniform: Uniform<TextUniform>,
    atlas: Texture,
    buffer: Buffer<TextSprite>,

    sprites: Vec<TextSprite>,
    layout: Layout<TextUserData>,
    packer: Packer,
    cache: HashMap<GlyphRasterConfig, CharCacheValue>,
    dirty: bool,
}

impl TextShaderPass {
    pub fn new(ortho: Matrix4<f32>) -> TextShaderPass {
        let max = max_texture_size().min(4096) as u32;
        let atlas = Texture::from_image(&Image::from_color(0u8, max, max));
        TextShaderPass {
            uniform: Uniform::new(TextUniform::new(ortho)),
            atlas,
            buffer: Buffer::new(),

            sprites: Vec::new(),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            packer: Packer::new(max, max),
            cache: HashMap::new(),
            dirty: false,
        }
    }

    /// Sets the orthographic projection used to draw this pass. If none is passed, this function
    /// does nothing.
    pub fn set_ortho(&mut self, ortho: Matrix4<f32>) {
        self.uniform.set(TextUniform::new(ortho));
    }

    /// Draws the pass to the screen.
    pub fn draw(&mut self, shader: &TextShader) {
        if self.sprites.len() > 0 {
            if self.dirty {
                self.dirty = false;
                self.buffer.set(&self.sprites);
            }
            shader.draw(&self.uniform, &self.atlas, &self.buffer);
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
                    // info!("{:?}", metrics); // Debug
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
}
