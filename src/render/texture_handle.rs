use crate::render::raw::{
    resource, OpenGL, PixelFormat, PixelInternalFormat, PixelType, TextureBindingTarget, TextureLoadTarget,
    TextureMagFilterValue, TextureMinFilterValue, TextureParameterTarget, TextureUnit, TextureWrapValue,
};
use crate::texture::*;

static DEFAULT: [u8; 4] = [255u8, 255u8, 255u8, 255u8];

pub struct TextureHandle {
    gl: OpenGL,
    id: resource::Texture,
    unit: TextureUnit,
}

impl TextureHandle {
    pub fn new(gl: OpenGL, texture_unit: TextureUnit) -> TextureHandle {
        let id = gl.create_texture();
        let unit = texture_unit;
        let texture = TextureHandle {
            gl,
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
        self.gl.active_texture(self.unit);
        self.gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
        self.gl.tex_image_2d(
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
        self.gl.tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        self.gl.tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        self.gl.tex_parameter_min_filter(TextureParameterTarget::Texture2D, TextureMinFilterValue::Nearest);
        self.gl.tex_parameter_mag_filter(TextureParameterTarget::Texture2D, TextureMagFilterValue::Nearest);
    }
}

impl Drop for TextureHandle {
    fn drop(&mut self) {
        self.gl.delete_texture(self.id);
    }
}
