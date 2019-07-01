#![feature(const_fn, test, alloc_layout_extra, duration_constants)]
#![allow(dead_code, non_camel_case_types, non_snake_case, intra_doc_link_resolution_failure)]

#[macro_use]
pub extern crate log;
extern crate test;

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

use crate::render::*;
use crate::utility::bounded_spsc;
use crate::utility::bucket_spsc;
use crate::utility::control;
use cgmath::*;
use std::thread;
use std::time::Duration;

/// The main entry point into the Storm engine.
pub struct Engine {
    render_client: RenderClient,
    input_client: InputClient,
}

impl Engine {
    // Starts the engine. The game_loop parameter is called once with a valid instance of the engine
    // once the engine is constructed.
    pub fn start(desc: WindowDescription, mut game_loop: impl FnMut(Engine) + Send + 'static) {
        simple_logger::init().unwrap();

        // Init SDL2
        let sdl = unsafe { beryllium::init().expect("Unable to init beryllium (SDL).") };

        // Make a window
        let window = StormWindow::new(&desc, &sdl);

        // Inter-thread messaging.
        let (render_producer_pipe, render_consumer_pipe) = bucket_spsc::make(1);
        let (input_producer_pipe, input_consumer_pipe) = bounded_spsc::make(512);
        let (engine_watcher, engine_probe) = control::make_probe();

        thread::spawn(move || {
            let engine = Engine {
                render_client: RenderClient::new(render_producer_pipe),
                input_client: InputClient::new(input_consumer_pipe),
            };
            game_loop(engine);
            info!("Game thread has exited.");
            engine_probe.finalize();
        });

        // Rendering
        let mut render_server = RenderServer::new(window, render_consumer_pipe);
        let mut input_server = InputServer::new(
            input_producer_pipe,
            Vector2::new(
                desc.size.x as f32, // width
                desc.size.y as f32, // height
            ),
        );

        while engine_watcher.alive() {
            render_server.tick();
            input_server.tick(&sdl);
            thread::sleep(Duration::MICROSECOND);
        }
    }

    // ////////////////////////////////////////////////////////
    // Input
    // ////////////////////////////////////////////////////////

    /// Fetches all the events that are pending, calls the callback function for
    /// each of them, and returns.
    pub fn input_poll(&mut self) -> Option<InputMessage> {
        self.input_client.poll()
    }

    // ////////////////////////////////////////////////////////
    // Audio
    // ////////////////////////////////////////////////////////

    // TODO: Audio
    // pub fn audio_load(&mut self) {}

    // pub fn audio_play(&mut self) {}

    // ////////////////////////////////////////////////////////
    // Batch
    // ////////////////////////////////////////////////////////

    pub fn batch_create(&mut self, desc: &BatchDescription) -> BatchReference {
        self.render_client.batch_create(desc)
    }

    pub fn batch_update(&mut self, batch: &BatchReference, desc: &BatchDescription) {
        self.render_client.batch_update(batch, desc);
    }

    pub fn batch_remove(&mut self, batch: &BatchReference) {
        self.render_client.batch_remove(batch);
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    /// Appends a new sprite to the batch to render.
    pub fn sprite_set(&mut self, batch: &BatchReference, descs: &Vec<SpriteDescription>) {
        self.render_client.sprite_set(batch, descs);
    }

    /// Clears all sprites from the given batch.
    pub fn sprite_clear(&mut self, batch: &BatchReference) {
        self.render_client.sprite_clear(batch);
    }

    // ////////////////////////////////////////////////////////
    // String
    // ////////////////////////////////////////////////////////

    /// Loads a new font.
    pub fn font_load(&mut self, path: &str) -> FontReference {
        self.render_client.font_create(path)
    }

    /// Gets the default font.
    pub fn font_default() -> FontReference {
        DEFAULT_FONT
    }

    /// Appends a new string to the batch to render.
    pub fn string_set(&mut self, batch: &BatchReference, descs: &Vec<StringDescription>) {
        self.render_client.string_set(batch, descs);
    }

    /// Clears all strings from the given batch.
    pub fn string_clear(&mut self, batch: &BatchReference) {
        self.render_client.string_clear(batch);
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    /// Loads a new texture.
    pub fn texture_load(&mut self, path: &str) -> Texture {
        self.render_client.texture_create(path)
    }

    /// Gets the default texture.
    pub fn texture_default() -> Texture {
        DEFAULT_TEXTURE
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    /// Sets the title of the window.
    pub fn window_title(&mut self, title: &str) {
        self.render_client.window_title(title);
    }

    /// Commits the queued window related changes to the renderer. This may block
    /// if the renderer is getting changes faster than it can process.
    pub fn window_commit(&mut self) {
        self.render_client.commit();
    }
}
