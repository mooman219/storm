use core::time::Duration;
use storm::cgmath::prelude::*;
use storm::cgmath::*;
use storm::color::RGBA8;
use storm::event::*;
use storm::graphics::{
    clear, set_window_display_mode, shaders::sprite::*, window_logical_size, ClearMode, DisplayMode,
    TextureSection, Vsync, WindowSettings,
};
use storm::graphics::{default_texture, Buffer, Uniform};
use storm::math::OrthographicCamera;
use storm::*;

/// Run with: cargo run --example particles --release
fn main() {
    start(
        WindowSettings {
            title: String::from("Storm: Particles"),
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

    let sprite_shader = SpriteShader::new();
    let mut particle_buffer = Buffer::new();
    let default_texture = default_texture();

    let mut transform = OrthographicCamera::new(window_logical_size());
    transform.set().rotation = 0.125;
    let mut transform_uniform: Uniform<SpriteUniform> = Uniform::new(&mut transform);

    let mut sprites = Vec::new();
    let mut particles = Vec::new();
    const RANGE: i32 = 100;
    for x in -RANGE..RANGE {
        for y in -RANGE..RANGE {
            let (sprite, particle) = Particle::new(Vector3::new(x as f32 * 5.0, y as f32 * 5.0, 0.0));
            sprites.push(sprite);
            particles.push(particle);
        }
    }
    particle_buffer.set(&sprites);

    move |event| match event {
        Event::CloseRequested => request_stop(),
        Event::KeyPressed {
            keycode,
            ..
        } => match keycode {
            KeyboardButton::Escape => request_stop(),
            KeyboardButton::Left => {
                transform.set().rotation += 0.005;
                transform_uniform.set(&mut transform);
            }
            KeyboardButton::Right => {
                transform.set().rotation -= 0.005;
                transform_uniform.set(&mut transform);
            }
            KeyboardButton::U => set_window_display_mode(DisplayMode::Windowed {
                width: 1500,
                height: 1000,
                resizable: true,
            }),
            KeyboardButton::I => set_window_display_mode(DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            }),
            KeyboardButton::O => set_window_display_mode(DisplayMode::WindowedFullscreen),
            KeyboardButton::P => set_window_display_mode(DisplayMode::Fullscreen),
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
        } => {
            if is_dragging {
                let scale = transform.get().scale;
                transform.set().translation += delta.extend(0.0) / scale;
                transform_uniform.set(&mut transform);
            }
        }
        Event::WindowResized {
            logical_size,
            ..
        } => {
            transform.set_size(logical_size);
            transform_uniform.set(&mut transform);
        }
        Event::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => transform.set().scale *= 1.1,
                ScrollDirection::Down => transform.set().scale /= 1.1,
                _ => {}
            }
            transform_uniform.set(&mut transform);
        }
        Event::Update(_delta) => {
            for index in 0..sprites.len() {
                Particle::tick(&mut sprites[index], &mut particles[index], 1.0 / 144.0);
            }
            particle_buffer.set(&sprites);
            clear(ClearMode::color_depth(RGBA8::BLACK));
            sprite_shader.draw(&transform_uniform, &default_texture, &[&particle_buffer]);
        }
        _ => {}
    }
}

pub struct Particle {
    pos: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl Particle {
    const G: f32 = 10.674;
    const MASS: f32 = 500.0;

    pub fn new(pos: Vector3<f32>) -> (Sprite, Particle) {
        let sprite = Sprite::new(pos, Vector2::new(2.0, 2.0), TextureSection::full(), RGBA8::WHITE, 0.0);
        let velocity = if pos.y < 0.0 {
            Vector2::new(20.0, 0.0)
        } else {
            Vector2::new(-20.0, 0.0)
        };
        let particle = Particle {
            pos: pos.truncate(),
            velocity: velocity,
            acceleration: Vector2::zero(),
        };
        (sprite, particle)
    }

    pub fn tick(sprite: &mut Sprite, particle: &mut Particle, delta: f32) {
        let length_squared = particle.pos.x * particle.pos.x + particle.pos.y * particle.pos.y;
        let length = f32::sqrt(length_squared);
        let norm = particle.pos / length;
        particle.acceleration = -(norm * (Self::G * Self::MASS)) / length_squared.max(1000.0);

        particle.velocity += particle.acceleration;
        particle.pos += particle.velocity * delta;
        sprite.pos = particle.pos.extend(0.0);
    }
}
