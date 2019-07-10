#![allow(dead_code, non_camel_case_types, non_snake_case, intra_doc_link_resolution_failure)]

#[macro_use]
pub extern crate log;

pub use cgmath;

pub mod math;
pub mod time;

pub use crate::input::*;
pub use crate::texture::*;
pub use crate::types::*;

mod font;
mod input;
mod render;
mod texture;
mod types;
mod utility;

use crate::color::RGBA8;
use crate::render::{RenderClient, RenderServer};
use crate::utility::{bounded_spsc, control, swap_spsc};
use cgmath::*;
use std::path::Path;
use std::thread;

/// The main entry point into the Storm engine. All interactions with the engine are managed by the
/// API on this type. The engine is send, and can be moved between threads.
pub struct Engine {
    render_client: RenderClient,
    input_client: InputClient,
}

impl Engine {
    // Starts the engine. The game_loop parameter is called once with a valid instance of the engine
    // once the engine is constructed. If the game loop exits or panics, the engine shuts down.
    pub fn start(desc: WindowSettings, mut game_loop: impl FnMut(Engine) + Send + 'static) {
        simple_logger::init().unwrap();
        info!("Engine started.");

        // Init SDL2
        let sdl = unsafe { beryllium::init().expect("Unable to init beryllium (SDL).") };

        // Inter-thread messaging.
        let (render_producer_pipe, render_consumer_pipe) = swap_spsc::make();
        let (input_producer_pipe, input_consumer_pipe) = bounded_spsc::make(512);
        let (engine_watcher, engine_probe) = control::make_probe();

        // Rendering and input
        let mut render_server = RenderServer::new(&desc, &sdl, render_consumer_pipe);
        let mut input_server = InputServer::new(
            input_producer_pipe,
            Vector2::new(
                desc.size.x as f32, // width
                desc.size.y as f32, // height
            ),
        );

        thread::spawn(move || {
            let engine = Engine {
                render_client: RenderClient::new(render_producer_pipe),
                input_client: InputClient::new(input_consumer_pipe),
            };
            info!("Game started.");
            game_loop(engine);
            info!("Game exited.");
            engine_probe.finalize();
        });

        while engine_watcher.alive() {
            render_server.tick();
            input_server.tick(&sdl);
            thread::sleep(time::MICROSECOND);
        }
    }

    // ////////////////////////////////////////////////////////
    // Input
    // ////////////////////////////////////////////////////////

    /// Polls for an input message. If there are no buffered input messages, then this returns None.
    pub fn input_poll(&mut self) -> Option<InputMessage> {
        self.input_client.poll()
    }

    // ////////////////////////////////////////////////////////
    // Audio
    // ////////////////////////////////////////////////////////

    // TODO: Audio

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    /// Creates a new batch with the given settings and returns a token to reference the batch by
    /// later. The returned token can be freely copied.
    pub fn batch_create(&mut self, desc: &BatchSettings) -> BatchToken {
        self.render_client.batch_create(desc)
    }

    /// Updates the settings for an existing batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn batch_update(&mut self, batch: &BatchToken, desc: &BatchSettings) {
        self.render_client.batch_update(batch, desc);
    }

    /// Removes an existing batch from the engine. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn batch_remove(&mut self, batch: &BatchToken) {
        self.render_client.batch_remove(batch);
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    /// Sets the sprites to render for a given batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn sprite_set(&mut self, batch: &BatchToken, descs: &Vec<Sprite>) {
        self.render_client.sprite_set(batch, descs);
    }

    /// Clears all sprites from the given batch. This does the same thing as passing an empty Vec to
    /// sprite_set. If the token references an invalid or removed batch, this will panic.
    pub fn sprite_clear(&mut self, batch: &BatchToken) {
        self.render_client.sprite_clear(batch);
    }

    // ////////////////////////////////////////////////////////
    // Text
    // ////////////////////////////////////////////////////////

    /// Loads a new font and returns a token to reference it with later.
    pub fn font_load(&mut self, path: &str) -> FontToken {
        self.render_client.font_create(path)
    }

    // TODO: Alternative font loading functions.

    /// Sets the text to render for a given batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn text_set(&mut self, batch: &BatchToken, descs: &Vec<Text>) {
        self.render_client.string_set(batch, descs);
    }

    /// Clears all text from the given batch. This does the same thing as passing an empty Vec to
    /// sprite_set. If the token references an invalid or removed batch, this will panic.
    pub fn text_clear(&mut self, batch: &BatchToken) {
        self.render_client.string_clear(batch);
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    // TODO: Non panicing API for texture loading.

    /// Loads a new texture. If there is an issue loading the texture, this function will panic.
    pub fn texture_load(&mut self, path: &str) -> Texture {
        self.render_client.texture_create(&Path::new(path))
    }

    /// Loads a new texture. If there is an issue loading the texture, this function will panic.
    pub fn texture_load_path(&mut self, path: &Path) -> Texture {
        self.render_client.texture_create(path)
    }

    // TODO: Alternative texture loading functions.

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    /// Sets the title of the window.
    pub fn window_title(&mut self, title: &str) {
        self.render_client.window_title(title);
    }

    /// Sets the clear color for the window.
    pub fn window_clear_color(&mut self, clear_color: RGBA8) {
        self.render_client.window_clear_color(clear_color);
    }

    /// Commits the queued window, batch, sprite, text, and texture related changes to the renderer.
    /// This function will not block.
    pub fn window_commit(&mut self) {
        self.render_client.commit();
    }
}
