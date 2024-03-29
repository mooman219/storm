use crate::color::ColorDescriptor;
use crate::graphics::{
    graphics, resource, TextureBindingTarget, TextureLoadTarget, TextureMagFilterValue,
    TextureMinFilterValue, TextureParameterTarget, TextureSection, TextureWrapValue,
};
use crate::image::Image;
use crate::{App, Context};
use alloc::rc::Rc;

/// Describes how a texture will be filtered. Different settings can improve texture rendering when
/// viewing textures far away, or at steep angles.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextureFiltering {
    min_filter: TextureMinFilterValue,
    mip_levels: Option<i32>,
    anisotropy: Option<f32>,
}

impl TextureFiltering {
    /// Applies no filtering to the texture.
    pub fn none() -> TextureFiltering {
        TextureFiltering {
            min_filter: TextureMinFilterValue::Nearest,
            mip_levels: None,
            anisotropy: None,
        }
    }

    /// Generates mipmaps, using the nearest mipmap to select the texture.
    /// # Arguments
    ///
    /// * `mip_levels` - The number of mip map levels to generate. If the requested level isn't
    /// available, this falls back to the max supported level.
    pub fn bilinear(mip_levels: u32) -> TextureFiltering {
        TextureFiltering {
            min_filter: TextureMinFilterValue::LinearMipmapNearest,
            mip_levels: Some(mip_levels as i32),
            anisotropy: None,
        }
    }

    /// Generates mipmaps, linearly interpolating between the mipmap to select the texture.
    /// # Arguments
    ///
    /// * `mip_levels` - The number of mip map levels to generate. If the requested level isn't
    /// available, this falls back to the max supported level.
    pub fn trilinear(mip_levels: u32) -> TextureFiltering {
        TextureFiltering {
            min_filter: TextureMinFilterValue::LinearMipmapLinear,
            mip_levels: Some(mip_levels as i32),
            anisotropy: None,
        }
    }

    /// Generates mipmaps and anisotropic mipmap levels, linearly interpolating between the mipmap
    /// to select the texture. If anisotropic mipmaps aren't available, this silently falls back to
    /// trilinear filtering. Use max_texture_anisotropy() to check if this feature is supported, as
    /// well as to get the max anisotropy.
    /// # Arguments
    ///
    /// * `mip_levels` - The number of mip map levels to generate. If the requested level isn't
    /// available, this falls back to the max supported level.
    /// * `anisotropy` - The number of anisotropy samples. This must be a power of two value. If the
    /// requested level isn't available, this silently falls back to the max supported level.
    pub fn anisotropic(mip_levels: u32, anisotropy: u32) -> TextureFiltering {
        assert!(anisotropy.is_power_of_two(), "anisotropy is not a power of two.");
        TextureFiltering {
            min_filter: TextureMinFilterValue::LinearMipmapLinear,
            mip_levels: Some(mip_levels as i32),
            anisotropy: Some(anisotropy as f32),
        }
    }

    /// Gets the requested mip levels. None if no filtering is being requested.
    pub fn mip_levels(&self) -> Option<i32> {
        self.mip_levels
    }
}

/// Represents a GPU resource for a texture.
pub struct Texture {
    id: resource::Texture,
    filter: TextureFiltering,
    width: u32,
    height: u32,
    rc: Rc<()>,
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Texture {
            id: self.id,
            filter: self.filter,
            width: self.width,
            height: self.height,
            rc: self.rc.clone(),
        }
    }
}

impl Texture {
    /// Interpret a slice of bytes as a PNG, decodes it into an RGBA image, then uploads it image to
    /// the GPU, creating a texture.
    pub fn from_png(ctx: &Context<impl App>, bytes: &[u8], filtering: TextureFiltering) -> Texture {
        Self::from_image(ctx, &Image::from_png(bytes), filtering)
    }

    /// Uploads an image to the GPU, creating a texture.
    pub fn from_image<T: ColorDescriptor>(
        ctx: &Context<impl App>,
        image: &Image<T>,
        filtering: TextureFiltering,
    ) -> Texture {
        let max_size = ctx.max_texture_size() as u32;
        if image.width() > max_size || image.height() > max_size {
            panic!(
                "The max width or height texture may have on this device is {}. \
                 The given image has a (width, height) of ({}, {})",
                max_size,
                image.width(),
                image.height()
            );
        }
        let gl = graphics().gl();
        let id = gl.create_texture();
        let texture = Texture {
            id,
            filter: filtering,
            width: image.width(),
            height: image.height(),
            rc: Rc::new(()),
        };
        let prev = gl.bind_texture(TextureBindingTarget::Texture2D, Some(id));
        gl.tex_image_2d(
            TextureLoadTarget::Texture2D,
            0,
            image.width() as i32,
            image.height() as i32,
            0,
            T::layout().gpu_format(),
            T::layout().cpu_format(),
            T::component_type().pixel_type(),
            image.as_slice(),
        );

        gl.tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_min_filter(TextureParameterTarget::Texture2D, filtering.min_filter);
        gl.tex_parameter_mag_filter(TextureParameterTarget::Texture2D, TextureMagFilterValue::Nearest);
        texture.generate_mipmap();
        gl.bind_texture(TextureBindingTarget::Texture2D, prev);
        texture
    }

    fn generate_mipmap(&self) {
        let gl = graphics().gl();
        if let Some(mip_levels) = self.filter.mip_levels {
            gl.tex_parameter_max_mipmaps(TextureParameterTarget::Texture2D, mip_levels);
            gl.generate_mipmap(TextureParameterTarget::Texture2D);
        }

        if let Some(requested_anisotropy) = self.filter.anisotropy {
            if let Some(supported_anisotropy) = graphics().max_texture_anisotropy() {
                gl.tex_parameter_anisotropy(
                    TextureParameterTarget::Texture2D,
                    supported_anisotropy.min(requested_anisotropy),
                );
            }
        }
    }

    /// The width of the texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Coordinates relative to the top left corner of the texture. (0, 0) is the top left of the
    /// texture, and (width, height) is the bottom right of the texture.
    pub fn subsection(&self, left: u32, right: u32, top: u32, bottom: u32) -> TextureSection {
        TextureSection::from_texture(&self, left, right, top, bottom)
    }

    /// Sets a subsection of the texture to the given image. (0, 0) is the top left of the texture,
    /// and (width, height) is the bottom right of the texture.
    /// # Arguments
    ///
    /// * `offset_x` - The top left texel x coordinate to offset the image by.
    /// * `offset_y` - The top left texel y coordinate to offset the image by.
    /// * `image` - The image to overwrite the texture with.
    pub fn set<Z: ColorDescriptor>(&self, offset_x: u32, offset_y: u32, image: &Image<Z>) {
        assert!(image.width() + offset_x <= self.width && image.height() + offset_y <= self.height);
        let gl = graphics().gl();
        let prev = gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
        gl.tex_sub_image_2d(
            TextureLoadTarget::Texture2D,
            0,
            offset_x as i32,
            offset_y as i32,
            image.width() as i32,
            image.height() as i32,
            Z::layout().cpu_format(),
            Z::component_type().pixel_type(),
            image.as_slice(),
        );
        self.generate_mipmap();
        gl.bind_texture(TextureBindingTarget::Texture2D, prev);
    }

    pub(crate) fn bind(&self, unit: u32) {
        let gl = graphics().gl();
        gl.active_texture(unit);
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if Rc::<()>::strong_count(&self.rc) == 1 {
            graphics().gl().delete_texture(self.id);
        }
    }
}
