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
extern crate rayon;
extern crate rusttype;
extern crate test;
extern crate unicode_normalization;

pub extern crate cgmath;

pub mod color;
pub mod math;
pub mod time;

mod font;
mod input;
mod layer;
mod render;
mod sprite;
mod text;
mod texture;
mod utility;

pub use input::*;
pub use layer::*;
pub use sprite::*;
pub use text::*;
pub use texture::*;

use glutin::dpi::*;
use render::*;
use std::thread;
use utility::bucket_spsc;
use utility::control;

/// The main entry point into the Storm engine.
pub struct Engine {
    render_client: RenderClient,
    input_manager: InputManager,
}

impl Engine {
    // TODO: Allow for assigning window setting on initial creation
    /// Creates and runs an instance of the engine. This creates a window on
    /// another thread which listens for messages from the engine.
    pub fn new() -> Engine {
        // Inter-thread messaging for rendering
        let (render_producer_control, render_consumer_control) = control::make();
        let (render_producer_pipe, render_consumer_pipe) = bucket_spsc::make(2);

        // Winow creation
        let event_loop = glutin::EventsLoop::new();
        let window = Window::new(
            glutin::ContextBuilder::new()
                .build_windowed(
                    glutin::WindowBuilder::new()
                        .with_title("Storm Engine")
                        .with_dimensions(LogicalSize::from((500, 500))),
                    &event_loop,
                )
                .expect("Unable to build the window."),
        );

        // Rendering
        thread::spawn(move || {
            render::start(window, render_consumer_pipe, render_consumer_control);
        });

        Engine {
            render_client: RenderClient::new(render_producer_pipe, render_producer_control),
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
    // Layer
    // ////////////////////////////////////////////////////////

    pub fn layer_create(&mut self, depth: usize, desc: &LayerDescription) -> LayerReference {
        self.render_client.layer_create(depth, desc)
    }

    pub fn layer_update(&mut self, layer: &LayerReference, desc: &LayerDescription) {
        self.render_client.layer_update(layer, desc)
    }

    pub fn layer_remove(&mut self, layer: &LayerReference) {
        self.render_client.layer_remove(layer)
    }

    pub fn layer_clear(&mut self, layer: &LayerReference) {
        self.render_client.layer_clear(layer)
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_create(&mut self, layer: &LayerReference, desc: &SpriteDescription) -> SpriteReference {
        self.render_client.sprite_create(layer, desc)
    }

    pub fn sprite_update(&mut self, sprite: &SpriteReference, desc: &SpriteDescription) {
        self.render_client.sprite_update(sprite, desc)
    }

    pub fn sprite_remove(&mut self, sprite: &SpriteReference) {
        self.render_client.sprite_remove(sprite)
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    /// Loads a new texture.
    pub fn texture_load(&mut self, path: &str) -> TextureReference {
        self.render_client.texture_create(path)
    }

    /// Gets the default texture reference.
    pub fn texture_default() -> TextureReference {
        DEFAULT_TEXTURE
    }

    // ////////////////////////////////////////////////////////
    // Text
    // ////////////////////////////////////////////////////////

    pub fn font_load(&mut self, path: &str) -> FontReference {
        self.render_client.font_create(path)
    }

    pub fn font_default(&mut self) -> FontReference {
        DEFAULT_FONT
    }

    pub fn text_create(
        &mut self,
        layer: &LayerReference,
        text: &str,
        desc: &TextDescription,
    ) -> TextReference {
        self.render_client.text_create(layer, text, desc)
    }

    pub fn text_update(&mut self, text_ref: &TextReference, text: &str, desc: &TextDescription) {
        self.render_client.text_update(text_ref, text, desc);
    }

    pub fn text_remove(&mut self, text_ref: &TextReference) {
        self.render_client.text_remove(text_ref);
    }

    // ////////////////////////////////////////////////////////
    // Audio
    // ////////////////////////////////////////////////////////

    // TODO: Audio
    // pub fn audio_load(&mut self) {}

    // pub fn audio_play(&mut self) {}

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
