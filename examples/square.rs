extern crate log;
extern crate storm;

mod logger;

use log::LevelFilter;
use logger::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example square --release
fn main() {
    SimpleLogger::init(LevelFilter::Trace);

    let mut clock = Clock::new(144);
    let mut engine = Engine::new();

    let screen = engine.batch_create(&BatchDescription::default());
    let mut sprites = Vec::new();
    sprites.push(SpriteDescription::default());
    engine.sprite_set(&screen, &sprites);

    let mut is_active = true;
    while is_active {
        engine.input_poll(|message| match message {
            InputMessage::CloseRequested => is_active = false,
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => is_active = false,
                _ => {},
            },
            _ => {},
        });

        engine.window_commit();
        clock.tick();
    }
}
