use crate::color::ColorDescriptor;
use alloc::{vec, vec::Vec};

/// Basic image type.
#[derive(Clone)]
pub struct Image<T: ColorDescriptor> {
    pixels: Vec<T>,
    width: u32,
    height: u32,
}

impl<T: ColorDescriptor> Image<T> {
    /// Creates an image with the given color and size.
    pub fn from_color(color: T, width: u32, height: u32) -> Image<T> {
        assert!(width > 0 && height > 0, "Neither width or height can be 0.");
        let pixels = vec![color; (width * height) as usize];
        Image {
            pixels,
            width,
            height,
        }
    }

    /// Creates an image with the given buffer and size. The buffer length must match the image
    /// dimensions.
    pub fn from_vec(buf: Vec<T>, width: u32, height: u32) -> Image<T> {
        assert!(width > 0 && height > 0, "Neither width or height can be 0.");
        assert!(buf.len() == (width * height) as usize, "Buffer length must match image dimensions.");
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

    /// The width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn as_slice(&self) -> &[T] {
        self.pixels.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.pixels.as_mut_slice()
    }

    /// Returns the underlying Vec that backs the image data.
    pub fn into_vec(self) -> Vec<T> {
        self.pixels
    }

    /// Gets the value of an individual pixel. 0 is the first pixel, and width * height is the last
    /// pixel.
    pub fn get_indexed(&self, index: usize) -> T {
        self.pixels[index]
    }

    /// Gets the value of an individual pixel. (0, 0) is the top left of the image, and
    /// (width, height) is the bottom right of the image.
    pub fn get(&self, x: u32, y: u32) -> T {
        self.get_indexed(self.index_for(x, y))
    }

    /// Sets an individual pixel to the given value. 0 is the first pixel, and width * height is the
    /// last pixel.
    pub fn set_indexed(&mut self, index: usize, val: T) {
        self.pixels[index] = val;
    }

    /// Sets an individual pixel to the given value. (0, 0) is the top left of the image, and
    /// (width, height) is the bottom right of the image.
    pub fn set(&mut self, x: u32, y: u32, val: T) {
        self.set_indexed(self.index_for(x, y), val);
    }

    /// Sets a subsection of the image to the given image. (0, 0) is the top left of the image, and
    /// (width, height) is the bottom right of the image.
    /// # Arguments
    ///
    /// * `offset_x` - The top left texel x coordinate to offset the image by.
    /// * `offset_y` - The top left texel y coordinate to offset the image by.
    /// * `tex` - The image to overwrite the current image with.
    pub fn set_subsection(&mut self, offset_x: u32, offset_y: u32, tex: &Image<T>) {
        assert!(tex.width + offset_x <= self.width && tex.height + offset_y <= self.height);
        for x in 0..tex.width {
            for y in 0..tex.height {
                let index_self = self.index_for(x + offset_x, y + offset_y);
                let index_tex = tex.index_for(x, y);
                self.pixels[index_self] = tex.pixels[index_tex];
            }
        }
    }

    /// Extends all edges of this image by `thickness`. The pixel color of the padded region is
    /// clamped to the edge of the original image.
    pub fn pad(&self, thickness: u32) -> Image<T> {
        let mut result = Image::from_color(
            T::default(), //
            self.width + thickness * 2,
            self.height + thickness * 2,
        );
        result.set_subsection(thickness, thickness, &self);

        // Top Left Corner
        let reference = self.get(0, 0);
        for x in 0..thickness {
            for y in 0..thickness {
                result.set(x, y, reference);
            }
        }
        // Top Right Corner
        let reference = self.get(self.width - 1, 0);
        let offset_x = self.width + thickness;
        for x in 0..thickness {
            for y in 0..thickness {
                result.set(x + offset_x, y, reference);
            }
        }
        // Bottom Left Corner
        let reference = self.get(0, self.height - 1);
        let offset_y = self.height + thickness;
        for x in 0..thickness {
            for y in 0..thickness {
                result.set(x, y + offset_y, reference);
            }
        }
        // Bottom Right Corner
        let reference = self.get(self.width - 1, self.height - 1);
        let offset_x = self.width + thickness;
        let offset_y = self.height + thickness;
        for x in 0..thickness {
            for y in 0..thickness {
                result.set(x + offset_x, y + offset_y, reference);
            }
        }

        // Top Side
        for x in 0..self.width {
            let reference = self.get(x, 0);
            for y in 0..thickness {
                result.set(x + thickness, y, reference);
            }
        }
        // Bottom Side
        let offset_y = self.height + thickness;
        for x in 0..self.width {
            let reference = self.get(x, self.height - 1);
            for y in 0..thickness {
                result.set(x + thickness, y + offset_y, reference);
            }
        }
        // Left Side
        for y in 0..self.height {
            let reference = self.get(0, y);
            for x in 0..thickness {
                result.set(x, y + thickness, reference);
            }
        }
        // Right Side
        let offset_x = self.width + thickness;
        for y in 0..self.height {
            let reference = self.get(self.width - 1, y);
            for x in 0..thickness {
                result.set(x + offset_x, y + thickness, reference);
            }
        }

        result
    }
}
