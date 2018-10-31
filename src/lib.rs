#![feature(asm, const_fn, pointer_methods, optin_builtin_traits)]
#![allow(dead_code, non_camel_case_types, non_snake_case)]

extern crate bounded_spsc_queue;
extern crate gl;
extern crate glutin;
extern crate image;

pub extern crate cgmath;
#[macro_use]
pub extern crate log;

pub mod channel;
pub mod game;
pub mod input;
pub mod math;
pub mod render;
pub mod time;
pub mod utility;

mod logger;
#[cfg(test)]
mod test;

use cgmath::*;
use channel::consume_spsc;
use channel::replace_spsc;
use game::*;
use glutin::dpi::*;
use logger::*;
use render::display::*;
use std::thread;

/// Creates and runs a game. Threads for input, rendering, and game logic are created along with
/// communication channels between them. The game is then instantiated. This function blocks until
/// the game window is closed.
pub fn run<G: Game>() {
    // Initialze logging.
    SimpleLogger::init();

    // Winow creation
    let event_loop = glutin::EventsLoop::new();
    let display = Display::new(
        glutin::WindowBuilder::new()
            .with_title(G::TITLE)
            .with_dimensions(LogicalSize::from((500, 500))),
        glutin::ContextBuilder::new().with_multisampling(2),
        &event_loop,
    );

    // Inter-thread message queues for input and rendering
    let (render_producer_pipe, render_consumer_pipe) = bounded_spsc_queue::make(4);
    let (input_producer_pipe, input_consumer_pipe) = bounded_spsc_queue::make(256);
    let (resize_producer, resize_consumer) = consume_spsc::make();
    let (cursor_producer, _cursor_consumer) = replace_spsc::make(Vector2::zero());

    // Game thread (daemon)
    thread::spawn(move || {
        game::start::<G>(input_consumer_pipe, render_producer_pipe);
    });

    // Render thread (daemon)
    thread::spawn(move || {
        render::start(display, render_consumer_pipe, resize_consumer);
    });

    // Input thread (main)
    input::start(event_loop, input_producer_pipe, resize_producer, cursor_producer);
}
