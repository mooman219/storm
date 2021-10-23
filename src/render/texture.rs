use crate::render::raw::{
    resource, TextureBindingTarget, TextureLoadTarget, TextureMagFilterValue, TextureMinFilterValue,
    TextureParameterTarget, TextureUnit, TextureWrapValue,
};
use crate::render::{ColorDescription, OpenGLState};
use crate::{Image, TextureSection};
use alloc::rc::Rc;
use core::marker::PhantomData;

/// Represents a GPU resource for a texture.
pub struct Texture<T: ColorDescription> {
    id: resource::Texture,
    width: u32,
    height: u32,
    rc: Rc<()>,
    _pixel: PhantomData<T>,
}

impl<T: ColorDescription> Texture<T> {
    pub(crate) fn clone(&self) -> Texture<T> {
        Texture {
            id: self.id,
            width: self.width,
            height: self.height,
            rc: self.rc.clone(),
            _pixel: PhantomData,
        }
    }

    /// Uploads an image to the GPU, creating a texture.
    pub(crate) fn from_image(image: &Image<T>) -> Texture<T> {
        let gl = &mut OpenGLState::ctx().gl;
        let id = gl.create_texture();
        let texture = Texture {
            id,
            width: image.width(),
            height: image.height(),
            rc: Rc::new(()),
            _pixel: PhantomData,
        };
        gl.active_texture(TextureUnit::Temporary);
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
    pub fn set<Z: ColorDescription>(&self, offset_x: u32, offset_y: u32, image: &Image<Z>) {
        assert!(image.width() + offset_x <= self.width && image.height() + offset_y <= self.height);
        let gl = &mut OpenGLState::ctx().gl;
        gl.active_texture(TextureUnit::Temporary);
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

    pub(crate) fn bind(&self, unit: TextureUnit) {
        let gl = &mut OpenGLState::ctx().gl;
        gl.active_texture(unit);
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
    }
}

impl<T: ColorDescription> Drop for Texture<T> {
    fn drop(&mut self) {
        if Rc::<()>::strong_count(&self.rc) == 1 {
            OpenGLState::ctx().gl.delete_texture(self.id);
        }
    }
}
