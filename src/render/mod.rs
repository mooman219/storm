mod gl;

use crate::render::gl::OpenGLState;
use crate::text::*;
use crate::texture::*;
use crate::time::*;
use crate::types::*;
use crate::utility::unordered_tracker::*;
use cgmath::*;
use std::io::Read;

pub struct Renderer {
    state: OpenGLState,
    timer_render: Timer,
    atlas: TextureAtlas,
    text_cache: TextCache,
    sprite_scratch: Vec<Sprite>,
    batch_tracker: UnorderedTracker<BatchToken>,
}

impl Renderer {
    pub fn new(desc: &WindowSettings, event_loop: &glutin::event_loop::EventLoop<()>) -> Renderer {
        Renderer {
            state: OpenGLState::new(desc, event_loop),
            timer_render: Timer::new("[R] Frame"),
            atlas: TextureAtlas::new(),
            text_cache: TextCache::new(),
            sprite_scratch: Vec::new(),
            batch_tracker: UnorderedTracker::new(),
        }
    }

    pub fn current_logical_size(&self) -> Vector2<f32> {
        self.state.current_logical_size()
    }

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    pub fn batch_create(&mut self, desc: &BatchSettings) -> BatchToken {
        self.state.batch_create(desc);
        let batch_key = self.batch_tracker.add();
        BatchToken::new(batch_key)
    }

    pub fn batch_remove(&mut self, batch: &BatchToken) {
        let index = self.batch_tracker.remove(batch.key());
        self.state.batch_remove(index);
    }

    pub fn batch_update(&mut self, batch: &BatchToken, desc: &BatchSettings) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_update(index, desc);
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_set(&mut self, batch: &BatchToken, descs: &Vec<Sprite>) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_sprite_set(index, descs);
    }

    pub fn sprite_clear(&mut self, batch: &BatchToken) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_sprite_set(index, &Vec::new());
    }

    // ////////////////////////////////////////////////////////
    // String
    // ////////////////////////////////////////////////////////

    pub fn font_create(&mut self, path: &str) -> FontToken {
        FontToken::new(self.text_cache.add_font_path(path))
    }

    pub fn string_set(&mut self, batch: &BatchToken, descs: &Vec<Text>) {
        let index = self.batch_tracker.get(batch.key());
        unsafe { self.sprite_scratch.set_len(0) };
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, &mut self.sprite_scratch);
        }
        self.state.batch_string_set(index, &self.sprite_scratch)
    }

    pub fn string_clear(&mut self, batch: &BatchToken) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_string_set(index, &Vec::new());
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_create<R: Read>(&mut self, bytes: R, format: TextureFormat) -> Texture {
        let image = Image::from_raw(bytes, format);
        let uv = self.atlas.add(image);
        Texture(uv)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {
        self.state.window_title(&title);
    }

    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.state.window_display_mode(display_mode);
    }

    pub fn window_clear_color(&mut self, clear_color: RGBA8) {
        self.state.window_clear_color(clear_color);
    }

    pub fn window_vsync(&mut self, vsync: Vsync) {
        self.state.window_vsync(vsync);
    }

    pub fn draw(&mut self) {
        self.timer_render.start();
        if let Some(atlas) = self.atlas.sync() {
            self.state.upload_texture_atlas(atlas);
        }
        self.state.draw();
        self.timer_render.stop();
    }
}
