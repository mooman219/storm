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
    // Init code.
    init();

    // Render messaging. Max of 3 frames can be buffered.
    let (render_producer, render_consumer) = bounded_spsc_queue::make(3);
    let (input_producer, input_consumer) = bounded_spsc_queue::make(3);

    thread::spawn(move || {
        game::game_loop(render_producer, input_consumer);
    });

    // Render and input loops follow.

    // Setup communication
    let (mut input, mut render) = render::create_target(input_producer, render_consumer);
    // Loop
    while input.is_active() {
        // Input
        input.tick();
        // Render
        render.handle_resize(input.next_resize());
        render.tick();
        // Sleep
        sleep(Duration::new(0, 100));
    }
}
