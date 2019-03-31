extern crate storm;

mod logger;
use logger::*;
use storm::log::LevelFilter;

use std::thread::sleep;
use std::time::Duration;
use storm::cgmath::*;
use storm::layer::*;
use storm::render::color;
use storm::render::color::Color;
use storm::sprite::*;
use storm::time::clock::*;
use storm::*;

/// Run with: cargo run --example testgame --release
fn main() {
    // Initialze logging.
    SimpleLogger::init(LevelFilter::Trace);

    let mut engine = Engine::new();
    let layer = engine.layer_create(0, &LayerDescription::default());
    loop {
        engine.sprite_create(&layer, &SpriteDescription::default());
        engine.window_commit();
        engine.test();
    }
}
