#![feature(shared, asm, test, const_fn, const_size_of, untagged_unions, optin_builtin_traits)]
#![allow(dead_code, unions_with_drop_fields)]

extern crate bounded_spsc_queue;
extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate rand;
extern crate test;

mod game;
mod math;
mod physics;
mod render;
mod time;
mod utility;

use std::thread;

fn init() {
    math::init();
}

fn main() {
    // Init code.
    init();

    let (frame_producer, frame_consumer) = bounded_spsc_queue::make(3);

    thread::spawn(move || {
        game::game_loop(frame_producer);
    });
    render::render_loop(frame_consumer);
}
