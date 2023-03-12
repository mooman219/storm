use core::convert::{From, Into};
use core::time::Duration;
use storm::audio::*;
use storm::cgmath::{Vector2, Vector3};
use storm::color::RGBA8;
use storm::event::*;
use storm::fontdue::{layout::LayoutSettings, Font};
use storm::graphics::{
    shaders::{sprite::*, text::*},
    Buffer, ClearMode, DepthTest, DisplayMode, Shader, Texture, Uniform, Vsync, WindowSettings,
};
use storm::math::{OrthographicCamera, AABB2D};
use storm::*;

static SOUND: &[u8] = include_bytes!("resources/boop.flac");
static FONT: &[u8] = include_bytes!("resources/Roboto-Regular.ttf");

/// Run with: cargo run --example pong --release
fn main() {
    // Create the engine context and describe the window.
    start::<PongApp>(WindowSettings {
        title: String::from("Video Game"),
        display_mode: DisplayMode::Windowed {
            width: 1280,
            height: 1024,
            resizable: true,
        },
        vsync: Vsync::Disabled,
    });
}

const SPEED: f32 = 250.0;

struct PongApp {
    text_shader: Shader,
    sprite_shader: Shader,
    default_texture: Texture,
    background: Buffer<Sprite>,
    paddles: Buffer<Sprite>,
    ball: Buffer<Sprite>,
    transform_uniform: Uniform,
    boop: Sound,
    text_layer: TextShaderPass,
    up: bool,
    down: bool,
    paddle_speed: [f32; 2],
    paddle_sprites: [Sprite; 2],
    ball_speed: Vector3<f32>,
    ball_sprites: [Sprite; 1],
}

impl App for PongApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));

        let text_shader = Shader::new(ctx, TEXT_SHADER);
        let sprite_shader = Shader::new(ctx, SPRITE_SHADER);
        let default_texture = ctx.default_texture();

        let mut background = Buffer::new(ctx);
        let mut paddles = Buffer::new(ctx);
        let mut ball = Buffer::new(ctx);

        let mut transform = OrthographicCamera::new(ctx.window_logical_size());
        let transform_uniform = Uniform::new(ctx, transform.matrix());

        let boop = Sound::from_bytes(SOUND).unwrap();

        let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
        let mut text_layer = TextShaderPass::new(ctx, transform.matrix());
        let layout_settings = LayoutSettings {
            x: 100.0,
            y: 500.0,
            max_width: Some(500.0),
            ..Default::default()
        };
        let message = String::from("This is a test.\nNew line.");
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

        background.set_data(&[Sprite {
            pos: Vector3::new(-500.0, -400.0, -0.1),
            size: Vector2::new(1000, 800),
            color: RGBA8::new(15, 15, 15, 255),
            ..Default::default()
        }]);

        let up = false;
        let down = false;
        let paddle_speed = [0.0f32; 2];
        let paddle_sprites = [
            Sprite {
                pos: Vector3::new(-500.0, -60.0, 0.0),
                size: Vector2::new(30, 120),
                color: RGBA8::WHITE,
                ..Default::default()
            },
            Sprite {
                pos: Vector3::new(500.0 - 30.0, -60.0, 0.0),
                size: Vector2::new(30, 120),
                color: RGBA8::WHITE,
                ..Default::default()
            },
        ];
        paddles.set_data(&paddle_sprites);

        let ball_speed = Vector3::new(-300.0, 0.0, 0.0);
        let ball_sprites = [Sprite {
            pos: Vector3::new(-12.0, -12.0, 0.0),
            size: Vector2::new(25, 25),
            color: RGBA8::WHITE,
            ..Default::default()
        }];
        ball.set_data(&ball_sprites);

        PongApp {
            text_shader,
            sprite_shader,
            default_texture,
            background,
            paddles,
            ball,
            transform_uniform,
            boop,
            text_layer,
            up,
            down,
            paddle_speed,
            paddle_sprites,
            ball_speed,
            ball_sprites,
        }
    }

    fn on_update(&mut self, ctx: &mut Context<Self>, delta: f32) {
        ctx.clear(ClearMode::new().with_color(RGBA8::BLACK).with_depth(1.0, DepthTest::Less));
        self.paddle_sprites[0].pos.y += self.paddle_speed[0] * delta;

        let mut ball_aabb: AABB2D = self.ball_sprites[0].into();
        if ball_aabb.slide(
            &(self.ball_speed * delta).truncate(),
            &[self.paddle_sprites[0].into(), self.paddle_sprites[1].into()],
        ) {
            self.ball_speed *= -1.0;
            let _ = self.boop.play(ctx, 0.4, 0.0);
        }
        self.ball_sprites[0].pos = Vector3::new(ball_aabb.min.x, ball_aabb.min.y, 0.0);
        if self.ball_sprites[0].pos.x < -500.0 || self.ball_sprites[0].pos.x > 500.0 - 30.0 {
            self.ball_sprites[0].pos = Vector3::new(-15.0, -15.0, 0.0);
            self.ball_speed = Vector3::new(-700.0, 0.0, 0.0);
        }

        self.ball.set_data(&self.ball_sprites);
        self.paddles.set_data(&self.paddle_sprites);

        self.sprite_shader.bind(&[&self.transform_uniform], &[&self.default_texture]);
        self.paddles.draw();
        self.background.draw();
        self.ball.draw();
        ctx.clear(ClearMode::new().with_depth(1.0, DepthTest::Less));
        self.text_layer.draw(&self.text_shader);
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Up => {
                if !self.up {
                    self.paddle_speed[0] += SPEED;
                    self.up = true;
                }
            }
            KeyboardButton::Down => {
                if !self.down {
                    self.paddle_speed[0] -= SPEED;
                    self.down = true;
                }
            }
            KeyboardButton::Escape => ctx.request_stop(),
            _ => {}
        }
    }

    fn on_key_released(&mut self, _ctx: &mut Context<Self>, key: event::KeyboardButton) {
        match key {
            KeyboardButton::Up => {
                if self.up {
                    self.paddle_speed[0] -= SPEED;
                    self.up = false;
                }
            }
            KeyboardButton::Down => {
                if self.down {
                    self.paddle_speed[0] += SPEED;
                    self.down = false;
                }
            }
            _ => {}
        }
    }
}
