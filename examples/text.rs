use core::time::Duration;
use storm::color::RGBA8;
use storm::fontdue::layout::LayoutSettings;
use storm::fontdue::Font;
use storm::graphics::shaders::text::{Text, TextShader};
use storm::*;

static FONT: &[u8] = include_bytes!("resources/Roboto-Regular.ttf");

/// Run with: cargo run --example text --release
fn main() {
    // Create the engine context and describe the window.
    Context::start(
        WindowSettings {
            title: String::from("Storm: Text"),
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
    let mut text_shader = TextShader::new(ctx);

    // Create a Layers to draw on.
    let mut text_layer = text_shader.new_instance();

    // Setup the layout for our text.
    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let layout_settings = LayoutSettings {
        x: 100.0,
        y: 500.0,
        max_width: Some(500.0),
        ..Default::default()
    };

    // Append some text with our layout settings.
    let mut message = String::from("abcdefghijklmnopqrstuvwxyz\nABCDEFGHIJKLMNOPQRSTUVWXYZ");
    text_layer.append(
        &fonts,
        &layout_settings,
        &[Text {
            text: &message,
            font_index: 0,
            px: 16.0,
            color: RGBA8::WHITE,
            depth: 0.0,
        }],
    );

    move |event, ctx| match event {
        Event::ReceivedCharacter(char) => {
            // Backspace
            if char == '\u{08}' {
                return;
            }
            message.push(char);
            text_layer.clear_text();
            text_layer.append(
                &fonts,
                &layout_settings,
                &[Text {
                    text: &message,
                    font_index: 0,
                    px: 16.0,
                    color: RGBA8::WHITE,
                    depth: 0.0,
                }],
            );
        }
        Event::CloseRequested => ctx.stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Escape => ctx.stop(),
            KeyboardButton::Tab => {
                *text_layer.transform().scale() = 1.0;
            }
            KeyboardButton::Back => {
                message.pop();
                text_layer.clear_text();
                text_layer.append(
                    &fonts,
                    &layout_settings,
                    &[Text {
                        text: &message,
                        font_index: 0,
                        px: 16.0,
                        color: RGBA8::WHITE,
                        depth: 0.0,
                    }],
                );
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
                let scale = *text_layer.transform().scale();
                *text_layer.transform().translation() += delta / scale;
            }
        }
        Event::CursorScroll(direction) => match direction {
            ScrollDirection::Up => *text_layer.transform().scale() *= 1.1,
            ScrollDirection::Down => *text_layer.transform().scale() /= 1.1,
            _ => {}
        },
        Event::WindowResized {
            logical_size,
            ..
        } => {
            *text_layer.transform().logical_size() = logical_size;
        }
        Event::Update(_delta) => {
            ctx.clear(ClearMode::color_depth(RGBA8::BLACK));
            text_layer.draw();
        }
        _ => {}
    }
}
