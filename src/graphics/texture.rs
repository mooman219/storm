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
    anisotropy: Option<f32>,
}

impl TextureFiltering {
    /// Applies no filtering to the texture.
    pub const NONE: TextureFiltering = TextureFiltering {
        min_filter: TextureMinFilterValue::Nearest,
        anisotropy: None,
    };
    /// Generates mipmaps, using the nearest mipmap to select the texture.
    pub const BILINEAR: TextureFiltering = TextureFiltering {
        min_filter: TextureMinFilterValue::LinearMipmapNearest,
        anisotropy: None,
    };
    /// Generates mipmaps, linearly interpolating between the mipmap to select the texture.
    pub const TRILINEAR: TextureFiltering = TextureFiltering {
        min_filter: TextureMinFilterValue::LinearMipmapLinear,
        anisotropy: None,
    };
    /// Generates mipmaps and 2 anisotropic mipmap levels, linearly interpolating between the mipmap
    /// to select the texture. If anisotropic mipmaps aren't available, this silently falls back to
    /// trilinear filtering. Use max_texture_anisotropy() to check if this feature is supported.
    pub const ANISOTROPIC2X: TextureFiltering = TextureFiltering {
        min_filter: TextureMinFilterValue::LinearMipmapLinear,
        anisotropy: Some(2.0),
    };
    /// Generates mipmaps and 4 anisotropic mipmap levels, linearly interpolating between the mipmap
    /// to select the texture. If anisotropic mipmaps aren't available, this silently falls back to
    /// trilinear filtering. Use max_texture_anisotropy() to check if this feature is supported.
    pub const ANISOTROPIC4X: TextureFiltering = TextureFiltering {
        min_filter: TextureMinFilterValue::LinearMipmapLinear,
        anisotropy: Some(4.0),
    };
    /// Generates mipmaps and 8 anisotropic mipmap levels, linearly interpolating between the mipmap
    /// to select the texture. If anisotropic mipmaps aren't available, this silently falls back to
    /// trilinear filtering. Use max_texture_anisotropy() to check if this feature is supported.
    pub const ANISOTROPIC8X: TextureFiltering = TextureFiltering {
        min_filter: TextureMinFilterValue::LinearMipmapLinear,
        anisotropy: Some(8.0),
    };
    /// Generates mipmaps and 16 anisotropic mipmap levels, linearly interpolating between the
    /// mipmap to select the texture. If anisotropic mipmaps aren't available, this silently falls
    /// back to trilinear filtering. Use max_texture_anisotropy() to check if this feature is
    /// supported.
    pub const ANISOTROPIC16X: TextureFiltering = TextureFiltering {
        min_filter: TextureMinFilterValue::LinearMipmapLinear,
        anisotropy: Some(16.0),
    };
}

/// Represents a GPU resource for a texture.
pub struct Texture {
    id: resource::Texture,
    width: u32,
    height: u32,
    rc: Rc<()>,
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Texture {
            id: self.id,
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
            width: image.width(),
            height: image.height(),
            rc: Rc::new(()),
        };
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(id));
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

        if filtering != TextureFiltering::NONE {
            gl.generate_mipmap(TextureParameterTarget::Texture2D);
        }

        if let Some(requested_anisotropy) = filtering.anisotropy {
            if let Some(supported_anisotropy) = ctx.max_texture_anisotropy() {
                gl.tex_parameter_anisotropy(
                    TextureParameterTarget::Texture2D,
                    supported_anisotropy.min(requested_anisotropy),
                );
            }
        }

        gl.tex_parameter_max_mipmaps(TextureParameterTarget::Texture2D, 4);
        gl.tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_min_filter(TextureParameterTarget::Texture2D, filtering.min_filter);
        gl.tex_parameter_mag_filter(TextureParameterTarget::Texture2D, TextureMagFilterValue::Nearest);
        gl.bind_texture(TextureBindingTarget::Texture2D, None);
        texture
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
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
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
        gl.bind_texture(TextureBindingTarget::Texture2D, None);
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
