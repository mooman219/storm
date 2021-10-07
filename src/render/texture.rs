use crate::render::raw::{
    resource, PixelFormat, PixelInternalFormat, PixelType, TextureBindingTarget, TextureLoadTarget,
    TextureMagFilterValue, TextureMinFilterValue, TextureParameterTarget, TextureUnit, TextureWrapValue,
};
use crate::render::OpenGLState;
use crate::{Image, TextureSection};
use alloc::rc::Rc;

/// Represents a GPU resource for a texture.
pub struct Texture {
    id: resource::Texture,
    width: u32,
    height: u32,
    rc: Rc<()>,
}

impl Texture {
    pub(crate) fn clone(&self) -> Texture {
        Texture {
            id: self.id,
            width: self.width,
            height: self.height,
            rc: self.rc.clone(),
        }
    }

    pub(crate) fn from_image(image: &Image) -> Texture {
        Texture::new(
            image.as_slice(),
            image.width(),
            image.height(),
            PixelFormat::RGBA,
            PixelInternalFormat::RGBA8,
        )
    }

    pub(crate) fn from_coverage(coverage: &[u8], width: u32, height: u32) -> Texture {
        Texture::new(coverage, width, height, PixelFormat::RED, PixelInternalFormat::R8)
    }

    fn new<T: Sized>(
        slice: &[T],
        width: u32,
        height: u32,
        input_format: PixelFormat,
        gpu_format: PixelInternalFormat,
    ) -> Texture {
        let gl = &mut OpenGLState::ctx().gl;
        let id = gl.create_texture();
        let texture = Texture {
            id,
            width,
            height,
            rc: Rc::new(()),
        };
        gl.active_texture(TextureUnit::Temporary);
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(id));
        gl.tex_image_2d(
            TextureLoadTarget::Texture2D,
            0,
            width as i32,
            height as i32,
            0,
            gpu_format,
            input_format,
            PixelType::UnsignedByte,
            slice,
        );
        gl.tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_min_filter(TextureParameterTarget::Texture2D, TextureMinFilterValue::Nearest);
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
    pub fn set_image(&self, offset_x: u32, offset_y: u32, image: &Image) {
        self.set_subsections(
            offset_x,
            offset_y,
            image.as_slice(),
            image.width(),
            image.height(),
            PixelFormat::RGBA,
        );
    }

    pub(crate) fn set_coverage(&self, offset_x: u32, offset_y: u32, slice: &[u8], width: u32, height: u32) {
        self.set_subsections(offset_x, offset_y, slice, width, height, PixelFormat::RED);
    }

    fn set_subsections<T: Sized>(
        &self,
        offset_x: u32,
        offset_y: u32,
        slice: &[T],
        width: u32,
        height: u32,
        format: PixelFormat,
    ) {
        assert!(width + offset_x <= self.width && height + offset_y <= self.height);
        let gl = &mut OpenGLState::ctx().gl;
        gl.active_texture(TextureUnit::Temporary);
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
        gl.tex_sub_image_2d(
            TextureLoadTarget::Texture2D,
            0,
            offset_x as i32,
            offset_y as i32,
            width as i32,
            height as i32,
            format,
            PixelType::UnsignedByte,
            slice,
        );
        gl.bind_texture(TextureBindingTarget::Texture2D, None);
    }

    pub(crate) fn bind(&self, unit: TextureUnit) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.active_texture(unit);
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if Rc::<()>::strong_count(&self.rc) == 1 {
            OpenGLState::ctx().gl.delete_texture(self.id);
        }
    }
}
