use core::time::Duration;
use storm::*;

/// Run with: cargo run --example square --release
fn main() {
    simple_logger::SimpleLogger::new().init().expect("Unable to init logger");
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
        run,
    );
}

fn run(engine: &mut Engine) -> impl FnMut(InputMessage, &mut Engine) {
    engine.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
    let mut is_dragging = false;
    // Create a batch to draw on. Batches persist between engine.window_commit()'s.
    let mut screen_settings = BatchSettings {
        ..BatchSettings::default()
    };
    let screen = engine.render.batch_create(&screen_settings);
    // Add all the strings we want to draw to a vec.
    let mut message = String::from("> the quick brown fox jumps over the lazy dog.");
    let mut strings = Vec::new();
    let mut text = Text::default();
    text.set_string(&message);
    text.pos.x = 100.0;
    text.pos.y = 500.0;
    text.max_width = Some(500.0);
    text.scale = 16;
    text.color = colors::BLACK;
    strings.push(text);
    // Assign the strings we want to draw to a batch.
    engine.render.text_set(&screen, &strings);
    engine.render.clear_color(colors::WHITE);

    move |event, engine| match event {
        InputMessage::ReceivedCharacter(char) => {
            message.push(char);
            let mut strings = Vec::new();
            let mut text = Text::default();
            text.set_string(&message);
            text.pos.x = 100.0;
            text.pos.y = 500.0;
            text.max_width = Some(500.0);
            text.scale = 16;
            text.color = colors::BLACK;
            strings.push(text);
            engine.render.text_set(&screen, &strings);
        }
        InputMessage::CloseRequested => engine.stop(),
        InputMessage::KeyPressed(key) => match key {
            KeyboardButton::Escape => engine.stop(),
            KeyboardButton::Tab => {
                screen_settings.scale = 1.0;
                engine.render.batch_update(&screen, &screen_settings);
            }
            _ => {}
        },
        InputMessage::CursorPressed {
            button,
            ..
        } => match button {
            CursorButton::Left => is_dragging = true,
            _ => {}
        },
        InputMessage::CursorReleased {
            button,
            ..
        } => match button {
            CursorButton::Left => is_dragging = false,
            _ => {}
        },
        InputMessage::CursorMoved {
            delta,
            ..
        } => {
            if is_dragging {
                screen_settings.translation += delta / screen_settings.scale;
                engine.render.batch_update(&screen, &screen_settings);
            }
        }
        InputMessage::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => screen_settings.scale *= 1.1,
                ScrollDirection::Down => screen_settings.scale /= 1.1,
                _ => {}
            }
            engine.render.batch_update(&screen, &screen_settings);
        }
        InputMessage::Update(_delta) => {
            engine.render.draw();
        }
        _ => {}
    }
}
