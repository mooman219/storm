pub mod color;

pub(crate) use crate::texture::font_atlas::FontAtlas;
pub(crate) use crate::texture::image::Image;
pub(crate) use crate::texture::packer::PIXEL_SIZE;
pub(crate) use crate::texture::texture_atlas::TextureAtlas;

mod font_atlas;
mod formats;
mod image;
mod packer;
mod texture_atlas;
