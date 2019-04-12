use cgmath::*;
use color;
use hashbrown::HashMap;
use rayon::prelude::*;
use render::raw::*;
use render::texture::handle::*;
use render::texture::packer::*;
use render::texture::*;
use render::vertex::*;
use rusttype::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use text::*;
use texture::*;
use unicode_normalization::UnicodeNormalization;

pub struct TextureManager {
    packer: TexturePacker,
    atlas: TextureHandle,
    uv: Vec<Vector4<u16>>,
    dirty: bool,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        let mut manager = TextureManager {
            packer: TexturePacker::new(),
            atlas: TextureHandle::new(TextureUnit::Atlas),
            uv: Vec::new(),
            dirty: false,
        };
        manager.add(Texture::from_default(color::WHITE, 8, 8));
        manager.sync();
        manager
    }

    pub fn add(&mut self, texture: Texture) {
        let uv = self.packer.pack(&texture);
        self.uv.push(uv);
        self.dirty = true;
        trace!("Loaded raw texture at {:?}", uv);
    }

    pub fn add_path(&mut self, path: &str) {
        let uv = self.packer.pack_path(Path::new(path));
        self.uv.push(uv);
        self.dirty = true;
        trace!("Loaded texture {} at {:?}", path, uv);
    }

    pub fn sync(&mut self) {
        if self.dirty {
            let new_atlas = self.packer.export();
            self.atlas.set_texture(&new_atlas);
            self.dirty = false;
        }
    }

    pub fn get_uv(&self, reference: &TextureReference) -> Vector4<u16> {
        self.uv[reference.key()]
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct TextCacheKey {
    font: usize,
    character: char,
    scale: u32,
}

#[derive(Debug, Copy, Clone)]
struct TextCacheValue {
    visible: bool,
    uv: Vector4<u16>,
    size: Vector2<f32>,
    advance_width: f32,
}

pub struct TextManager {
    packer: TexturePacker,
    cache: HashMap<TextCacheKey, TextCacheValue>,
    atlas: TextureHandle,
    fonts: Vec<Font<'static>>,
    dirty: bool,
}

const TEXT_SCALE: f32 = 1.0 / 64.0;

impl TextManager {
    pub fn new() -> TextManager {
        let mut manager = TextManager {
            packer: TexturePacker::new(),
            cache: HashMap::new(),
            atlas: TextureHandle::new(TextureUnit::Font),
            fonts: Vec::new(),
            dirty: true,
        };
        manager.add_font_bytes(include_bytes!("./font/PressStart2P.ttf") as &[u8]);
        manager.sync();
        manager
    }

    pub fn add_font_bytes(&mut self, bytes: &'static [u8]) {
        self.fonts.push(Font::from_bytes(bytes).expect("Unable to parse font"));
        trace!("Loaded raw font");
    }

    pub fn add_font_path(&mut self, path: &str) {
        let file = File::open(Path::new(path)).expect("Unable to read path");
        let mut file = BufReader::new(file);
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("Unable to read bytes");
        self.fonts.push(Font::from_bytes(bytes).expect("Unable to parse font"));
        trace!("Loaded font {}", path);
    }

    pub fn sync(&mut self) {
        if self.dirty {
            let new_atlas = self.packer.export();
            self.atlas.set_texture(&new_atlas);
            self.dirty = false;
        }
    }

    pub fn rasterize(&mut self, text: &str, desc: &TextDescription) -> Vec<TextureVertex> {
        let mut vertices = Vec::new();
        let mut buffer = Vec::new();
        let font = self.fonts.get(desc.font.key()).expect("Unknown font reference");
        let mut caret = desc.pos;
        // TODO: Parallelize this
        // text.nfc().into_iter().par_iter().map();
        for c in text.nfc() {
            let key = TextCacheKey {
                font: desc.font.key(),
                character: c,
                scale: desc.scale,
            };
            let entry = self.cache.get(&key).cloned();
            let value = match entry {
                Some(value) => value,
                None => {
                    let mut value = TextCacheValue {
                        visible: false,
                        uv: Vector4::zero(),
                        size: Vector2::zero(),
                        advance_width: 1.0,
                    };
                    let glyph = font.glyph(c).scaled(Scale::uniform(desc.scale as f32));
                    value.advance_width = glyph.h_metrics().advance_width * TEXT_SCALE;
                    let glyph = glyph.positioned(point(0.0, 0.0));

                    let rect = glyph.pixel_bounding_box();
                    if let Some(rect) = rect {
                        if rect.width() > 0 && rect.height() > 0 {
                            value.size = Vector2::new(rect.width() as f32, rect.height() as f32) * TEXT_SCALE;
                            let size = Vector2::new(rect.width() as u32, rect.height() as u32);
                            buffer.resize((size.x * size.y) as usize, color::BLACK);
                            glyph.draw(|x, y, v| {
                                buffer[(x + y * size.x) as usize] = color::Color::new(1.0, 1.0, 1.0, v)
                            });
                            let texture = Texture::from_color_Vec(&buffer, size.x, size.y);
                            value.uv = self.packer.pack(&texture);
                            value.visible = true;
                            self.dirty = true;
                        }
                    }

                    // trace!("Cached {:?} {:?}", key, value);
                    self.cache.insert(key, value);
                    value
                },
            };
            caret.x += value.advance_width;
            if value.visible {
                vertices.push(TextureVertex::new(caret, value.size, value.uv, desc.color));
            }
        }
        vertices
    }
}
