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
    let texture = engine.texture_load("./examples/resources/2.png");
    let layer = engine.layer_create(0, &LayerDescription::default());
    let mut sprite = SpriteDescription::default();
    sprite.texture = texture;
    engine.sprite_create(&layer, &sprite);

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
