#![feature(asm, test, const_fn, untagged_unions, optin_builtin_traits)]
#![allow(dead_code, unions_with_drop_fields)]

extern crate bounded_spsc_queue;
extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate test;

mod game;
mod input;
mod math;
mod physics;
mod render;
mod time;
mod utility;

use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn init() {
    math::init();
}

fn main() {
    // Init code. This must run first, before the rest of the program,
    // otherwise there'll be undefined behavior.
    init();

    // Render messaging. Max of 3 frames.
    let (render_producer, render_consumer) = bounded_spsc_queue::make(3);
    // Input messaging. Max of 100 frames.
    let (input_producer, input_consumer) = bounded_spsc_queue::make(100);

    thread::spawn(move || {
        game::game_loop(render_producer, input_consumer);
    });

    // Render and input loops follow. They must exist on the same thread as
    // they're coupled together by the window.

    // Setup communication
    let (mut input, mut render) = render::create_target(input_producer, render_consumer);
    // Loop
    while input.is_active() {
        // Input
        input.tick();
        // Render
        render.resize(input.next_resize());
        render.tick();
        // Sleep
        sleep(Duration::new(0, 100));
    }
}
