#![feature(asm, const_fn, untagged_unions, optin_builtin_traits)]
#![allow(dead_code, unions_with_drop_fields)]

extern crate bounded_spsc_queue;
extern crate cgmath;
extern crate gl;
extern crate glutin;

pub mod input;
pub mod math;
pub mod physics;
pub mod render;
pub mod time;
pub mod utility;
pub mod engine;
pub mod game;

#[cfg(test)]
mod test;
