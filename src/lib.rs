#![allow(dead_code, non_camel_case_types, non_snake_case, intra_doc_link_resolution_failure)]

#[macro_use]
pub extern crate log;

pub use cgmath;

pub mod math;
pub mod time;

pub use crate::input::*;
pub use crate::types::*;

mod input;
mod render;
mod text;
mod texture;
mod types;
mod utility;

use crate::render::{RenderClient, RenderServer};
use crate::utility::{bounded_spsc, control, swap_spsc};
use glutin::event::Event;
use glutin::event_loop::ControlFlow;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::thread;

/// The main entry point into the Storm engine. All interactions with the engine are managed by the
/// API on this type. The engine is send, and can be moved between threads.
pub struct Engine {
    render_client: RenderClient,
    input_client: InputClient,
}

trait Program {
    fn create(engine: &mut Engine) -> Self;

    fn input(&mut self, event: InputMessage, engine: &mut Engine);

    fn update(&mut self, engine: &mut Engine);
}

impl Engine {
    // Starts the engine. The game_loop parameter is called once with a valid instance of the engine
    // once the engine is constructed. If the game loop exits or panics, the engine shuts down.
    pub fn start(desc: WindowSettings, mut game_loop: impl FnMut(Engine) + Send + 'static) {
        info!("Engine started.");

        // Init winit
        let event_loop = glutin::event_loop::EventLoop::new();

        // Inter-thread messaging.
        let (render_producer_pipe, render_consumer_pipe) = swap_spsc::make();
        let (input_producer_pipe, input_consumer_pipe) = bounded_spsc::make(512);
        let (engine_watcher, engine_probe) = control::make_probe();

        // Rendering and input
        let mut render_server = RenderServer::new(&desc, &event_loop, render_consumer_pipe);
        let mut input_server = InputServer::new(input_producer_pipe, desc);

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

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent {
                    event,
                    ..
                } => {
                    input_server.push(event);
                }
                Event::MainEventsCleared => {
                    input_server.finalize();
                    render_server.tick();
                }
                Event::LoopDestroyed => {
                    return;
                }
                _ => {}
            }
            if !engine_watcher.alive() {
                *control_flow = ControlFlow::Exit;
            }
        });
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

    /// Loads a new texture from a given path. If there is an issue loading the texture, this
    /// function will panic.
    pub fn texture_load<P: AsRef<Path>>(&mut self, path: P, format: TextureFormat) -> Result<Texture, &str> {
        if let Ok(f) = File::open(path) {
            let reader = BufReader::new(f);
            Ok(self.render_client.texture_create(reader, format))
        } else {
            Err("Unable to open file to read path.")
        }
    }

    /// Loads a new texture from an in memory source. If there is an issue loading the texture, this
    /// function will panic.
    ///
    /// If loading from an array, like from include_bytes!(), you can use as_ref() on the array to
    /// convert it into a readable type.
    pub fn texture_create<R: Read>(&mut self, reader: R, format: TextureFormat) -> Texture {
        self.render_client.texture_create(reader, format)
    }

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

    /// Sets the display mode of the window.
    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.render_client.window_display_mode(display_mode)
    }

    /// Sets the vsync setting for the window.
    pub fn window_vsync(&mut self, vsync: Vsync) {
        self.render_client.window_vsync(vsync);
    }

    /// Commits the queued window, batch, sprite, text, and texture related changes to the renderer.
    /// This function will not block.
    pub fn window_commit(&mut self) {
        self.render_client.commit();
    }
}
