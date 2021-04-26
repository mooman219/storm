mod gl;

use crate::render::gl::OpenGLState;
use crate::text::*;
use crate::texture::*;
use crate::time::*;
use crate::types::*;
use crate::utility::unordered_tracker::*;
use cgmath::*;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub struct Renderer {
    state: OpenGLState,
    timer_render: Timer,
    atlas: TextureAtlas,
    text_cache: TextCache,
    sprite_scratch: Vec<Sprite>,
    batch_tracker: UnorderedTracker<BatchToken>,
}

impl Renderer {
    pub(crate) fn new(desc: &WindowSettings, event_loop: &glutin::event_loop::EventLoop<()>) -> Renderer {
        Renderer {
            state: OpenGLState::new(desc, event_loop),
            timer_render: Timer::new("[R] Frame"),
            atlas: TextureAtlas::new(),
            text_cache: TextCache::new(),
            sprite_scratch: Vec::new(),
            batch_tracker: UnorderedTracker::new(),
        }
    }

    /// The current logical size of the surface.
    pub fn current_logical_size(&self) -> Vector2<f32> {
        self.state.current_logical_size()
    }

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    /// Creates a new batch with the given settings and returns a token to reference the batch by
    /// later. The returned token can be freely copied.
    pub fn batch_create(&mut self, desc: &BatchSettings) -> BatchToken {
        self.state.batch_create(desc);
        let batch_key = self.batch_tracker.add();
        BatchToken::new(batch_key)
    }

    /// Removes an existing batch from the engine. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn batch_remove(&mut self, batch: &BatchToken) {
        let index = self.batch_tracker.remove(batch.key());
        self.state.batch_remove(index);
    }

    /// Updates the settings for an existing batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn batch_update(&mut self, batch: &BatchToken, desc: &BatchSettings) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_update(index, desc);
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    /// Sets the sprites to render for a given batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn sprite_set(&mut self, batch: &BatchToken, descs: &Vec<Sprite>) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_sprite_set(index, descs);
    }

    /// Clears all sprites from the given batch. This does the same thing as passing an empty Vec to
    /// sprite_set. If the token references an invalid or removed batch, this will panic.
    pub fn sprite_clear(&mut self, batch: &BatchToken) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_sprite_set(index, &Vec::new());
    }

    // ////////////////////////////////////////////////////////
    // String
    // ////////////////////////////////////////////////////////

    /// Loads a new font and returns a token to reference it with later.
    pub fn font_load(&mut self, path: &str) -> FontToken {
        FontToken::new(self.text_cache.add_font_path(path))
    }

    /// Sets the text to render for a given batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn text_set(&mut self, batch: &BatchToken, descs: &Vec<Text>) {
        let index = self.batch_tracker.get(batch.key());
        unsafe { self.sprite_scratch.set_len(0) };
        for desc in descs {
            self.text_cache.rasterize(&mut self.atlas, desc, &mut self.sprite_scratch);
        }
        self.state.batch_string_set(index, &self.sprite_scratch)
    }

    /// Clears all text from the given batch. This does the same thing as passing an empty Vec to
    /// sprite_set. If the token references an invalid or removed batch, this will panic.
    pub fn text_clear(&mut self, batch: &BatchToken) {
        let index = self.batch_tracker.get(batch.key());
        self.state.batch_string_set(index, &Vec::new());
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    /// Loads a new texture from a given path. If there is an issue loading the texture, this
    /// function will panic.
    pub fn texture_load<P: AsRef<Path>>(
        &mut self,
        path: P,
        format: TextureFormat,
    ) -> Result<Texture, &'static str> {
        if let Ok(f) = File::open(path) {
            let reader = BufReader::new(f);
            Ok(self.texture_create(reader, format))
        } else {
            Err("Unable to open file to read path.")
        }
    }

    /// Loads a new texture from a given path. If there is an issue loading the texture, this
    /// function will panic.
    pub fn texture_create<R: Read>(&mut self, bytes: R, format: TextureFormat) -> Texture {
        let image = Image::from_raw(bytes, format);
        let uv = self.atlas.add(image);
        Texture(uv)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    /// Sets the title of the window.
    pub fn window_title(&mut self, title: &str) {
        self.state.window_title(&title);
    }

    /// Sets the display mode of the window.
    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.state.window_display_mode(display_mode);
    }

    /// Sets the vsync setting for the window.
    pub fn window_vsync(&mut self, vsync: Vsync) {
        self.state.window_vsync(vsync);
    }

    /// Sets the clear color for the window.
    pub fn clear_color(&mut self, clear_color: RGBA8) {
        self.state.clear_color(clear_color);
    }

    /// Draws the current renderer state to the screen.
    pub fn draw(&mut self) {
        self.timer_render.start();
        if let Some(atlas) = self.atlas.sync() {
            self.state.upload_texture_atlas(atlas);
        }
        self.state.draw();
        self.timer_render.stop();
    }
}
