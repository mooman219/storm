#![allow(dead_code)]
#![feature(rustc_private)]
extern crate storm;
extern crate rand;


use storm::game::*;
use storm::input::message::*;
use storm::render::message::*;
use storm::time::clock::*;

mod game_of_life;
use game_of_life::system::System;

pub struct GOL {
    system: System,
    render: RenderProducer,
    clock: Clock
}

impl Game for GOL {
    const TITLE: &'static str = "Game Of Life";

    fn new(mut render: RenderProducer) -> Self {
        let mut new_gol =
        GOL {
            system: System::new(&mut render),
            render: render,
            clock: Clock::new(1)
        };

        new_gol
    }

    fn input(&mut self, event: InputFrame) {
        self.system.handle_input(event);
    }

    fn tick(&mut self) {
        self.system.tick(&mut self.render);
        self.render.send();
    }
}

fn main() {
    storm::run::<GOL>();
}