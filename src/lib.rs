#![feature(asm, const_fn)]
#![allow(dead_code, non_camel_case_types, non_snake_case)]

extern crate core;
extern crate gl;
extern crate glutin;
extern crate image;

pub extern crate cgmath;
#[macro_use]
pub extern crate log;

pub mod channel;
pub mod input;
pub mod layer;
pub mod manager;
pub mod math;
pub mod message;
pub mod render;
pub mod sprite;
pub mod texture;
pub mod time;
pub mod utility;

mod logger;
#[cfg(test)]
mod test;

use cgmath::*;
use channel::bounded_spsc;
use channel::consume_spsc;
use channel::replace_spsc;
use glutin::dpi::*;
use layer::*;
use logger::*;
use manager::*;
use message::*;
use render::display::*;
use sprite::*;
use std::mem;
use std::thread;
use texture::*;

/// Creates and runs a game. Threads for input, rendering, and game logic are created along with
/// communication channels between them. The game is then instantiated. This function blocks until
/// the game window is closed.
pub fn run<T: FnOnce(Engine) + Send + 'static>(game: T) {
    // Initialze logging.
    SimpleLogger::init();

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
    let (input_producer_pipe, input_consumer_pipe) = bounded_spsc::make(512);
    let (resize_producer, resize_consumer) = consume_spsc::make();
    let (cursor_producer, _cursor_consumer) = replace_spsc::make(Vector2::zero());

    // Game thread (daemon)
    thread::spawn(move || {
        game(Engine::new(render_producer_pipe, input_consumer_pipe));
    });

    // Render thread (daemon)
    thread::spawn(move || {
        render::start(display, render_consumer_pipe, resize_consumer);
    });

    // Input thread (main)
    // input::start(event_loop, input_producer_pipe, resize_producer, cursor_producer);
}

pub struct Engine {
    render_batch: Vec<RenderMessage>,
    render_pipe: bounded_spsc::Producer<Vec<RenderMessage>>,
    input_pipe: bounded_spsc::Consumer<InputMessage>,
    state_manager: StateManager,
}

impl Engine {
    fn new(
        render_pipe: bounded_spsc::Producer<Vec<RenderMessage>>,
        input_pipe: bounded_spsc::Consumer<InputMessage>,
    ) -> Engine {
        Engine {
            render_batch: Vec::new(),
            render_pipe: render_pipe,
            input_pipe: input_pipe,
            state_manager: StateManager::new(),
        }
    }

    // ////////////////////////////////////////////////////////
    // Engine
    // ////////////////////////////////////////////////////////

    // pub fn engine_input_alive() -> bool {
    //     // todo
    //     false
    // }

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

    pub fn input_poll(&mut self) -> Option<InputMessage> {
        self.input_pipe.try_pop()
    }

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

    pub fn texture_create(&mut self, raw: Vec<u8>, height: usize, width: usize) -> TextureReference {
        self.render_batch.push(RenderMessage::TextureCreate {
            raw: raw,
            height: height,
            width: width,
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
