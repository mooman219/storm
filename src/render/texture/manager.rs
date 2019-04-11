use cgmath::*;
use color;
use hashbrown::HashMap;
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
    uv: Vec<Vector4<f32>>,
    dirty: bool,
}

impl TextureManager {
    pub fn new() -> TextureManager {
        TextureManager {
            packer: TexturePacker::new(TexturePackerConfig {
                max_width: 2048,
                max_height: 2048,
                texture_padding: 0,
            }),
            atlas: TextureHandle::new(TextureUnit::Atlas),
            uv: Vec::new(),
            dirty: false,
        }
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

    pub fn get_uv(&self, reference: &TextureReference) -> Vector4<f32> {
        self.uv[reference.key()]
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
struct TextCacheKey {
    font: usize,
    character: char,
}

pub struct TextManager {
    packer: TexturePacker,
    cache: HashMap<TextCacheKey, Vector4<f32>>,
    atlas: TextureHandle,
    fonts: Vec<Font<'static>>,
    dirty: bool,
}

impl TextManager {
    pub fn new() -> TextManager {
        TextManager {
            packer: TexturePacker::new(TexturePackerConfig {
                max_width: 2048,
                max_height: 2048,
                texture_padding: 0,
            }),
            cache: HashMap::new(),
            atlas: TextureHandle::new(TextureUnit::Font),
            fonts: Vec::new(),
            dirty: true,
        }
    }

    pub fn add_font_path(&mut self, path: &str) {
        let file = File::open(Path::new(path)).expect("Unable to read path");
        let mut file = BufReader::new(file);
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("Unable to read bytes");
        let font: Font<'static> = Font::from_bytes(bytes).expect("Unable to parse font");
        self.fonts.push(font);
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
        // TODO: Parallelize this
        for c in text.nfc() {
            let key = TextCacheKey {
                font: desc.font.key(),
                character: c,
                // TODO: Add scale
            };
            let entry = self.cache.get(&key).cloned();
            let uv = match entry {
                Some(uv) => uv,
                None => {
                    self.dirty = true;
                    let glyph = font.glyph(c).scaled(Scale::uniform(16.0)).positioned(point(0.0, 0.0));
                    // TODO: Replace the glyph with a blank zero width one if
                    // the bounding box is missing.
                    let bounding_box = glyph
                        .pixel_bounding_box()
                        .expect("Why are we missing a pixel bounding box");
                    let width = bounding_box.width() as u32;
                    let height = bounding_box.height() as u32;
                    buffer.resize((width * height) as usize, color::BLACK);
                    glyph.draw(|x, y, v| {
                        let index = x + y * width;
                        buffer[index as usize] = color::Color::new(1.0 * v, 1.0 * v, 1.0 * v, v)
                    });
                    let texture = Texture::from_color_Vec(&buffer, width, height);
                    let uv = self.packer.pack(&texture);
                    self.cache.insert(key, uv);
                    trace!("Cached character {} at {:?}", c, uv);
                    uv
                },
            };
            // uvs.push(uv);
        }
        // TODO: Actually generate vertices
        vertices
    }
}
