mod rect;
mod texture;
mod texture_handle;
mod texture_packer;

pub use render::texture::rect::*;
pub use render::texture::texture::*;
pub use render::texture::texture_handle::*;
pub use render::texture::texture_packer::*;

use image;
use std::path::Path;

pub trait Packer {
    fn pack(&mut self, texture: &Texture) -> Rect;

    fn pack_path(&mut self, path: &Path) -> Rect {
        let texture = match image::open(path) {
            Ok(img) => img.to_rgba(),
            Err(msg) => panic!("Unable to open image: {}", msg),
        };
        let width = texture.width();
        let height = texture.height();
        let texture = Texture::from_raw(texture.into_raw().as_slice(), width, height);
        self.pack(&texture)
    }

    fn export(&self) -> Texture;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
