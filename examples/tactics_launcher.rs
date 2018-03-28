#![allow(dead_code)]
#![feature(rustc_private)]
extern crate rand;
extern crate storm;

use storm::cgmath::*;
use storm::game::*;
use storm::input::message::*;
use storm::render::color;
use storm::render::message::*;
use storm::time::clock::*;
use storm::utility::slotmap::*;


mod tactics;
use tactics::system::System;

fn main() {
    let mut system = System::new();
    system.update();
}