use crate::color::*;
use crate::texture::image::*;
use cgmath::*;
use std;
use std::cmp::max;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    #[inline(always)]
    fn top(&self) -> u32 {
        self.y
    }

    #[inline(always)]
    fn bottom(&self) -> u32 {
        self.y + self.h - 1
    }

    #[inline(always)]
    fn left(&self) -> u32 {
        self.x
    }

    #[inline(always)]
    fn right(&self) -> u32 {
        self.x + self.w - 1
    }

    #[inline(always)]
    fn contains(&self, other: &Rect) -> bool {
        self.left() <= other.left()
            && self.right() >= other.right()
            && self.top() <= other.top()
            && self.bottom() >= other.bottom()
    }

    #[inline(always)]
    fn contains_point(&self, x: u32, y: u32) -> bool {
        self.left() <= x && self.right() >= x && self.top() <= y && self.bottom() >= y
    }
}

struct Skyline {
    pub x: u32,
    pub y: u32,
    pub w: u32,
}

impl Skyline {
    #[inline(always)]
    fn left(&self) -> u32 {
        self.x
    }

    #[inline(always)]
    fn right(&self) -> u32 {
        self.x + self.w - 1
    }
}

const MAX: u32 = 65535;
const SIZE: u32 = 2048;
const FULL_PIXEL: u32 = 32;

struct SkylinePacker {
    border: Rect,
    // The skylines are sorted by their `x` position.
    skylines: Vec<Skyline>,
}

impl SkylinePacker {
    fn new() -> SkylinePacker {
        let mut skylines = Vec::new();
        skylines.push(Skyline {
            x: 0,
            y: 0,
            w: SIZE,
        });

        SkylinePacker {
            border: Rect::new(0, 0, SIZE, SIZE),
            skylines: skylines,
        }
    }

    // Return `rect` if rectangle (w, h) can fit the skyline started at `i`.
    fn can_put(&self, mut i: usize, w: u32, h: u32) -> Option<Rect> {
        let mut rect = Rect::new(self.skylines[i].x, 0, w, h);
        let mut width_left = rect.w;
        loop {
            rect.y = max(rect.y, self.skylines[i].y);
            // The source rect is too large.
            if !self.border.contains(&rect) {
                return None;
            }
            if self.skylines[i].w >= width_left {
                return Some(rect);
            }
            width_left -= self.skylines[i].w;
            i += 1;
            assert!(i < self.skylines.len());
        }
    }

    fn find_skyline(&self, w: u32, h: u32) -> Option<(usize, Rect)> {
        let mut bottom = std::u32::MAX;
        let mut width = std::u32::MAX;
        let mut index = None;
        let mut rect = Rect::new(0, 0, 0, 0);

        // Keep the `bottom` and `width` as small as possible.
        for i in 0..self.skylines.len() {
            if let Some(r) = self.can_put(i, w, h) {
                if r.bottom() < bottom || (r.bottom() == bottom && self.skylines[i].w < width) {
                    bottom = r.bottom();
                    width = self.skylines[i].w;
                    index = Some(i);
                    rect = r;
                }
            }
        }

        if let Some(index) = index {
            Some((index, rect))
        } else {
            None
        }
    }

    fn split(&mut self, index: usize, rect: &Rect) {
        let skyline = Skyline {
            x: rect.left(),
            y: rect.bottom() + 1,
            w: rect.w,
        };

        assert!(skyline.right() <= self.border.right());
        assert!(skyline.y <= self.border.bottom());

        self.skylines.insert(index, skyline);

        let i = index + 1;
        while i < self.skylines.len() {
            assert!(self.skylines[i - 1].left() <= self.skylines[i].left());

            if self.skylines[i].left() <= self.skylines[i - 1].right() {
                let shrink = self.skylines[i - 1].right() - self.skylines[i].left() + 1;
                if self.skylines[i].w <= shrink {
                    self.skylines.remove(i);
                } else {
                    self.skylines[i].x += shrink;
                    self.skylines[i].w -= shrink;
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn merge(&mut self) {
        let mut i = 1;
        while i < self.skylines.len() {
            if self.skylines[i - 1].y == self.skylines[i].y {
                self.skylines[i - 1].w += self.skylines[i].w;
                self.skylines.remove(i);
                i -= 1;
            }
            i += 1;
        }
    }

    fn pack(&mut self, width: u32, height: u32) -> Rect {
        if let Some((i, mut rect)) = self.find_skyline(width + 1, height + 1) {
            self.split(i, &rect);
            self.merge();
            rect.w -= 1;
            rect.h -= 1;
            rect
        } else {
            panic!("Unable to find space for the texture.");
        }
    }
}

pub struct TexturePacker {
    packer: SkylinePacker,
    texture: Image,
}

impl TexturePacker {
    pub fn new() -> TexturePacker {
        TexturePacker {
            packer: SkylinePacker::new(),
            texture: Image::from_default(TRANSPARENT, SIZE, SIZE),
        }
    }

    pub fn pack(&mut self, texture: &Image) -> Vector4<u16> {
        let rect = self.packer.pack(texture.width(), texture.height());
        self.texture.set_texture(rect.x, rect.y, texture);

        // UV Layout: xmin xmax ymin ymax
        let vector = Vector4::new(
            (rect.x * FULL_PIXEL) as u16,
            ((rect.x + rect.w) * FULL_PIXEL) as u16,
            (rect.y * FULL_PIXEL) as u16,
            ((rect.y + rect.h) * FULL_PIXEL) as u16,
        );
        // trace!("Packing rect: {:?} -> {:?}", rect, vector); // DEBUG
        vector
    }

    pub fn pack_path(&mut self, path: &Path) -> Vector4<u16> {
        let texture = match image::open(path) {
            Ok(img) => img,
            Err(msg) => panic!("Unable to open image: {}", msg),
        };
        let texture = Image::from_image(texture);
        self.pack(&texture)
    }

    pub fn export(&self) -> Image {
        self.texture.clone()
    }
}
