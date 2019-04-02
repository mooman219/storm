extern crate log;
extern crate storm;

mod logger;

use log::LevelFilter;
use logger::*;
use storm::*;

/// Run with: cargo run --example testgame --release
fn main() {
    SimpleLogger::init(LevelFilter::Trace);

    let mut engine = Engine::new();
    let layer = engine.layer_create(0, &LayerDescription::default());
    engine.sprite_create(&layer, &SpriteDescription::default());

    let mut is_active = true;
    while is_active {
        engine.input_poll(|message| match message {
            InputMessage::CloseRequested => is_active = false,
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => {
                    is_active = false;
                },
                _ => {},
            },
            _ => {},
        });

        engine.window_commit();
    }
}
