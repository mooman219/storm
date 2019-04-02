extern crate log;
extern crate storm;

mod logger;
use log::LevelFilter;
use logger::*;

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
