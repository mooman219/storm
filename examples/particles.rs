use core::time::Duration;
use storm::cgmath::prelude::*;
use storm::cgmath::*;
use storm::color::RGBA8;
use storm::event::*;
use storm::graphics::{
    clear, set_window_display_mode, shaders::sprite::*, window_logical_size, ClearMode, DisplayMode, Texture,
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
        new,
    );
}

struct ParticlesApp {
    is_dragging: bool,
    sprite_shader: SpriteShader,
    particle_buffer: Buffer<Sprite>,
    default_texture: Texture,
    transform: OrthographicCamera,
    transform_uniform: Uniform<SpriteUniform>,
    sprites: Vec<Sprite>,
    particles: Vec<Particle>,
}

fn new() -> impl App {
    wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
    let is_dragging = false;

    let sprite_shader = SpriteShader::new();
    let mut particle_buffer = Buffer::new();
    let default_texture = default_texture();

    let mut transform = OrthographicCamera::new(window_logical_size());
    transform.set().rotation = 0.125;
    let transform_uniform: Uniform<SpriteUniform> = Uniform::new(&mut transform);

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

    ParticlesApp {
        is_dragging,
        sprite_shader,
        particle_buffer,
        default_texture,
        transform,
        transform_uniform,
        sprites,
        particles,
    }
}

impl App for ParticlesApp {
    fn on_update(&mut self, _delta: f32) {
        for index in 0..self.sprites.len() {
            Particle::tick(&mut self.sprites[index], &mut self.particles[index], 1.0 / 144.0);
        }
        self.particle_buffer.set(&self.sprites);
        clear(ClearMode::color_depth(RGBA8::BLACK));
        self.sprite_shader.draw(&self.transform_uniform, &self.default_texture, &[&self.particle_buffer]);
    }

    fn on_close_requested(&mut self) {
        request_stop();
    }

    fn on_key_pressed(&mut self, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Escape => request_stop(),
            KeyboardButton::Left => {
                self.transform.set().rotation += 0.005;
                self.transform_uniform.set(&mut self.transform);
            }
            KeyboardButton::Right => {
                self.transform.set().rotation -= 0.005;
                self.transform_uniform.set(&mut self.transform);
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
        }
    }

    fn on_cursor_pressed(
        &mut self,
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
        button: event::CursorButton,
        _physical_pos: cgmath::Vector2<f32>,
        _normalized_pos: cgmath::Vector2<f32>,
    ) {
        match button {
            CursorButton::Left => self.is_dragging = false,
            _ => {}
        }
    }

    fn on_cursor_delta(&mut self, delta: cgmath::Vector2<f32>, _focused: bool) {
        if self.is_dragging {
            let scale = self.transform.get().scale;
            self.transform.set().translation += delta.extend(0.0) / scale;
            self.transform_uniform.set(&mut self.transform);
        }
    }

    fn on_cursor_scroll(&mut self, direction: event::ScrollDirection) {
        match direction {
            ScrollDirection::Up => self.transform.set().scale *= 1.1,
            ScrollDirection::Down => self.transform.set().scale /= 1.1,
            _ => {}
        }
        self.transform_uniform.set(&mut self.transform);
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
