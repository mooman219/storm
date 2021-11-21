use crate::ColorDescription;

/// Basic image type.
#[derive(Clone)]
pub struct Image<T: ColorDescription> {
    pixels: Vec<T>,
    width: u32,
    height: u32,
}

impl<T: ColorDescription> Image<T> {
    pub fn from_color(color: T, width: u32, height: u32) -> Image<T> {
        assert!(width > 0 && height > 0, "Neither width or height can be 0.");
        let pixels = vec![color; (width * height) as usize];
        Image {
            pixels,
            width,
            height,
        }
    }

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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn as_slice(&self) -> &[T] {
        self.pixels.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.pixels.as_mut_slice()
    }

    pub fn into_vec(self) -> Vec<T> {
        self.pixels
    }

    pub fn get_indexed(&self, index: usize) -> T {
        self.pixels[index]
    }

    pub fn get(&self, x: u32, y: u32) -> T {
        self.get_indexed(self.index_for(x, y))
    }

    pub fn set_indexed(&mut self, index: usize, val: T) {
        self.pixels[index] = val;
    }

    pub fn set(&mut self, x: u32, y: u32, val: T) {
        self.set_indexed(self.index_for(x, y), val);
    }

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
}