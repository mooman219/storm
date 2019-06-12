use crate::render::gl::raw::*;
use crate::texture::*;

static DEFAULT: [u8; 4] = [255u8, 255u8, 255u8, 255u8];

pub struct TextureHandle {
    id: u32,
    unit: TextureUnit,
}

impl TextureHandle {
    pub fn new(texture_unit: TextureUnit) -> TextureHandle {
        let id = gen_texture();
        let unit = texture_unit;
        let texture = TextureHandle {
            id: id,
            unit: unit,
        };
        texture.set_raw(1, 1, (&DEFAULT).as_ptr());
        texture
    }

    pub fn set_texture(&self, texture: &Image) {
        self.set_raw(texture.width() as i32, texture.height() as i32, texture.as_ptr() as *const u8);
    }

    fn set_raw(&self, width: i32, height: i32, buffer: *const u8) {
        active_texture(self.unit);
        bind_texture(TextureBindingTarget::Texture2D, self.id);
        tex_image_2D(
            TextureLoadTarget::Texture2D,
            0,
            width,
            height,
            PixelInternalFormat::RGBA,
            PixelFormat::RGBA,
            PixelType::UnsignedByte,
            buffer as *const _,
        );
        tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        tex_parameter_min_filter(TextureParameterTarget::Texture2D, TextureMinFilterValue::Nearest);
        tex_parameter_mag_filter(TextureParameterTarget::Texture2D, TextureMagFilterValue::Nearest);
    }
}

impl Drop for TextureHandle {
    fn drop(&mut self) {
        delete_texture(self.id);
    }
}
