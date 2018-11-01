#![allow(dead_code)]
#![feature(rustc_private)]
extern crate rand;
extern crate storm;

use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::message::*;
use storm::time::clock::*;

mod pong; //system;
use pong::system::System;

/// Run with: cargo run --example pong_driver --release
pub struct Pong {
    system: System,
    render: RenderMessenger,
    clock: Clock,
}

impl Game for Pong {
    const TITLE: &'static str = "Pong";

    fn new(mut render: RenderMessenger) -> Self {
        let new_pong = Pong {
            system: System::new(&mut render),
            render: render,
            clock: Clock::new(60),
        };

        new_pong
    }

    fn input(&mut self, event: InputFrame) {
        self.system.input(event);
    }

    fn tick(&mut self) {
        self.system.tick(&mut self.render);
        self.render.send();
        self.clock.tick();
    }
}

fn main() {
    storm::log::set_max_level(LevelFilter::Trace);
    storm::run::<Pong>();
}
