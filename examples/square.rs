use storm::time::*;
use storm::*;

/// Run with: cargo run --example square --release
fn main() {
    simple_logger::init().unwrap();
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
    // Add all the sprites we want to draw to a vec.
    let mut sprites = Vec::new();
    sprites.push(Sprite::default());
    // Assign the sprites we want to draw to a batch.
    engine.sprite_set(&screen, &sprites);
    // Add all the strings we want to draw to a vec.
    let mut strings = Vec::new();
    let mut text = Text::default();
    text.set_string("Hello world!");
    text.pos.z = 0.5;
    text.pos.y += 50.0;
    text.max_width = Some(65.0);
    strings.push(text);
    // Assign the strings we want to draw to a batch.
    engine.text_set(&screen, &strings);

    let mut is_active = true;
    while is_active {
        // Input for closing the window.
        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                InputMessage::KeyPressed(key) => match key {
                    KeyboardButton::Escape => is_active = false,
                    _ => {}
                },
                _ => {}
            }
        }
        // Commit any state we changed to the window. This will trigger a draw.
        engine.window_commit();
        // This sleeps until it's ready for the next tick, ensuring the 144 TPS we set earlier.
        clock.tick();
    }
}
