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

use crate::input::InputConverter;
use crate::render::Renderer;
use glutin::event::Event;
use glutin::event_loop::ControlFlow;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

/// The main entry point into the Storm engine. All interactions with the engine are managed by the
/// API on this type. The engine is send, and can be moved between threads.
pub struct Engine {
    renderer: Renderer,
    control_flow: ControlFlow,
}

pub trait Program: Sized {
    fn create(engine: &mut Engine) -> Result<Self, &'static str>;

    fn input(&mut self, event: InputMessage, engine: &mut Engine);

    fn update(&mut self, engine: &mut Engine);
}

impl Engine {
    pub fn start<T: 'static + Program>(desc: WindowSettings) {
        info!("Starting engine...");
        let event_loop = glutin::event_loop::EventLoop::new();
        let renderer = Renderer::new(&desc, &event_loop);
        let mut input = InputConverter::new(renderer.current_logical_size());
        let mut engine = Engine {
            renderer,
            control_flow: ControlFlow::Poll,
        };
        info!("Starting program...");
        let mut program = T::create(&mut engine).expect("Unable to create program.");
        info!("Starting loop...");
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event,
                    ..
                } => {
                    input.push(event, &mut program, &mut engine);
                }
                Event::MainEventsCleared => {
                    program.update(&mut engine);
                }
                Event::LoopDestroyed => {
                    engine.control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
            *control_flow = engine.control_flow;
        });
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
        self.renderer.batch_create(desc)
    }

    /// Updates the settings for an existing batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn batch_update(&mut self, batch: &BatchToken, desc: &BatchSettings) {
        self.renderer.batch_update(batch, desc);
    }

    /// Removes an existing batch from the engine. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn batch_remove(&mut self, batch: &BatchToken) {
        self.renderer.batch_remove(batch);
    }

    // ////////////////////////////////////////////////////////
    // Sprite
    // ////////////////////////////////////////////////////////

    /// Sets the sprites to render for a given batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn sprite_set(&mut self, batch: &BatchToken, descs: &Vec<Sprite>) {
        self.renderer.sprite_set(batch, descs);
    }

    /// Clears all sprites from the given batch. This does the same thing as passing an empty Vec to
    /// sprite_set. If the token references an invalid or removed batch, this will panic.
    pub fn sprite_clear(&mut self, batch: &BatchToken) {
        self.renderer.sprite_clear(batch);
    }

    // ////////////////////////////////////////////////////////
    // Text
    // ////////////////////////////////////////////////////////

    /// Loads a new font and returns a token to reference it with later.
    pub fn font_load(&mut self, path: &str) -> FontToken {
        self.renderer.font_create(path)
    }

    // TODO: Alternative font loading functions.

    /// Sets the text to render for a given batch. If the token references an invalid or removed
    /// batch, this will panic.
    pub fn text_set(&mut self, batch: &BatchToken, descs: &Vec<Text>) {
        self.renderer.string_set(batch, descs);
    }

    /// Clears all text from the given batch. This does the same thing as passing an empty Vec to
    /// sprite_set. If the token references an invalid or removed batch, this will panic.
    pub fn text_clear(&mut self, batch: &BatchToken) {
        self.renderer.string_clear(batch);
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
            Ok(self.renderer.texture_create(reader, format))
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
        self.renderer.texture_create(reader, format)
    }

    // ////////////////////////////////////////////////////////
    // Window
    // ////////////////////////////////////////////////////////

    /// Sets the title of the window.
    pub fn window_title(&mut self, title: &str) {
        self.renderer.window_title(title);
    }

    /// Sets the clear color for the window.
    pub fn window_clear_color(&mut self, clear_color: RGBA8) {
        self.renderer.window_clear_color(clear_color);
    }

    /// Sets the display mode of the window.
    pub fn window_display_mode(&mut self, display_mode: DisplayMode) {
        self.renderer.window_display_mode(display_mode)
    }

    /// Sets the vsync setting for the window.
    pub fn window_vsync(&mut self, vsync: Vsync) {
        self.renderer.window_vsync(vsync);
    }

    /// Stops the engine after the next update.
    pub fn stop(&mut self) {
        info!("Stopping engine...");
        self.control_flow = ControlFlow::Exit;
    }

    /// Draws the current renderer state to the screen.
    pub fn draw(&mut self) {
        self.renderer.draw();
    }
}
