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

fn main() {
    SimpleLogger::init(LevelFilter::Trace);

    // Tick at 144 ticks per second.
    let mut clock = Clock::new(144);
    // Create the engine context.
    let mut engine = Engine::new(WindowDescription {
        title: String::from("Storm: Square"),
        size: Vector2::new(1280, 1024),
        resizable: true,
    });;
    // Create a batch to draw on.
    let screen = engine.batch_create(&BatchDescription::default());
    // Add a sprite to that batch (a white square).
    let mut sprites = Vec::new();
    sprites.push(SpriteDescription::default());
    engine.sprite_set(&screen, &sprites);


    let mut is_active = true;
    while is_active {
        // Input for closing the window.
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

        // Draw to the window.
        engine.window_commit();
        // This sleeps until it's ready for the next tick, ensuring 144 TPS.
        clock.tick();
    }
}
```
