#![feature(const_fn, asm, test, alloc_layout_extra)]
#![allow(dead_code, non_camel_case_types, non_snake_case, intra_doc_link_resolution_failure)]

extern crate core;
extern crate gl;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate log;
extern crate hashbrown;
extern crate parking_lot;
extern crate rusttype;
extern crate test;
extern crate unicode_normalization;

pub extern crate cgmath;

pub mod math;
pub mod time;

pub use input::*;
pub use texture::*;
pub use types::*;

mod font;
mod input;
mod render;
mod texture;
mod types;
mod utility;

use glutin::dpi::*;
use render::*;
use std::thread;
use utility::bucket_spsc;

/// The main entry point into the Storm engine.
pub struct Engine {
    render_client: RenderClient,
    input_manager: InputManager,
}

impl Engine {
    // TODO: Allow for assigning window setting on initial creation
    /// Creates and runs an instance of the engine. This creates a window on
    /// another thread which listens for messages from the engine.
    pub fn new(desc: WindowDescription) -> Engine {
        // Inter-thread messaging.
        let (render_producer_pipe, render_consumer_pipe) = bucket_spsc::make(1);

        // Winow creation
        let event_loop = glutin::EventsLoop::new();
        let window = Window::new(
            glutin::ContextBuilder::new()
                .build_windowed(
                    glutin::WindowBuilder::new()
                        .with_title(desc.title)
                        .with_dimensions(LogicalSize::from((desc.size.x, desc.size.y)))
                        .with_resizable(desc.resizable),
                    &event_loop,
                )
                .expect("Unable to build the window."),
        );

        // Rendering
        thread::spawn(move || {
            render::start(window, render_consumer_pipe);
        });

        Engine {
            render_client: RenderClient::new(render_producer_pipe),
            input_manager: InputManager::new(event_loop),
        }
    }

    // ////////////////////////////////////////////////////////
    // Engine
    // ////////////////////////////////////////////////////////

    // TODO: Engine inspection
    // pub fn engine_render_alive() -> bool {
    //     false
    // }

    // pub fn engine_shutdow(&mut self) {}

    // ////////////////////////////////////////////////////////
    // Input
    // ////////////////////////////////////////////////////////

    /// Fetches all the events that are pending, calls the callback function for
    /// each of them, and returns.
    pub fn input_poll(&mut self, callback: impl FnMut(InputMessage)) {
        self.input_manager.poll(callback);
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
        self.render_client.batch_update(batch, desc)
    }

    pub fn batch_remove(&mut self, batch: &BatchReference) {
        self.render_client.batch_remove(batch)
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
        self.render_client.window_title(title)
    }

    /// Commits the queued window related changes to the renderer. This may block
    /// if the renderer is getting changes faster than it can process.
    pub fn window_commit(&mut self) {
        self.render_client.commit()
    }
}
