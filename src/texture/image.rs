use crate::texture::formats;
use crate::{TextureFormat, RGBA8};
use std::io::Read;

#[derive(Clone, Debug)]
pub struct Image {
    pixels: Vec<RGBA8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn from_raw<R: Read>(bytes: R, format: TextureFormat) -> Image {
        match format {
            TextureFormat::PNG => formats::png::read(bytes),
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
    fn index_for(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn as_ptr(&self) -> *const RGBA8 {
        self.pixels.as_ptr()
    }

    pub fn get(&self, x: u32, y: u32) -> RGBA8 {
        self.pixels[self.index_for(x, y)]
    }

    pub fn set(&mut self, x: u32, y: u32, val: RGBA8) {
        let index = self.index_for(x, y);
        self.pixels[index] = val;
    }

    pub fn set_texture(&mut self, offset_x: u32, offset_y: u32, tex: &Image) {
        for x in 0..tex.width {
            for y in 0..tex.height {
                let index_self = self.index_for(x + offset_x, y + offset_y);
                let index_tex = tex.index_for(x, y);
                self.pixels[index_self] = tex.pixels[index_tex];
            }
        }
    }
}
