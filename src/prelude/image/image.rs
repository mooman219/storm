use super::png;
use crate::RGBA8;

/// Enumeration for all the loadable texture formats. Currently only PNG is supported.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ImageFormat {
    PNG,
}

#[derive(Clone)]
pub struct Image {
    pixels: Vec<RGBA8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn from_bytes(bytes: &[u8], format: ImageFormat) -> Image {
        match format {
            ImageFormat::PNG => png::read(bytes),
        }
    }

    pub fn from_color(color: RGBA8, width: u32, height: u32) -> Image {
        if width == 0 || height == 0 {
            panic!("Neither width or height can be 0.");
        }
        let pixels = vec![color; (width * height) as usize];
        Image {
            pixels,
            width,
            height,
        }
    }

    pub fn from_vec(buf: Vec<RGBA8>, width: u32, height: u32) -> Image {
        if width == 0 || height == 0 {
            panic!("Neither width or height can be 0.");
        }
        Image {
            pixels: buf,
            width,
            height,
        }
    }

    #[inline(always)]
    pub fn index_for(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn as_slice(&self) -> &[RGBA8] {
        self.pixels.as_slice()
    }

    pub fn into_vec(self) -> Vec<RGBA8> {
        self.pixels
    }

    pub fn get_indexed(&self, index: usize) -> RGBA8 {
        self.pixels[index]
    }

    pub fn get(&self, x: u32, y: u32) -> RGBA8 {
        self.get_indexed(self.index_for(x, y))
    }

    pub fn set_indexed(&mut self, index: usize, val: RGBA8) {
        self.pixels[index] = val;
    }

    pub fn set(&mut self, x: u32, y: u32, val: RGBA8) {
        self.set_indexed(self.index_for(x, y), val);
    }

    pub fn set_subsection(&mut self, offset_x: u32, offset_y: u32, tex: &Image) {
        assert!(tex.width + offset_x <= self.width && tex.height + offset_y <= self.height);
        for x in 0..tex.width {
            for y in 0..tex.height {
                let index_self = self.index_for(x + offset_x, y + offset_y);
                let index_tex = tex.index_for(x, y);
                self.pixels[index_self] = tex.pixels[index_tex];
            }
        }
    }
}
