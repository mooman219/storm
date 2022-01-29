use cgmath::{Vector2, Vector3};
use core::time::Duration;
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
        new,
    );
}

struct TextureApp {
    sprite_shader: SpriteShader,
    texture_atlas: Texture,
    sprite_buffer: Buffer<Sprite>,
    transform: OrthographicCamera,
    transform_uniform: Uniform<SpriteUniform>,
    sound: SoundControl,
    sprites: [Sprite; 3],
    clicking: bool,
}

fn new() -> impl App {
    wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));

    let sprite_shader = SpriteShader::new();
    let texture_atlas = Texture::from_png(TEXTURE_A, TextureFiltering::NONE);
    let mut sprite_buffer = Buffer::new();

    let mut transform = OrthographicCamera::new(window_logical_size());
    transform.set().rotation = 0.12;

    let transform_uniform = Uniform::new(&mut transform);

    let source = Sound::from_flac(SOUND).unwrap();
    let sound = source.play(0.3, 0.1);

    let sprites = [
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

    let clicking = false;

    TextureApp {
        sprite_shader,
        texture_atlas,
        sprite_buffer,
        transform,
        transform_uniform,
        sound,
        sprites,
        clicking,
    }
}

impl App for TextureApp {
    fn on_update(&mut self, _delta: f32) {
        clear(ClearMode::color_depth(RGBA8::BLUE));
        self.sprite_shader.draw(&self.transform_uniform, &self.texture_atlas, &[&self.sprite_buffer]);
    }

    fn on_close_requested(&mut self) {
        request_stop();
    }

    fn on_key_pressed(&mut self, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Escape => request_stop(),
            KeyboardButton::P => self.sound.pause(),
            KeyboardButton::R => self.sound.resume(),
            KeyboardButton::Q => storm::asset::request_read("./docs/load.png"),
            KeyboardButton::A => storm::asset::request_read("./load.png"),
            _ => {}
        }
    }

    fn on_cursor_pressed(
        &mut self,
        _button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        normalized_pos: cgmath::Vector2<f32>,
    ) {
        let pos = self.transform.screen_to_world(normalized_pos);
        if pos.x >= self.sprites[1].pos.x
            && pos.x <= self.sprites[1].pos.x + self.sprites[1].size.x as f32
            && pos.y >= self.sprites[1].pos.y
            && pos.y <= self.sprites[1].pos.y + self.sprites[1].size.y as f32
        {
            self.clicking = true;
        }
    }

    fn on_cursor_released(
        &mut self,
        _button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
        self.clicking = false;
    }

    fn on_cursor_moved(&mut self, _physical_pos: cgmath::Vector2<f32>, normalized_pos: cgmath::Vector2<f32>) {
        let pos = self.transform.screen_to_world(normalized_pos);
        let mut x = pos.x - 12.0;
        if self.clicking {
            if x < -200.0 {
                x = -200.0;
            } else if x > 175.0 {
                x = 175.0
            }
            let volume = (x + 200.0) / 375.0;
            self.sound.set_volume(volume, 0.01);
            self.sprites[1].pos.x = x;
            self.sprite_buffer.set(&self.sprites);
        }
    }

    fn on_window_resized(
        &mut self,
        _physical_size: cgmath::Vector2<f32>,
        logical_size: cgmath::Vector2<f32>,
        _scale_factor: f32,
    ) {
        self.transform.set_size(logical_size);
        self.transform_uniform.set(&mut self.transform);
    }

    // Event::AssetRead(asset) => match asset.result {
    //     Ok(contents) => {
    //         info!("Loaded {}: {}", asset.relative_path, contents[1]);
    //     }
    //     Err(error) => warn!("Error {}: {:?}", asset.relative_path, error),
    // },
}
