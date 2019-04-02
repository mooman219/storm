#![feature(asm, const_fn)]
#![allow(dead_code, non_camel_case_types, non_snake_case)]

extern crate core;
extern crate gl;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate log;

pub extern crate cgmath;

pub mod channel;
pub mod math;
pub mod time;
pub mod utility;

mod color;
mod input;
mod layer;
mod manager;
mod render;
mod sprite;
#[cfg(test)]
mod test;
mod texture;

pub use color::*;
pub use input::*;
pub use layer::*;
pub use sprite::*;
pub use texture::*;

use channel::bounded_spsc;
use glutin::dpi::*;
use glutin::EventsLoop;
use manager::*;
use render::*;
use std::mem;
use std::thread;

/// The main entry point into the Storm engine.
pub struct Engine {
    render_batch: Vec<RenderMessage>,
    render_pipe: bounded_spsc::Producer<Vec<RenderMessage>>,
    state_manager: StateManager,
    event_loop: EventsLoop,
}

impl Engine {
    /// Creates and runs an instance of the engine. This creates a window on
    /// another thread which listens for messages from the engine.
    pub fn new() -> Engine {
        // Winow creation
        let event_loop = glutin::EventsLoop::new();
        let display = Display::new(
            glutin::WindowBuilder::new()
                .with_title("Storm Engine")
                .with_dimensions(LogicalSize::from((500, 500))),
            glutin::ContextBuilder::new().with_multisampling(2),
            &event_loop,
        );

        // Inter-thread message queues for input and rendering
        let (render_producer_pipe, render_consumer_pipe) = bounded_spsc::make(4);

        // Render thread (daemon)
        thread::spawn(move || {
            render::start(display, render_consumer_pipe);
        });

        Engine {
            render_batch: Vec::new(),
            render_pipe: render_producer_pipe,
            state_manager: StateManager::new(),
            event_loop: event_loop,
        }
    }

    pub fn test(&mut self) {
        loop {
            self.event_loop.poll_events(|event| match event {
                _ => (),
            });
            // self.window_commit();
        }
    }

    // ////////////////////////////////////////////////////////
    // Engine
    // ////////////////////////////////////////////////////////

    // pub fn engine_render_alive() -> bool {
    //     // todo
    //     false
    // }

    // pub fn engine_shutdow(&mut self) {
    //     // todo
    // }

    // ////////////////////////////////////////////////////////
    // Input
    // ////////////////////////////////////////////////////////

    // pub fn input_poll(&mut self) -> Option<InputMessage> {
    //     self.input_pipe.try_pop()
    // }

    // ////////////////////////////////////////////////////////
    // Layer
    // ////////////////////////////////////////////////////////

    pub fn layer_create(&mut self, depth: usize, desc: &LayerDescription) -> LayerReference {
        let (message, layer) = self.state_manager.layer_create(depth, desc);
        self.render_batch.push(message);
        layer
    }

    pub fn layer_update(&mut self, layer: &LayerReference, desc: &LayerDescription) {
        let message = self.state_manager.layer_update(layer, desc);
        self.render_batch.push(message);
    }

    pub fn layer_remove(&mut self, layer: &LayerReference) {
        let message = self.state_manager.layer_remove(layer);
        self.render_batch.push(message);
    }

    pub fn layer_clear(&mut self, layer: &LayerReference) {
        let message = self.state_manager.layer_clear(layer);
        self.render_batch.push(message);
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    pub fn sprite_create(&mut self, layer: &LayerReference, desc: &SpriteDescription) -> SpriteReference {
        let (message, sprite) = self.state_manager.sprite_create(layer, desc);
        self.render_batch.push(message);
        sprite
    }

    pub fn sprite_update(&mut self, sprite: &SpriteReference, desc: &SpriteDescription) {
        let message = self.state_manager.sprite_update(sprite, desc);
        self.render_batch.push(message);
    }

    pub fn sprite_remove(&mut self, sprite: &SpriteReference) {
        let message = self.state_manager.sprite_remove(sprite);
        self.render_batch.push(message);
    }

    // ////////////////////////////////////////////////////////
    // Texture
    // ////////////////////////////////////////////////////////

    pub fn texture_load(&mut self, path: &str) -> TextureReference {
        self.render_batch.push(RenderMessage::TextureLoad {
            path: String::from(path),
        });
        self.state_manager.texture_create()
    }

    pub fn texture_default() -> TextureReference {
        DEFAULT_TEXTURE
    }

    // ////////////////////////////////////////////////////////
    // Font
    // ////////////////////////////////////////////////////////

    // pub fn font_load(&mut self) {
    //     // todo
    // }

    // pub fn font_default(&mut self) {
    //     // todo
    // }

    // ////////////////////////////////////////////////////////
    // Text
    // ////////////////////////////////////////////////////////

    // ////////////////////////////////////////////////////////
    // Audio
    // ////////////////////////////////////////////////////////

    // pub fn audio_load(&mut self) {
    //     // todo
    // }

    // pub fn audio_play(&mut self) {
    //     // todo
    // }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    pub fn window_title(&mut self, title: &str) {
        self.render_batch.push(RenderMessage::WindowTitle {
            title: String::from(title),
        });
    }

    // pub fn window_size(&mut self) {
    //     // todo
    // }

    pub fn window_commit(&mut self) {
        let mut batch = Vec::new();
        mem::swap(&mut batch, &mut self.render_batch);
        self.render_pipe.push(batch);
    }
}
