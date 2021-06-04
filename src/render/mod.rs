mod batch;
mod buffer;
mod raw;
mod shader;
mod state;
mod texture_handle;
mod vertex;
mod window;

use self::state::OpenGLState;
use crate::text::*;
use crate::texture::*;
use crate::types::*;
use cgmath::*;

pub use self::batch::Batch;

pub struct Renderer {
    state: OpenGLState,
    atlas: TextureAtlas,
    text_cache: TextCache,
}

impl Renderer {
    pub(crate) fn new(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> Renderer {
        Renderer {
            state: OpenGLState::new(desc, event_loop),
            atlas: TextureAtlas::new(),
            text_cache: TextCache::new(),
        }
    }

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    pub fn batch_create(&mut self) -> Batch {
        self.state.batch_create()
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
    }

    pub fn text_clear(&mut self, descs: &Vec<Text>, output: &mut Vec<Sprite>) {
        unsafe { output.set_len(0) };
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, output);
        }
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create(&mut self, bytes: &[u8], format: TextureFormat) -> Texture {
        let image = Image::from_raw(bytes, format);
        let uv = self.atlas.add(image);
        Texture(uv)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_check_resize(&mut self) {
        self.state.window_check_resize();
    }

    pub fn window_logical_size(&self) -> Vector2<f32> {
        self.state.logical_size()
    }

    pub fn window_title(&mut self, title: &str) {
        self.state.window_title(&title);
    }

    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.state.window_display_mode(display_mode);
    }

    pub fn clear_color(&mut self, clear_color: RGBA8) {
        self.state.clear_color(clear_color);
    }

    pub fn draw(&mut self) {
        if let Some(atlas) = self.atlas.sync() {
            self.state.upload_texture_atlas(atlas);
        }
        self.state.draw();
    }
}
