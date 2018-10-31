mod rect;
mod rgba8;
mod texture;
mod texture_gl;
mod texture_packer;

pub use render::texture::rect::*;
pub use render::texture::texture::*;
pub use render::texture::texture_gl::*;
pub use render::texture::texture_packer::*;

use render::texture::rgba8::*;

use image;
use std::path::Path;

pub trait Packer {
    fn pack(&mut self, texture: &Texture) -> Option<Rect>;

    fn pack_path(&mut self, path: &Path) -> Option<Rect> {
        let texture = image::open(path).unwrap().to_rgba();
        let width = texture.width();
        let height = texture.height();
        let texture = Texture::from_raw(texture.into_raw().as_slice(), width, height);
        self.pack(&texture)
    }

    fn export(&self) -> Texture;
}
