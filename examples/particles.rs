use core::time::Duration;
use storm::cgmath::prelude::*;
use storm::cgmath::*;
use storm::color::RGBA8;
use storm::event::*;
use storm::graphics::{
    shaders::sprite::*, std140, ClearMode, DepthTest, DisplayMode, Texture, TextureSection, Vsync,
    WindowSettings,
};
use storm::graphics::{Buffer, Uniform};
use storm::math::OrthographicCamera;
use storm::*;

/// Run with: cargo run --example particles --release
fn main() {
    start::<ParticlesApp>(WindowSettings {
        title: String::from("Storm: Particles"),
        display_mode: DisplayMode::Windowed {
            width: 1280,
            height: 1024,
            resizable: true,
        },
        vsync: Vsync::Disabled,
    });
}

struct ParticlesApp {
    is_dragging: bool,
    sprite_shader: SpriteShader,
    particle_buffer: Buffer<Sprite>,
    default_texture: Texture,
    transform: OrthographicCamera,
    transform_uniform: Uniform<std140::mat4>,
    sprites: Vec<Sprite>,
    particles: Vec<Particle>,
}

impl App for ParticlesApp {
    fn new(ctx: &mut Context<Self>) -> Self {
        ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
        let is_dragging = false;

        let sprite_shader = SpriteShader::new(ctx);
        let mut particle_buffer = Buffer::new(ctx);
        let default_texture = ctx.default_texture();

        let mut transform = OrthographicCamera::new(ctx.window_logical_size());
        transform.set().rotation = 0.125;
        let transform_uniform: Uniform<std140::mat4> = Uniform::new(ctx, transform.matrix());

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

    fn on_update(&mut self, ctx: &mut Context<Self>, _delta: f32) {
        for index in 0..self.sprites.len() {
            Particle::tick(&mut self.sprites[index], &mut self.particles[index], 1.0 / 144.0);
        }
        self.particle_buffer.set(&self.sprites);
        ctx.clear(ClearMode::new().with_color(RGBA8::BLACK).with_depth(1.0, DepthTest::Less));
        self.sprite_shader.draw(&self.transform_uniform, &self.default_texture, &[&self.particle_buffer]);
    }

    fn on_close_requested(&mut self, ctx: &mut Context<Self>) {
        ctx.request_stop();
    }

    fn on_key_pressed(&mut self, ctx: &mut Context<Self>, key: event::KeyboardButton, _is_repeat: bool) {
        match key {
            KeyboardButton::Escape => ctx.request_stop(),
            KeyboardButton::Left => {
                self.transform.set().rotation += 0.005;
                self.transform_uniform.set(self.transform.matrix());
            }
            KeyboardButton::Right => {
                self.transform.set().rotation -= 0.005;
                self.transform_uniform.set(self.transform.matrix());
            }
            KeyboardButton::U => ctx.set_window_display_mode(DisplayMode::Windowed {
                width: 1500,
                height: 1000,
                resizable: true,
            }),
            KeyboardButton::I => ctx.set_window_display_mode(DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            }),
            KeyboardButton::O => ctx.set_window_display_mode(DisplayMode::WindowedFullscreen),
            KeyboardButton::P => ctx.set_window_display_mode(DisplayMode::Fullscreen),
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
            self.transform_uniform.set(self.transform.matrix());
        }
    }

    fn on_cursor_scroll(&mut self, _ctx: &mut Context<Self>, direction: event::ScrollDirection) {
        match direction {
            ScrollDirection::Up => self.transform.set().scale *= 1.1,
            ScrollDirection::Down => self.transform.set().scale /= 1.1,
            _ => {}
        }
        self.transform_uniform.set(self.transform.matrix());
    }

    fn on_window_resized(
        &mut self,
        _ctx: &mut Context<Self>,
        _physical_size: cgmath::Vector2<f32>,
        logical_size: cgmath::Vector2<f32>,
        _scale_factor: f32,
    ) {
        self.transform.set_size(logical_size);
        self.transform_uniform.set(self.transform.matrix());
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
