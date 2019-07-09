use crate::color::*;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Image {
    pixels: Vec<RGBA8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn from_color(color: RGBA8, width: u32, height: u32) -> Image {
        if width == 0 || height == 0 {
            panic!("Neither width or height can be 0.");
        }
        let mut pixels = Vec::new();
        for _ in 0..(width * height) {
            pixels.push(color);
        }
        Image {
            pixels: pixels,
            width: width,
            height: height,
        }
    }

    pub fn from_path(path: &Path) -> Image {
        let image = match image::open(&Path::new(path)) {
            Ok(img) => img,
            Err(msg) => panic!("Unable to open image: {}", msg),
        };
        Image::from_image(image)
    }

    pub fn from_image(image: DynamicImage) -> Image {
        let rgba_image = image.to_rgba();
        let width = rgba_image.width();
        let height = rgba_image.height();
        if width == 0 || height == 0 {
            panic!("Neither width or height can be 0.");
        }
        let mut pixels = Vec::new();
        for pixel in rgba_image.into_raw().as_slice().chunks_exact(4) {
            pixels.push(RGBA8::new_raw(pixel[0], pixel[1], pixel[2], pixel[3]));
        }
        Image {
            pixels: pixels,
            width: width,
            height: height,
        }
    }

    pub fn from_color_vec(buf: &Vec<RGBA8>, width: u32, height: u32) -> Image {
        if width == 0 || height == 0 {
            panic!("Neither width or height can be 0.");
        }
        Image {
            pixels: buf.clone(),
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

    pub fn save_png(&self, path: &str) {
        let width = self.width;
        let height = self.height;
        let mut pixels = Vec::new();
        for pixel in &self.pixels {
            pixels.push(pixel.r);
            pixels.push(pixel.g);
            pixels.push(pixel.b);
            pixels.push(pixel.a);
        }
        let image = match ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, pixels) {
            Some(image_buffer) => DynamicImage::ImageRgba8(image_buffer),
            None => panic!("Can't export texture"),
        };
        image.save(path).unwrap();
    }
}
