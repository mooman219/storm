use cgmath::{Vector2, Vector3};
use core::time::Duration;
use log::{info, warn};
use storm::audio::*;
use storm::color::RGBA8;
use storm::event::*;
use storm::graphics::Buffer;
use storm::graphics::{
    clear, shaders::sprite::*, window_logical_size, ClearMode, DisplayMode, Texture, TextureFiltering,
    Uniform, Vsync, WindowSettings,
};
use storm::math::OrthographicCamera;
use storm::*;

static TEXTURE_A: &[u8] = include_bytes!("resources/3.png");
static SOUND: &[u8] = include_bytes!("resources/boop.flac");

/// Run with: cargo run --example texture --release
fn main() {
    start(
        WindowSettings {
            title: String::from("Storm: Texture"),
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

    let sprite_shader = SpriteShader::new();
    let texture_atlas = Texture::from_png(TEXTURE_A, TextureFiltering::NONE);
    let mut sprite_buffer = Buffer::new();

    let mut transform = OrthographicCamera::new(window_logical_size());
    transform.set().rotation = 0.12;

    let mut transform_uniform = Uniform::new(&mut transform);

    let source = Sound::from_flac(SOUND).unwrap();
    let sound = source.play(0.3, 0.1);

    let mut sprites = [
        Sprite::default(),
        Sprite {
            pos: Vector3::new(-200.0, -62.0, 0.0),
            size: Vector2::new(25, 25),
            color: RGBA8::WHITE,
            ..Sprite::default()
        },
        Sprite {
            pos: Vector3::new(-200.0, -50.0, 0.0),
            size: Vector2::new(400, 3),
            color: RGBA8::BLACK,
            ..Sprite::default()
        },
    ];
    sprite_buffer.set(&sprites);

    let mut clicking = false;

    move |event| match event {
        Event::CloseRequested => request_stop(),
        Event::KeyPressed {
            keycode,
            ..
        } => match keycode {
            KeyboardButton::Escape => request_stop(),
            KeyboardButton::P => sound.pause(),
            KeyboardButton::R => sound.resume(),
            KeyboardButton::Q => storm::asset::request_read("./docs/load.png"),
            KeyboardButton::A => storm::asset::request_read("./load.png"),
            _ => {}
        },
        Event::CursorPressed {
            normalized_pos,
            ..
        } => {
            let pos = transform.screen_to_world(normalized_pos);
            if pos.x >= sprites[1].pos.x
                && pos.x <= sprites[1].pos.x + sprites[1].size.x as f32
                && pos.y >= sprites[1].pos.y
                && pos.y <= sprites[1].pos.y + sprites[1].size.y as f32
            {
                clicking = true;
            }
        }
        Event::CursorReleased {
            ..
        } => {
            clicking = false;
        }
        Event::CursorMoved {
            normalized_pos,
            ..
        } => {
            let pos = transform.screen_to_world(normalized_pos);
            let mut x = pos.x - 12.0;
            if clicking {
                if x < -200.0 {
                    x = -200.0;
                } else if x > 175.0 {
                    x = 175.0
                }
                let volume = (x + 200.0) / 375.0;
                sound.set_volume(volume, 0.01);
                sprites[1].pos.x = x;
                sprite_buffer.set(&sprites);
            }
        }
        Event::WindowResized {
            logical_size,
            ..
        } => {
            transform.set_size(logical_size);
            transform_uniform.set(&mut transform);
        }
        Event::Update(_delta) => {
            clear(ClearMode::color_depth(RGBA8::BLUE));
            sprite_shader.draw(&transform_uniform, &texture_atlas, &[&sprite_buffer]);
        }
        Event::AssetRead(asset) => match asset.result {
            Ok(contents) => {
                info!("Loaded {}: {}", asset.relative_path, contents[1]);
            }
            Err(error) => warn!("Error {}: {:?}", asset.relative_path, error),
        },
        _ => {}
    }
}
