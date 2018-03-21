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
mod engine;

use game::test_game::*;

fn init() {
    math::init();
}

fn main() {
    // Init code. This must run first, before the rest of the program,
    // otherwise there'll be undefined behavior.
    init();

    // utility::tests::compare_single_spsc_throughput();
    engine::run(TestGame::new());
}
