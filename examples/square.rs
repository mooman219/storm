use core::time::Duration;
use storm::*;

/// Run with: cargo run --example square --release
fn main() {
    simple_logger::SimpleLogger::new().init().expect("Unable to init logger");
    // Create the engine context and describe the window.
    Context::start(
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

fn run(ctx: &mut Context) -> impl FnMut(Event, &mut Context) {
    ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
    let mut is_dragging = false;

    // Create a Layers to draw on.
    let mut screen = ctx.layer_sprite();
    screen.clear().set(Some(ClearMode::color_depth(colors::WHITE)));
    let mut screen_transform = LayerTransform::new();
    let mut sprites = Vec::new();
    // Add all the strings we want to draw to a vec.
    let mut message = String::from("> Teh quick brown fox jumps over the lazy dog.");
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
    ctx.text_clear(&strings, &mut sprites);
    screen.set_sprites(&sprites);

    move |event, engine| match event {
        Event::ReceivedCharacter(char) => {
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
        Event::CloseRequested => engine.stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Escape => engine.stop(),
            KeyboardButton::Tab => {
                screen_transform.scale = 1.0;
                screen.transform().set(screen_transform.matrix());
            }
            _ => {}
        },
        Event::CursorPressed {
            button,
            ..
        } => match button {
            CursorButton::Left => is_dragging = true,
            _ => {}
        },
        Event::CursorReleased {
            button,
            ..
        } => match button {
            CursorButton::Left => is_dragging = false,
            _ => {}
        },
        Event::CursorMoved {
            delta,
            ..
        } => {
            if is_dragging {
                screen_transform.translation += delta / screen_transform.scale;
                screen.transform().set(screen_transform.matrix());
            }
        }
        Event::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => screen_transform.scale *= 1.1,
                ScrollDirection::Down => screen_transform.scale /= 1.1,
                _ => {}
            }
            screen.transform().set(screen_transform.matrix());
        }
        Event::Update(_delta) => {
            screen.draw();
        }
        _ => {}
    }
}
