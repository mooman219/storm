use super::TransformLayer;
use crate::prelude::Sprite;
use crate::render::buffer::Buffer;
use crate::render::raw::{BufferBindingTarget, TextureUnit};
use crate::render::OpenGLState;
use crate::{colors, TextureSection, RGBA8};
use crate::{Image, Packer, Texture};
use cgmath::Matrix4;
use cgmath::*;
use fontdue::layout::{CoordinateSystem, GlyphRasterConfig, Layout, LayoutSettings, TextStyle};
use fontdue::Font;
use hashbrown::HashMap;

#[derive(Debug, Copy, Clone)]
struct CharCacheValue {
    uv: TextureSection,
    size: Vector2<f32>,
}

/// Simple layer which draws text to the screen.
pub struct TextLayer {
    transform: TransformLayer,
    buffer: Buffer<Sprite>,
    sprites: Vec<Sprite>,
    layout: Layout<RGBA8>,
    packer: Packer,
    cache: HashMap<GlyphRasterConfig, CharCacheValue>,
    atlas: Texture,
}

impl TextLayer {
    pub(crate) fn new() -> TextLayer {
        let ctx = OpenGLState::ctx();
        let max = ctx.max_texture_size().min(8192);
        let layer = TextLayer {
            transform: TransformLayer::new(),
            buffer: Buffer::new(BufferBindingTarget::ArrayBuffer),
            sprites: Vec::new(),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            packer: Packer::new(max as u32, max as u32),
            cache: HashMap::new(),
            atlas: Texture::from_image(&Image::from_color(colors::TRANSPARENT, max as u32, max as u32)),
        };
        layer
    }

    /// Draws the layer to the screen.
    pub fn draw(&mut self) {
        if self.sprites.len() > 0 {
            if self.sprites.len() != self.buffer.len() {
                self.buffer.set(&self.sprites);
            }
            let ctx = OpenGLState::ctx();
            self.atlas.bind(TextureUnit::Alpha);
            ctx.sprite.bind();
            ctx.sprite.set_ortho(self.transform.ortho_transform());
            ctx.sprite.set_texture(TextureUnit::Alpha);
            self.buffer.draw();
        }
    }

    /// Appends text to the layer.
    pub fn append(&mut self, fonts: &[Font], layout: &LayoutSettings, styles: &[TextStyle<RGBA8>]) {
        self.layout.reset(layout);
        for style in styles {
            self.layout.append(fonts, style);
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
                    let bitmap = into_rgba(bitmap);
                    let image = Image::from_vec(bitmap, metrics.width as u32, metrics.height as u32);
                    let rect = self
                        .packer
                        .pack(metrics.width as u32, metrics.height as u32)
                        .expect("Text packer is full.");
                    self.atlas.set_subsection(rect.x, rect.y, &image);
                    let value = CharCacheValue {
                        uv: self.atlas.subsection(rect.x, rect.x + rect.w, rect.y, rect.y + rect.h),
                        size: Vector2::new(metrics.width as f32, metrics.height as f32),
                    };
                    self.cache.insert(glyph.key, value);
                    value
                }
            };
            self.sprites.push(Sprite::new(
                Vector3::new(glyph.x, glyph.y, 0.0),
                value.size,
                value.uv,
                glyph.user_data,
                0.0,
            ));
        }
    }

    /// Clears all the text, drawing nothing.
    pub fn clear_text(&mut self) {
        self.sprites.clear();
    }

    /// Gets the transform settings for this layer.
    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.transform.set(transform);
    }
}

fn into_rgba(bitmap: Vec<u8>) -> Vec<RGBA8> {
    let mut output = Vec::with_capacity(bitmap.len());
    for v in bitmap {
        output.push(RGBA8::new_raw(v, v, v, v));
    }
    output
}
