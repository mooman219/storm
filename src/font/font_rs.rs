use cgmath::*;
use color;
use font_rs::*;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::Read;
// use std::path::Path;

pub struct Font {
    font: font::Font<'static>,
}

pub struct RenderedGlyph {
    pub size: Vector2<usize>,
    pub topSideBearing: f32,
    pub data: Vec<color::Color>,
}

impl Font {
    pub fn from_bytes(bytes: &'static [u8]) -> Font {
        Font {
            // bytes: bytes,
            font: font::parse(bytes).expect("Unable to parse font"),
        }
    }

    // pub fn from_path(path: &str) -> Font {
    //     let file = File::open(Path::new(path)).expect("Unable to read path");
    //     let mut file = BufReader::new(file);
    //     let mut bytes = Vec::new();
    //     file.read_to_end(&mut bytes).expect("Unable to read bytes");
    //     trace!("Loaded font from {}", path);
    //     Font {
    //         font: font::parse(&bytes).expect("Unable to parse font"),
    //     }
    // }

    pub fn advance_height(&self, scale: u32) -> f32 {
        match self.font.get_v_metrics(scale) {
            Some(vmetrics) => vmetrics.ascent - vmetrics.descent + vmetrics.line_gap,
            None => {
                trace!("Missing vmetrics");
                scale as f32
            },
        }
    }

    pub fn advance_width(&self, glyph: char, scale: u32) -> f32 {
        match self.font.lookup_glyph_id(glyph as u32) {
            Some(id) => match self.font.get_h_metrics(id, scale) {
                Some(hmetrics) => hmetrics.advance_width,
                None => {
                    trace!("Missing hmetrics: {}", glyph);
                    0.0
                },
            },
            None => {
                trace!("Missing glyph: {}", glyph);
                0.0
            },
        }
    }

    pub fn render_glyph(&self, glyph: char, scale: u32) -> Option<RenderedGlyph> {
        match self.font.lookup_glyph_id(glyph as u32) {
            Some(id) => match self.font.render_glyph(id, scale) {
                Some(bitmap) => {
                    if bitmap.width > 0 && bitmap.height > 0 {
                        let data =
                            bitmap.data.iter().map(|v| color::Color::new_raw(255, 255, 255, *v)).collect();
                        let rendered_glyph = RenderedGlyph {
                            size: Vector2::new(bitmap.width, bitmap.height),
                            topSideBearing: 0.0,
                            data: data,
                        };
                        Some(rendered_glyph)
                    } else {
                        None
                    }
                },
                None => {
                    trace!("Unable to draw: '{}'", glyph);
                    None
                },
            },
            None => {
                trace!("Missing glyph: '{}'", glyph);
                None
            },
        }
    }
}
