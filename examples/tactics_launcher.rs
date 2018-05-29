#![allow(dead_code)]
#![feature(rustc_private)]
extern crate rand;
extern crate storm;

use storm::game::*;
use storm::input::message::*;
//use storm::log::LevelFilter;
use storm::render::message::*;
use storm::time::clock::*;

mod tactics;
use tactics::system::System;

pub struct TacticsLauncher {
    system: System,
    render: RenderProducer,
    clock: Clock,
}

impl Game for TacticsLauncher {
    const TITLE: &'static str = "Terra Ingognita";

    fn new(render: RenderProducer) -> Self {
        TacticsLauncher {
            system: System::new(),
            render,
            clock: Clock::new(200),
        }
    }

    fn input(&mut self, event: InputFrame) {
        self.system.handle_input(event);
    }

    fn tick(&mut self) {
        self.system.update(&mut self.render);
        self.render.send();
    }
}

fn main() {
    storm::run::<TacticsLauncher>();
}
