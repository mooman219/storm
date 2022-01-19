use core::convert::{From, Into};
use core::time::Duration;
use storm::audio::*;
use storm::cgmath::{Vector2, Vector3};
use storm::color::RGBA8;
use storm::event::*;
use storm::fontdue::{layout::LayoutSettings, Font};
use storm::graphics::Buffer;
use storm::graphics::{
    clear, default_texture,
    shaders::{sprite::*, text::*},
    window_logical_size, ClearMode, DisplayMode, Uniform, Vsync, WindowSettings,
};
use storm::math::{OrthographicCamera, AABB2D};
use storm::*;

static SOUND: &[u8] = include_bytes!("resources/boop.flac");
static FONT: &[u8] = include_bytes!("resources/Roboto-Regular.ttf");

/// Run with: cargo run --example pong --release
fn main() {
    // Create the engine context and describe the window.
    start(
        WindowSettings {
            title: String::from("Video Game"),
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

    let text_shader = TextShader::new();
    let sprite_shader = SpriteShader::new();
    let default_texture = default_texture();

    let mut background = Buffer::new();
    let mut paddles = Buffer::new();
    let mut ball = Buffer::new();

    let mut transform = OrthographicCamera::new(window_logical_size());
    let transform_uniform = Uniform::new(&mut transform);

    let boop = Sound::from_flac(SOUND).unwrap();

    let fonts = [Font::from_bytes(FONT, Default::default()).unwrap()];
    let mut text_layer = TextShaderPass::new(transform.matrix());
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

    background.set(&[Sprite {
        pos: Vector3::new(-500.0, -400.0, -0.1),
        size: Vector2::new(1000, 800),
        color: RGBA8::new(15, 15, 15, 255),
        ..Default::default()
    }]);

    let mut up = false;
    let mut down = false;
    let mut paddle_speed = [0.0f32; 2];
    let mut paddle_sprites = [
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
    paddles.set(&paddle_sprites);

    let mut ball_speed = Vector3::new(-300.0, 0.0, 0.0);
    let mut ball_sprites = [Sprite {
        pos: Vector3::new(-12.0, -12.0, 0.0),
        size: Vector2::new(25, 25),
        color: RGBA8::WHITE,
        ..Default::default()
    }];
    ball.set(&ball_sprites);

    const SPEED: f32 = 250.0;
    move |event| match event {
        Event::CloseRequested => request_stop(),
        Event::KeyPressed {
            keycode,
            ..
        } => match keycode {
            KeyboardButton::Up => {
                if !up {
                    paddle_speed[0] += SPEED;
                    up = true;
                }
            }
            KeyboardButton::Down => {
                if !down {
                    paddle_speed[0] -= SPEED;
                    down = true;
                }
            }
            KeyboardButton::Escape => request_stop(),
            _ => {}
        },
        Event::KeyReleased(key) => match key {
            KeyboardButton::Up => {
                if up {
                    paddle_speed[0] -= SPEED;
                    up = false;
                }
            }
            KeyboardButton::Down => {
                if down {
                    paddle_speed[0] += SPEED;
                    down = false;
                }
            }
            _ => {}
        },
        Event::Update(delta) => {
            clear(ClearMode::color_depth(RGBA8::BLACK));
            paddle_sprites[0].pos.y += paddle_speed[0] * delta;

            let mut ball_aabb: AABB2D = ball_sprites[0].into();
            if ball_aabb.slide(
                &(ball_speed * delta).truncate(),
                &[paddle_sprites[0].into(), paddle_sprites[1].into()],
            ) {
                ball_speed *= -1.0;
                let _ = boop.play(0.4, 0.0);
            }
            ball_sprites[0].pos = Vector3::new(ball_aabb.min.x, ball_aabb.min.y, 0.0);
            if ball_sprites[0].pos.x < -500.0 || ball_sprites[0].pos.x > 500.0 - 30.0 {
                ball_sprites[0].pos = Vector3::new(-15.0, -15.0, 0.0);
                ball_speed = Vector3::new(-700.0, 0.0, 0.0);
            }

            ball.set(&ball_sprites);
            paddles.set(&paddle_sprites);

            sprite_shader.draw(&transform_uniform, &default_texture, &[&paddles, &background, &ball]);
            clear(ClearMode::depth());
            text_layer.draw(&text_shader);
        }
        _ => {}
    }
}
