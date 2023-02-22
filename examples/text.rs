use core::time::Duration;
use fontdue::layout::VerticalAlign;
use storm::color::RGBA8;
use storm::event::*;
use storm::fontdue::{layout::LayoutSettings, Font};
use storm::graphics::{shaders::text::*, ClearMode, DepthTest, DisplayMode, Shader, Vsync, WindowSettings};
use storm::math::OrthographicCamera;
use storm::*;

static FONT: &[u8] = include_bytes!("resources/arial.ttf");

/// Run with: cargo run --example text --release
fn main() {
    // Create the engine context and describe the window.
    start::<TextApp>(WindowSettings {
        title: String::from("Storm: Text"),
        display_mode: DisplayMode::Windowed {
            width: 1280,
            height: 1024,
            resizable: true,
        },
        vsync: Vsync::Disabled,
    });
}

const SIZE: f32 = 40.0;
const COLOR: RGBA8 = RGBA8::WHITE;

struct TextApp {
    is_dragging: bool,
    transform: OrthographicCamera,
    text_shader: Shader<TextShader>,
    text_layer: TextShaderPass,
    fonts: [Font; 1],
    layout_settings: LayoutSettings,
    message: String,
}

impl App for TextApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        let is_dragging = false;
        let mut transform = OrthographicCamera::new(ctx.window_logical_size());
        let text_shader = Shader::new(ctx);

        // Create a Layers to draw on.
        let mut text_layer = TextShaderPass::new(ctx, transform.matrix());

        // Setup the layout for our text.
        let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
        let layout_settings = LayoutSettings {
            x: 100.0,
            y: 500.0,
            max_width: Some(500.0),
            max_height: Some(500.0),
            line_height: 0.5,
            vertical_align: VerticalAlign::Middle,
            ..Default::default()
        };

        // Append some text with our layout settings.
        let message = String::from("Fontdue on web");
        text_layer.append(
            &fonts,
            &layout_settings,
            &[Text {
                text: &message,
                font_index: 0,
                px: SIZE,
                color: COLOR,
                depth: 0.0,
            }],
        );

        TextApp {
            is_dragging,
            transform,
            text_shader,
            text_layer,
            fonts,
            layout_settings,
            message,
        }
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, _delta: f32) {
        ctx.clear(ClearMode::new().with_color(RGBA8::BLACK).with_depth(1.0, DepthTest::Less));
        self.text_layer.draw(&self.text_shader);
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_received_character(&mut self, _ctx: &mut Context<Self>, character: char) {
        // Backspace
        if character == '\u{08}' {
            return;
        }
        self.message.push(character);
        self.text_layer.clear_text();
        self.text_layer.append(
            &self.fonts,
            &self.layout_settings,
            &[Text {
                text: &self.message,
                font_index: 0,
                px: SIZE,
                color: COLOR,
                depth: 0.0,
            }],
        );
    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            KeyboardButton::Tab => {
                self.transform.set().scale = 1.0;
                self.text_layer.set_ortho(self.transform.matrix());
            }
            KeyboardButton::Back => {
                self.message.pop();
                self.text_layer.clear_text();
                self.text_layer.append(
                    &self.fonts,
                    &self.layout_settings,
                    &[Text {
                        text: &self.message,
                        font_index: 0,
                        px: SIZE,
                        color: COLOR,
                        depth: 0.0,
                    }],
                );
            }
            _ => {}
        }
    }

    fn on_cursor_pressed(
        &mut self,
        _ctx: &mut Context<Self>,
        button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
        match button {
            CursorButton::Left => self.is_dragging = true,
            _ => {}
        }
    }

    fn on_cursor_released(
        &mut self,
        _ctx: &mut Context<Self>,
        button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
        match button {
            CursorButton::Left => self.is_dragging = false,
            _ => {}
        }
    }

    fn on_cursor_delta(&mut self, _ctx: &mut Context<Self>, delta: cgmath::Vector2<f32>, _focused: bool) {
        if self.is_dragging {
            let scale = self.transform.get().scale;
            self.transform.set().translation += delta.extend(0.0) / scale;
            self.text_layer.set_ortho(self.transform.matrix());
        }
    }

    fn on_cursor_scroll(&mut self, _ctx: &mut Context<Self>, direction: event::ScrollDirection) {
        match direction {
            ScrollDirection::Up => self.transform.set().scale *= 1.1,
            ScrollDirection::Down => self.transform.set().scale /= 1.1,
            _ => {}
        }
        self.text_layer.set_ortho(self.transform.matrix());
    }

    fn on_window_resized(
        &mut self,
        _ctx: &mut Context<Self>,
        _physical_size: cgmath::Vector2<f32>,
        logical_size: cgmath::Vector2<f32>,
        _scale_factor: f32,
    ) {
        self.transform.set_size(logical_size);
        self.text_layer.set_ortho(self.transform.matrix());
    }
}
