use image::DynamicImage;
use render::raw::*;

pub struct TextureGl {
    id: u32,
    unit: TextureUnit,
}

impl TextureGl {
    pub fn new(texture_unit: TextureUnit) -> TextureGl {
        let id = gen_texture();
        let unit = texture_unit;
        let texture = TextureGl { id: id, unit: unit };
        texture.set_raw(1, 1, [255u8, 255u8, 255u8, 255u8].to_vec());
        texture
    }

    pub fn set_image(&self, image: DynamicImage) {
        // Some Image -> RGBA Image
        let rgba_image = image.to_rgba();
        let width = rgba_image.width() as i32;
        let height = rgba_image.height() as i32;
        // RGBA Image -> Vec<u8> -> *const u8
        self.set_raw(width, height, rgba_image.into_raw());
    }

    pub fn set_raw(&self, width: i32, height: i32, buffer: Vec<u8>) {
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
            buffer.as_ptr() as *const _,
        );
        tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        tex_parameter_min_filter(TextureParameterTarget::Texture2D, TextureMinFilterValue::Nearest);
        tex_parameter_mag_filter(TextureParameterTarget::Texture2D, TextureMagFilterValue::Nearest);
    }
}

impl Drop for TextureGl {
    fn drop(&mut self) {
        delete_texture(self.id);
    }
}
