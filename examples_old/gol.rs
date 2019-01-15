#![allow(dead_code)]
#![feature(rustc_private)]
extern crate rand;
extern crate storm;

use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::message::*;
use storm::time::clock::*;

mod game_of_life;
use game_of_life::system::System;

/// Run with: cargo run --example gol --release
pub struct GOL {
    system: System,
    render: RenderMessenger,
    clock: Clock,
}

impl Game for GOL {
    fn new(mut render: RenderMessenger) -> Self {
        render.window_title("Game of Life");
        let new_gol = GOL {
            system: System::new(&mut render),
            render: render,
            clock: Clock::new(1),
        };

        new_gol
    }

    fn input(&mut self, event: InputFrame) {
        self.system.handle_input(event);
    }

    fn tick(&mut self) {
        self.system.tick(&mut self.render);
        self.render.send();
        self.clock.tick();
    }
}

fn main() {
    storm::log::set_max_level(LevelFilter::Trace);
    storm::run::<GOL>();
}
