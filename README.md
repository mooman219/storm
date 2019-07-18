# Storm

[![Documentation](https://docs.rs/storm/badge.svg)](https://docs.rs/storm)
[![Crates.io](https://img.shields.io/crates/v/storm.svg)](https://crates.io/crates/storm)
[![License](https://img.shields.io/crates/l/storm.svg)](https://github.com/mooman219/storm/blob/master/LICENSE)

The storm engine is a simple 2D renderer designed for performance. It currently features an OpenGL 3.3 backend and supports Windows, Linux, and Mac. 

The engine is experimental and __will__ change at any time and requires SDL build. For Windows, SDL is included automatically. On Mac and Linux, follow the instructions [here](https://github.com/Lokathor/beryllium#building) to setup SDL.

## Todo

- Audio API
- Increase texture storage space

## Example
This example will render a white square in about the center of the screen with text below it.

```rust
use storm::time::*;
use storm::*;

fn main() {
    // Create the engine context and describe the window.
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Square"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Disabled,
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
        let mut text = Text::default();
        text.set_string("Hello world!");
        text.color = color::WHITE;
        text.pos.y -= 50.0;
        strings.push(text);
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
```
