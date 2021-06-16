use crate::render::raw::{
    resource, PixelFormat, PixelInternalFormat, PixelType, TextureBindingTarget, TextureLoadTarget,
    TextureMagFilterValue, TextureMinFilterValue, TextureParameterTarget, TextureUnit, TextureWrapValue,
};
use crate::render::OpenGLState;
use crate::texture::*;

static DEFAULT: [u8; 4] = [255u8, 255u8, 255u8, 255u8];

pub struct TextureHandle {
    id: resource::Texture,
    unit: TextureUnit,
}

impl TextureHandle {
    pub fn new(texture_unit: TextureUnit) -> TextureHandle {
        let id = OpenGLState::ctx().gl.create_texture();
        let unit = texture_unit;
        let texture = TextureHandle {
            id,
            unit,
        };
        texture.set_raw(1, 1, &DEFAULT);
        texture
    }

    pub fn set_texture(&self, texture: &Image) {
        let width = texture.width() as i32;
        let height = texture.height() as i32;
        let slice = texture.as_slice();
        self.set_raw(width, height, slice);
    }

    fn set_raw<T: Sized>(&self, width: i32, height: i32, buffer: &[T]) {
        let gl = &OpenGLState::ctx().gl;
        gl.active_texture(self.unit);
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
        gl.tex_image_2d(
            TextureLoadTarget::Texture2D,
            0,
            width,
            height,
            0,
            PixelInternalFormat::RGBA,
            PixelFormat::RGBA,
            PixelType::UnsignedByte,
            buffer,
        );
        gl.tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_min_filter(TextureParameterTarget::Texture2D, TextureMinFilterValue::Nearest);
        gl.tex_parameter_mag_filter(TextureParameterTarget::Texture2D, TextureMagFilterValue::Nearest);
    }
}

impl Drop for TextureHandle {
    fn drop(&mut self) {
        OpenGLState::ctx().gl.delete_texture(self.id);
    }
}
