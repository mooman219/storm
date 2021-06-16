mod buffer;
mod layer;
mod raw;
mod shader;
mod state;
mod texture_handle;
mod vertex;
mod window;

use self::raw::TextureUnit;
use self::state::OpenGLState;
use self::texture_handle::*;
use self::window::*;
use crate::text::*;
use crate::texture::*;
use crate::types::*;
use cgmath::*;

pub use self::layer::SpriteLayer;

pub struct Renderer {
    window: OpenGLWindow,
    texture_atlas: TextureHandle,
    atlas: TextureAtlas,
    text_cache: TextCache,
}

impl Renderer {
    pub fn new(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> Renderer {
        let window = OpenGLState::init(desc, event_loop);

        Renderer {
            window,
            texture_atlas: TextureHandle::new(TextureUnit::Atlas),
            atlas: TextureAtlas::new(),
            text_cache: TextCache::new(),
        }
    }

    // ////////////////////////////////////////////////////////
    // layer
    // ////////////////////////////////////////////////////////

    pub fn layer_sprite(&mut self) -> SpriteLayer {
        SpriteLayer::new()
    }

    // ////////////////////////////////////////////////////////
    // String
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, bytes: &[u8]) -> FontToken {
        FontToken::new(self.text_cache.add_font_bytes(bytes))
    }

    pub fn text_append(&mut self, descs: &Vec<Text>, output: &mut Vec<Sprite>) {
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, output);
        }
        self.texture_sync();
    }

    pub fn text_clear(&mut self, descs: &Vec<Text>, output: &mut Vec<Sprite>) {
        unsafe { output.set_len(0) };
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, output);
        }
        self.texture_sync();
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, bytes: &[u8], format: TextureFormat) -> Texture {
        let image = Image::from_raw(bytes, format);
        let uv = self.atlas.add(image);
        self.texture_sync();
        Texture(uv)
    }

    pub fn texture_sync(&mut self) {
        if let Some(atlas) = self.atlas.sync() {
            self.texture_atlas.set_texture(atlas);
        }
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_check_resize(&mut self) {
        OpenGLState::ctx().resize(self.window.physical_size(), self.window.logical_size());
    }

    pub fn window_swap_buffers(&self) {
        self.window.swap_buffers();
    }

    pub fn window_logical_size(&self) -> Vector2<f32> {
        self.window.logical_size()
    }

    pub fn window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.window.set_display_mode(display_mode);
    }
}
