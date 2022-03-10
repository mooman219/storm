use crate::color::ColorDescriptor;
use crate::graphics::{Texture, TextureFiltering, TextureSection};
use crate::image::{Image, Packer};
use crate::{App, Context};

/// Simple image atlas that adds padding to reduce mip map artifacts. Extra padding is added to
/// packed images based on the number of mip levels. More mip levels means more space dedicated to
/// padding.
pub struct TextureAtlas {
    atlas: Texture,
    packer: Packer,
    padding: Option<u32>,
}

impl TextureAtlas {
    /// Creates a new atlas.
    /// # Arguments
    ///
    /// * `size` - The width and height of the atlas. This must be a power of two value.
    /// * `filtering` - The filtering to apply. If any filtering is used, this atlas will add
    /// padding to each texture packed to offset mip map artifacts.
    pub fn new<T: ColorDescriptor, A: App>(
        ctx: &Context<A>,
        size: u32,
        filtering: TextureFiltering,
    ) -> TextureAtlas {
        assert!(size.is_power_of_two(), "size is not a power of two.");

        let atlas = Texture::from_image(ctx, &Image::from_color(T::default(), size, size), filtering);
        let packer = Packer::new(size, size);
        let padding = if let Some(mip_levels) = filtering.mip_levels() {
            Some(2u32.pow(mip_levels as u32))
        } else {
            None
        };

        TextureAtlas {
            atlas,
            packer,
            padding,
        }
    }

    /// Packs an image into the texture atlas, returning a texture section for where the image was
    /// added. Returns None if the image could not be fit in the atlas.
    pub fn pack<T: ColorDescriptor>(&mut self, image: &Image<T>) -> Option<TextureSection> {
        if let Some(padding) = self.padding {
            let image = image.pad(padding);
            let rect = self.packer.pack(image.width(), image.height());
            if let Some(rect) = rect {
                self.atlas.set(rect.x, rect.y, &image);
                return Some(self.atlas.subsection(
                    rect.x + padding,
                    rect.x + rect.w - padding,
                    rect.y + padding,
                    rect.y + rect.h - padding,
                ));
            }
        } else {
            let rect = self.packer.pack(image.width(), image.height());
            if let Some(rect) = rect {
                self.atlas.set(rect.x, rect.y, image);
                return Some(self.atlas.subsection(rect.x, rect.x + rect.w, rect.y, rect.y + rect.h));
            }
        }
        None
    }

    /// Gets a reference to the underlying texture.
    pub fn get(&self) -> &Texture {
        &self.atlas
    }
}
