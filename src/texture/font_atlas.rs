use crate::font::*;
use crate::texture::packer::*;
use crate::texture::*;
use crate::types::*;
use cgmath::*;
use hashbrown::HashMap;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct CharCacheKey {
    font: usize,
    character: char,
    scale: u32,
}

#[derive(Debug, Copy, Clone)]
struct CharCacheValue {
    visible: bool,
    uv: Vector4<u16>,
    size: Vector2<f32>,
    offset: Vector2<f32>,
    advance_width: f32,
}

pub struct FontAtlas {
    packer: TexturePacker,
    cache: HashMap<CharCacheKey, CharCacheValue>,
    fonts: Vec<Font>,
    dirty: bool,
}

impl FontAtlas {
    pub fn new() -> FontAtlas {
        let mut manager = FontAtlas {
            packer: TexturePacker::new(),
            cache: HashMap::new(),
            fonts: Vec::new(),
            dirty: true,
        };
        manager.add_font_bytes(include_bytes!("./resources/RobotoMono-Regular.ttf") as &[u8]);
        manager.sync();
        manager
    }

    pub fn add_font_bytes(&mut self, bytes: &'static [u8]) {
        self.fonts.push(Font::from_bytes(bytes));
        info!("Loaded font from bytes");
    }

    pub fn add_font_path(&mut self, path: &str) {
        self.fonts.push(Font::from_path(path));
        info!("Loaded font from path: {}", path);
    }

    pub fn rasterize(&mut self, desc: &Text, quads: &mut Vec<Sprite>) {
        // Needed for glyph calculation.
        let font_index = desc.font.key();
        let font = &self.fonts[font_index];
        let scale = desc.scale;

        // Needed for vertex layout.
        let advance_height = font.advance_height(scale);
        let max_width = desc.max_width.unwrap_or(std::f32::MAX);
        let mut caret = Vector2::zero();

        for c in desc.string.nfc() {
            let key = CharCacheKey {
                font: font_index,
                character: c,
                scale: scale,
            };

            let value = match self.cache.get(&key).copied() {
                Some(value) => value,
                None => {
                    let mut value = CharCacheValue {
                        visible: false,
                        uv: Vector4::zero(),
                        size: Vector2::zero(),
                        offset: Vector2::zero(),
                        advance_width: font.advance_width(c, scale),
                    };
                    if let Some(glyph) = font.render_glyph(c, scale) {
                        value.visible = true;
                        value.offset = glyph.offset;
                        value.size = Vector2::new(glyph.size.x as f32, glyph.size.y as f32);
                        let texture =
                            Image::from_color_Vec(&glyph.data, glyph.size.x as u32, glyph.size.y as u32);
                        value.uv = self.packer.pack(&texture);
                        self.dirty = true;
                    }
                    self.cache.insert(key, value);
                    value
                },
            };

            // Create the vertex.
            if value.visible {
                quads.push(Sprite::new(
                    Vector3::new(
                        desc.pos.x + caret.x + value.offset.x,
                        desc.pos.y + caret.y - value.offset.y,
                        desc.pos.z,
                    ),
                    value.size,
                    Texture(value.uv),
                    desc.color,
                    0.0,
                ));
            }

            // Move the caret forward.
            caret.x += value.advance_width;
            if caret.x > max_width {
                caret.y -= advance_height;
                caret.x = 0.0;
            }
        }
    }

    pub fn sync(&mut self) -> Option<Image> {
        if self.dirty {
            self.dirty = false;
            Some(self.packer.export())
        } else {
            None
        }
    }
}
