use core::time::Duration;
use storm::color::RGBA8;
use storm::event::*;
use storm::fontdue::{layout::LayoutSettings, Font};
use storm::graphics::{
    clear, shaders::text::*, window_logical_size, ClearMode, DisplayMode, Vsync, WindowSettings,
};
use storm::math::OrthographicCamera;
use storm::*;

static FONT: &[u8] = include_bytes!("resources/Roboto-Regular.ttf");

/// Run with: cargo run --example text --release
fn main() {
    // Create the engine context and describe the window.
    start(
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

fn run() -> impl FnMut(Event) {
    wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
    let mut is_dragging = false;
    let mut transform = OrthographicCamera::new(window_logical_size());
    let text_shader = TextShader::new();

    // Create a Layers to draw on.
    let mut text_layer = TextShaderPass::new(transform.matrix());

    // Setup the layout for our text.
    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let layout_settings = LayoutSettings {
        x: 100.0,
        y: 500.0,
        max_width: Some(500.0),
        ..Default::default()
    };

    // Append some text with our layout settings.
    const SIZE: f32 = 100.0;
    let mut message = String::from("Nice\nPost");
    text_layer.append(
        &fonts,
        &layout_settings,
        &[Text {
            text: &message,
            font_index: 0,
            px: SIZE,
            color: RGBA8::WHITE,
            depth: 0.0,
        }],
    );

    move |event| match event {
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
                    px: SIZE,
                    color: RGBA8::WHITE,
                    depth: 0.0,
                }],
            );
        }
        Event::CloseRequested => request_stop(),
        Event::KeyPressed {
            keycode,
            ..
        } => match keycode {
            KeyboardButton::Escape => request_stop(),
            KeyboardButton::Tab => {
                transform.set().scale = 1.0;
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
                        px: SIZE,
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
        Event::CursorDelta {
            delta,
            ..
        } => {
            if is_dragging {
                let scale = transform.get().scale;
                transform.set().translation += delta.extend(0.0) / scale;
                text_layer.set_ortho(transform.matrix());
            }
        }
        Event::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => transform.set().scale *= 1.1,
                ScrollDirection::Down => transform.set().scale /= 1.1,
                _ => {}
            }
            text_layer.set_ortho(transform.matrix());
        }
        Event::WindowResized {
            logical_size,
            ..
        } => {
            transform.set_size(logical_size);
            text_layer.set_ortho(transform.matrix());
        }
        Event::Update(_delta) => {
            clear(ClearMode::color_depth(RGBA8::BLACK));
            text_layer.draw(&text_shader);
        }
        _ => {}
    }
}
