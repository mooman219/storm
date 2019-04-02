use color::*;
use image::{DynamicImage, ImageBuffer, Rgba};

pub struct Texture {
    pixels: Vec<Color>,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_default(default: Color, width: u32, height: u32) -> Texture {
        if width == 0 || height == 0 {
            panic!("Neither width or height can be 0.");
        }
        let mut pixels = Vec::new();
        for _ in 0..(width * height) {
            pixels.push(default);
        }
        Texture {
            pixels: pixels,
            width: width,
            height: height,
        }
    }

    pub fn from_image(image: DynamicImage) -> Texture {
        let rgba_image = image.to_rgba();
        let width = rgba_image.width();
        let height = rgba_image.height();
        Texture::from_raw(rgba_image.into_raw().as_slice(), width, height)
    }

    pub fn from_raw(buf: &[u8], width: u32, height: u32) -> Texture {
        if width == 0 || height == 0 {
            panic!("Neither width or height can be 0.");
        }
        let mut pixels = Vec::new();
        for pixel in buf.chunks(4) {
            pixels.push(Color {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            });
        }
        Texture {
            pixels: pixels,
            width: width,
            height: height,
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

    pub fn as_ptr(&self) -> *const Color {
        self.pixels.as_ptr()
    }

    pub fn get(&self, x: u32, y: u32) -> Color {
        self.pixels[self.index_for(x, y)]
    }

    pub fn set(&mut self, x: u32, y: u32, val: Color) {
        let index = self.index_for(x, y);
        self.pixels[index] = val;
    }

    pub fn set_texture(&mut self, offset_x: u32, offset_y: u32, tex: &Texture) {
        let max = (self.width * self.height) as usize;
        for x in 0..tex.width {
            for y in 0..tex.height {
                let index_self = self.index_for(x + offset_x, y + offset_y);
                if index_self < max {
                    let index_tex = tex.index_for(x, y);
                    self.pixels[index_self] = tex.pixels[index_tex];
                }
            }
        }
    }

    pub fn to_dynamic_image(&self) -> Result<DynamicImage, &str> {
        let width = self.width;
        let height = self.height;
        let mut pixels = Vec::new();
        for pixel in &self.pixels {
            pixels.push(pixel.r);
            pixels.push(pixel.g);
            pixels.push(pixel.b);
            pixels.push(pixel.a);
        }
        match ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, pixels) {
            Some(image_buffer) => Ok(DynamicImage::ImageRgba8(image_buffer)),
            None => Err("Can't export texture"),
        }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Texture {
        Texture {
            pixels: self.pixels.clone(),
            width: self.width,
            height: self.height,
        }
    }
}
