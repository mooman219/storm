use crate::color;
use cgmath::*;
use rusttype;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub struct Font {
    font: rusttype::Font<'static>,
}

pub struct RenderedGlyph {
    pub size: Vector2<usize>,
    pub offset: Vector2<f32>,
    pub data: Vec<color::RGBA8>,
}

impl Font {
    pub fn from_bytes(bytes: &'static [u8]) -> Font {
        trace!("Loaded font raw");
        Font {
            font: rusttype::Font::from_bytes(bytes).expect("Unable to parse font"),
        }
    }

    pub fn from_path(path: &str) -> Font {
        let file = File::open(Path::new(path)).expect("Unable to read path");
        let mut file = BufReader::new(file);
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("Unable to read bytes");
        trace!("Loaded font from {}", path);
        Font {
            font: rusttype::Font::from_bytes(bytes).expect("Unable to parse font"),
        }
    }

    pub fn advance_height(&self, scale: u32) -> f32 {
        let v_metrics = self.font.v_metrics(rusttype::Scale::uniform(scale as f32));
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        advance_height
    }

    pub fn advance_width(&self, glyph: char, scale: u32) -> f32 {
        let glyph = self.font.glyph(glyph).scaled(rusttype::Scale::uniform(scale as f32));
        let advance_width = glyph.h_metrics().advance_width;
        advance_width
    }

    pub fn render_glyph(&self, glyph: char, scale: u32) -> Option<RenderedGlyph> {
        let glyph = self
            .font
            .glyph(glyph)
            .scaled(rusttype::Scale::uniform(scale as f32))
            .positioned(rusttype::point(0.0, 0.0));
        let rect = glyph.pixel_bounding_box();
        if let Some(rect) = rect {
            if rect.width() > 0 && rect.height() > 0 {
                let size = Vector2::new(rect.width() as usize, rect.height() as usize);
                let mut buffer = vec![color::BLACK; (size.x * size.y) as usize];
                glyph.draw(|x, y, v| {
                    let v = (v * 255.0).round().max(0.0).min(255.0) as u8;
                    buffer[(x as usize) + (y as usize) * size.x] = color::RGBA8::new_raw(255, 255, 255, v);
                });
                let rendered_glyph = RenderedGlyph {
                    size: size,
                    offset: Vector2::new(rect.min.x as f32, rect.max.y as f32),
                    data: buffer,
                };
                return Some(rendered_glyph);
            }
        }
        None
    }
}
