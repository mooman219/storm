# Storm Engine
The storm engine is a simple 2D renderer designed for performance. It currently features an OpenGL backend and supports Windows, Linux, ans Mac. The engine is experimental and can change at any time.

## Example
This example will render a white square in about the center of the screen.

```rust
extern crate log;
extern crate storm;

mod logger;

use cgmath::*;
use log::LevelFilter;
use logger::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example square --release
fn main() {
    // It's ideal to setup a logger to get the engine output.
    SimpleLogger::init(LevelFilter::Trace);
    // Tick at 144 ticks per second.
    let mut clock = Clock::new(144);
    // Create the engine context and describe the window.
    let mut engine = Engine::new(WindowDescription {
        title: String::from("Storm: Square"),
        size: Vector2::new(1280, 1024),
        resizable: true,
    });
    // Create a batch to draw on. Batches persist between engine.window_commit()'s.
    let screen = engine.batch_create(&BatchDescription::default());
    {
        // Add all the sprites we want to draw to a vec.
        let mut sprites = Vec::new();
        sprites.push(SpriteDescription::default());
        // Assign the sprites we want to draw to a batch.
        engine.sprite_set(&screen, &sprites);
    }
    {
        // Add all the strings we want to draw to a vec.
        let mut strings = Vec::new();
        let mut string = StringDescription::default();
        string.string.push_str("Hello world!");
        string.color = color::WHITE;
        string.pos.y -= 50.0;
        strings.push(string);
        // Assign the strings we want to draw to a batch.
        engine.string_set(&screen, &strings);
    }
    let mut is_active = true;
    while is_active {
        // Input for closing the window.
        engine.input_poll(|message| match message {
            InputMessage::CloseRequested => is_active = false,
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => is_active = false,
                _ => {},
            },
            _ => {},
        });
        // Commit any state we changed to the window. This will trigger a draw.
        engine.window_commit();
        // This sleeps until it's ready for the next tick, ensuring the 144 TPS we set earlier.
        clock.tick();
    }
}
```
