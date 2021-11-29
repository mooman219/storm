use super::TransformLayer;
use crate::render::buffer::Buffer;
use crate::render::raw::{BufferBindingTarget, TextureUnit};
use crate::render::OpenGLState;
use crate::{Image, Packer, Text, TextSprite, TextUserData, Texture, TextureSection};
use cgmath::Matrix4;
use cgmath::*;
use fontdue::layout::{CoordinateSystem, GlyphRasterConfig, Layout, LayoutSettings};
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
    buffer: Buffer<TextSprite>,
    sprites: Vec<TextSprite>,
    layout: Layout<TextUserData>,
    packer: Packer,
    cache: HashMap<GlyphRasterConfig, CharCacheValue>,
    atlas: Texture<u8>,
    dirty: bool,
}

impl TextLayer {
    /// Creates a new text layer. Layers represent draw calls and hold configuration associated
    /// with drawing to the screen.
    pub(crate) fn new() -> TextLayer {
        let ctx = OpenGLState::ctx();
        let max = ctx.max_texture_size().min(4096) as u32;
        let layer = TextLayer {
            transform: TransformLayer::new(),
            buffer: Buffer::new(BufferBindingTarget::ArrayBuffer),
            sprites: Vec::new(),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            packer: Packer::new(max, max),
            cache: HashMap::new(),
            atlas: Texture::from_image(&Image::from_color(0u8, max, max)),
            dirty: false,
        };
        layer
    }

    /// Draws the layer to the screen.
    pub fn draw(&mut self) {
        if self.sprites.len() > 0 {
            if self.dirty {
                self.dirty = false;
                self.buffer.set(&self.sprites);
            }
            let ctx = OpenGLState::ctx();
            self.atlas.bind(TextureUnit::Alpha);
            ctx.text.bind();
            ctx.text.set_ortho(self.transform.ortho_transform());
            ctx.text.set_texture(TextureUnit::Alpha);
            self.buffer.draw();
        }
    }

    /// Appends text to the layer.
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

    /// Gets the transform settings for this layer.
    pub fn set_transform(&mut self, transform: Matrix4<f32>) {
        self.transform.set(transform);
    }
}
