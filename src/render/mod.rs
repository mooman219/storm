mod buffer;
mod layer;
mod raw;
mod shader;
mod state;
mod texture_handle;
mod vertex;
mod window;

use self::raw::{OpenGL, TextureUnit};
use self::state::OpenGLState;
use self::texture_handle::*;
use self::window::*;
use crate::math::ortho_from_bounds;
use crate::text::*;
use crate::texture::*;
use crate::types::*;
use crate::utility::bad::UnsafeShared;
use cgmath::*;

pub use self::layer::Layer;
pub use self::raw::ClearMode;

pub struct Renderer {
    window: OpenGLWindow,
    state: UnsafeShared<OpenGLState>,
    texture_atlas: TextureHandle,
    matrix_bounds: Matrix4<f32>,
    logical_size: Vector2<f32>,
    atlas: TextureAtlas,
    text_cache: TextCache,
}

impl Renderer {
    pub fn new(desc: &WindowSettings, event_loop: &winit::event_loop::EventLoop<()>) -> Renderer {
        let (window, gl) = OpenGLWindow::new(desc, event_loop);

        let gl = OpenGL::new(gl);
        let state = UnsafeShared::new(OpenGLState::new(gl));

        let texture_atlas = TextureHandle::new(state.clone(), TextureUnit::Atlas);
        let logical_size = window.logical_size();

        Renderer {
            window,
            state: state,
            texture_atlas,
            matrix_bounds: ortho_from_bounds(&logical_size),
            logical_size,
            atlas: TextureAtlas::new(),
            text_cache: TextCache::new(),
        }
    }

    // ////////////////////////////////////////////////////////
    // layer
    // ////////////////////////////////////////////////////////

    pub fn layer_create(&mut self) -> Layer {
        let (a, b) = Layer::new(self.state.clone(), &self.matrix_bounds);
        self.state.layer_add(a);
        b
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
        let new_logical_size = self.window.logical_size();
        if self.logical_size != new_logical_size {
            self.logical_size = new_logical_size;
            let new_physical_size = self.window.physical_size();
            self.matrix_bounds = ortho_from_bounds(&new_logical_size);

            trace!("Window resized: Physical({:?}) Logical({:?})", new_physical_size, new_logical_size);

            self.state.resize(&new_physical_size, &self.matrix_bounds);
        }
    }

    pub fn window_swap_buffers(&self) {
        self.window.swap_buffers();
    }

    pub fn window_logical_size(&self) -> Vector2<f32> {
        self.logical_size
    }

    pub fn window_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.window.set_display_mode(display_mode);
    }

    pub fn clear_color(&mut self, clear_color: RGBA8) {
        let color: Vector4<f32> = clear_color.into();
        self.state.gl.clear_color(color.x, color.y, color.z, color.w);
    }

    pub fn clear(&mut self, clear_mode: ClearMode) {
        self.state.gl.clear(clear_mode);
    }
}
