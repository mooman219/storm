use crate::texture::*;
use crate::*;
use cgmath::*;
use fontdue::layout::{
    CoordinateSystem, GlyphRasterConfig, HorizontalAlign, Layout, LayoutSettings, TextStyle, VerticalAlign,
};
use fontdue::{Font, FontSettings};
use hashbrown::HashMap;

#[derive(Debug, Copy, Clone)]
struct CharCacheValue {
    uv: Vector4<u16>,
    size: Vector2<f32>,
}

pub struct TextCache {
    cache: HashMap<GlyphRasterConfig, CharCacheValue>,
    layout: Layout,
    fonts: Vec<Font>,
    dirty: bool,
}

impl TextCache {
    pub fn new() -> TextCache {
        let mut manager = TextCache {
            cache: HashMap::new(),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
            fonts: Vec::new(),
            dirty: true,
        };
        manager.add_font_bytes(include_bytes!("fonts/Roboto-Regular.ttf") as &[u8]);
        manager
    }

    pub fn add_font_bytes(&mut self, bytes: &[u8]) -> usize {
        let index = self.fonts.len();
        let settings = FontSettings {
            scale: 50.0,
            ..FontSettings::default()
        };
        self.fonts.push(Font::from_bytes(bytes, settings).expect("Unable to parse font."));
        info!("Loaded font from bytes.");
        index
    }

    // pub fn add_font_path(&mut self, path: &str) -> usize {
    //     let file = File::open(Path::new(path)).expect("Unable to read path.");
    //     let mut file = BufReader::new(file);
    //     let mut bytes = Vec::new();
    //     file.read_to_end(&mut bytes).expect("Unable to read bytes.");
    //     self.add_font_bytes(&bytes)
    // }

    pub fn rasterize(&mut self, atlas: &mut TextureAtlas, desc: &Text, quads: &mut Vec<Sprite>) {
        let font_index = desc.font.key();
        let font = &self.fonts[font_index];
        self.layout.reset(&LayoutSettings {
            x: desc.pos.x,
            y: desc.pos.y,
            max_width: desc.max_width,
            max_height: Some(500.0),
            horizontal_align: HorizontalAlign::Center,
            vertical_align: VerticalAlign::Middle,
            ..LayoutSettings::default()
        });
        let style = TextStyle::new(&desc.string, desc.scale as f32, font_index);
        self.layout.append(self.fonts.as_slice(), &style);

        for &position in self.layout.glyphs() {
            if position.width == 0 {
                continue;
            }
            let value = match self.cache.get(&position.key).copied() {
                Some(value) => value,
                None => {
                    let (metrics, bitmap) = font.rasterize_config(position.key);
                    let bitmap = Self::alpha_to_rgba(&bitmap);
                    let texture = Image::from_vec(bitmap, metrics.width as u32, metrics.height as u32);
                    let value = CharCacheValue {
                        uv: atlas.add(texture),
                        size: Vector2::new(metrics.width as f32, metrics.height as f32),
                    };
                    self.cache.insert(position.key, value);
                    value
                }
            };
            quads.push(Sprite::new(
                Vector3::new(position.x, position.y, desc.pos.z),
                value.size,
                Texture(value.uv),
                desc.color,
                0.0,
            ));
        }
    }

    fn alpha_to_rgba(bitmap: &Vec<u8>) -> Vec<RGBA8> {
        let mut output = Vec::with_capacity(bitmap.len());
        for v in bitmap {
            let v = *v;
            output.push(RGBA8::new_raw(v, v, v, v));
        }
        output
    }
}
