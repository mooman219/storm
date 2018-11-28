mod texture;
mod texture_handle;
mod texture_packer;

pub use render::texture::texture::*;
pub use render::texture::texture_handle::*;
pub use render::texture::texture_packer::*;

use cgmath::*;
use image;
use std::path::Path;

pub trait Packer {
    fn pack(&mut self, texture: &Texture) -> Vector4<f32>;

    fn pack_path(&mut self, path: &Path) -> Vector4<f32> {
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
