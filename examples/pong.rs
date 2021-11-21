use core::convert::{From, Into};
use core::time::Duration;
use storm::cgmath::{Vector2, Vector3};
use storm::math::AABB2D;
use storm::*;

static SOUND: &[u8] = include_bytes!("resources/boop.flac");
// static FONT: &[u8] = include_bytes!("../resources/Roboto-Regular.ttf");

/// Run with: cargo run --example square --release
fn main() {
    // Create the engine context and describe the window.
    Context::start(
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

fn run(ctx: &mut Context) -> impl FnMut(Event, &mut Context) {
    ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));

    let boop = ctx.load_flac(SOUND).unwrap();

    let mut background = ctx.sprite_layer();
    background.set_sprites(&[Sprite {
        pos: Vector3::new(-500.0, -400.0, -0.1),
        size: Vector2::new(1000, 800),
        color: RGBA8::new(15, 15, 15, 255),
        ..Default::default()
    }]);

    let mut up = false;
    let mut down = false;
    let mut paddles = ctx.sprite_layer();
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
    paddles.set_sprites(&paddle_sprites);

    let mut ball = ctx.sprite_layer();
    let mut ball_speed = Vector3::new(-300.0, 0.0, 0.0);
    let mut ball_sprites = [Sprite {
        pos: Vector3::new(-15.0, -15.0, 0.0),
        size: Vector2::new(30, 30),
        color: RGBA8::WHITE,
        ..Default::default()
    }];
    ball.set_sprites(&ball_sprites);

    const SPEED: f32 = 200.0;
    move |event, ctx| match event {
        Event::CloseRequested => ctx.stop(),
        Event::KeyPressed(key) => match key {
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
            KeyboardButton::Escape => ctx.stop(),
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
            KeyboardButton::Escape => ctx.stop(),
            _ => {}
        },
        Event::Update(delta) => {
            ctx.clear(ClearMode::color_depth(RGBA8::BLACK));
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

            ball.set_sprites(&ball_sprites);
            paddles.set_sprites(&paddle_sprites);

            paddles.draw();
            background.draw();
            ball.draw();
        }
        _ => {}
    }
}
