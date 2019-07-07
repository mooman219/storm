use crate::cgmath::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example square --release
fn main() {
    // Create the engine context and describe the window.
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Square"),
            size: Vector2::new(1280, 1024),
            resizable: true,
        },
        game,
    );
}

fn game(mut engine: Engine) {
    // Tick at 144 ticks per second.
    let mut clock = Clock::new(144);
    // Create a batch to draw on. Batches persist between engine.window_commit()'s.
    let screen = engine.batch_create(&BatchSettings::default());
    {
        // Add all the sprites we want to draw to a vec.
        let mut sprites = Vec::new();
        sprites.push(Sprite::default());
        // Assign the sprites we want to draw to a batch.
        engine.sprite_set(&screen, &sprites);
    }
    {
        // Add all the strings we want to draw to a vec.
        let mut strings = Vec::new();
        let mut string = Text::default();
        string.string.push_str("Hello world!");
        string.color = color::WHITE;
        string.pos.y -= 50.0;
        strings.push(string);
        // Assign the strings we want to draw to a batch.
        engine.text_set(&screen, &strings);
    }
    let mut is_active = true;
    while is_active {
        // Input for closing the window.
        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                InputMessage::KeyPressed(key) => match key {
                    KeyboardButton::Escape => is_active = false,
                    _ => {},
                },
                _ => {},
            }
        }
        // Commit any state we changed to the window. This will trigger a draw.
        engine.window_commit();
        // This sleeps until it's ready for the next tick, ensuring the 144 TPS we set earlier.
        clock.tick();
    }
}
