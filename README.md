# Storm Engine
Playing around with opengl in rust.

## Example

```rust
extern crate log;
extern crate storm;

mod logger;

use log::LevelFilter;
use logger::*;
use storm::time::*;
use storm::*;

fn main() {
    SimpleLogger::init(LevelFilter::Trace);

    // Tick at 60 ticks per second.
    let mut clock = Clock::new(60);
    // Create the engine context.
    let mut engine = Engine::new();
    // Create a layer to draw on.
    let layer = engine.layer_create(0, &LayerDescription::default());
    // Add a sprite to that layer (a white square).
    engine.sprite_create(&layer, &SpriteDescription::default());

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
        // This sleeps until it's ready for the next tick, ensuring 60 TPS.
        clock.tick();
    }
}
```
