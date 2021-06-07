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

    // Create a Layers to draw on.
    let mut screen = engine.layer_create();
    let mut screen_transform = LayerTransform::new();
    let mut sprites = Vec::new();
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
    engine.text_clear(&strings, &mut sprites);
    screen.set_sprites(&sprites);

    engine.clear_color(colors::WHITE);

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
            engine.text_clear(&strings, &mut sprites);
            screen.set_sprites(&sprites);
        }
        InputMessage::CloseRequested => engine.stop(),
        InputMessage::KeyPressed(key) => match key {
            KeyboardButton::Escape => engine.stop(),
            KeyboardButton::Tab => {
                screen_transform.scale = 1.0;
                screen.set_transform(&screen_transform);
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
                screen_transform.translation += delta / screen_transform.scale;
                screen.set_transform(&screen_transform);
            }
        }
        InputMessage::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => screen_transform.scale *= 1.1,
                ScrollDirection::Down => screen_transform.scale /= 1.1,
                _ => {}
            }
            screen.set_transform(&screen_transform);
        }
        InputMessage::Update(_delta) => {
            engine.clear(ClearMode::COLOR | ClearMode::DEPTH);
            screen.draw();
        }
        _ => {}
    }
}
