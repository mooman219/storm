use core::time::Duration;
use storm::fontdue::layout::{LayoutSettings, TextStyle};
use storm::fontdue::Font;
use storm::*;

static FONT: &[u8] = include_bytes!("resources/Roboto-Regular.ttf");

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
    let mut screen = ctx.text_layer();
    let mut screen_transform = LayerTransform::new();
    // Add all the strings we want to draw to a vec.
    let mut message = String::from("> Teh quick brown fox jumps over the lazy dog.");
    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let layout_settings = LayoutSettings {
        x: 100.0,
        y: 500.0,
        max_width: Some(500.0),
        ..Default::default()
    };
    screen.append(
        &fonts,
        &layout_settings,
        &[TextStyle {
            text: &message,
            font_index: 0,
            px: 16.0,
            user_data: colors::BLACK,
        }],
    );

    move |event, ctx| match event {
        Event::ReceivedCharacter(char) => {
            message.push(char);
            screen.clear_text();
            screen.append(
                &fonts,
                &layout_settings,
                &[TextStyle {
                    text: &message,
                    font_index: 0,
                    px: 16.0,
                    user_data: colors::BLACK,
                }],
            );
        }
        Event::CloseRequested => ctx.stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Escape => ctx.stop(),
            KeyboardButton::Tab => {
                screen_transform.scale = 1.0;
                screen.set_transform(screen_transform.matrix());
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
                screen.set_transform(screen_transform.matrix());
            }
        }
        Event::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => screen_transform.scale *= 1.1,
                ScrollDirection::Down => screen_transform.scale /= 1.1,
                _ => {}
            }
            screen.set_transform(screen_transform.matrix());
        }
        Event::Update(_delta) => {
            ctx.clear(ClearMode::color_depth(colors::WHITE));
            screen.draw();
        }
        _ => {}
    }
}
