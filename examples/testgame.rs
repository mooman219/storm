extern crate log;
extern crate storm;

mod logger;

use cgmath::prelude::*;
use cgmath::*;
use log::LevelFilter;
use logger::*;
use storm::color::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example testgame --release
fn main() {
    SimpleLogger::init(LevelFilter::Trace);

    let mut clock = Clock::new(144);
    let mut engine = Engine::new();

    let screen = engine.batch_create(&BatchDescription::default());
    let mut sprites = Vec::new();
    let mut particles = Vec::new();
    for x in -250..250 {
        for y in -250..250 {
            let (sprite, particle) = Particle::new(Vector3::new(x as f32 * 2.0, y as f32 * 2.0, 0.0));
            sprites.push(sprite);
            particles.push(particle);
        }
    }

    let mut is_active = true;
    while is_active {
        engine.input_poll(|message| match message {
            InputMessage::CloseRequested => is_active = false,
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => is_active = false,
                _ => {},
            },
            _ => {},
        });

        let delta = clock.get_delta();
        for index in 0..sprites.len() {
            Particle::tick(&mut sprites[index], &mut particles[index], delta);
        }

        engine.sprite_set(&screen, &sprites);
        engine.window_commit();
        clock.tick();
    }
}

pub struct Particle {
    pos: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl Particle {
    // Play with these values to alter the way gravity works.
    const G: f32 = 6.674;
    const MASS: f32 = 200.0;

    pub fn new(pos: Vector3<f32>) -> (SpriteDescription, Particle) {
        let sprite = SpriteDescription::new(pos, Vector2::new(2.0, 2.0), DEFAULT_TEXTURE, BLACK, 0.0);
        let particle = Particle {
            pos: pos.truncate(),
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),
        };
        (sprite, particle)
    }

    pub fn tick(sprite: &mut SpriteDescription, particle: &mut Particle, delta: f32) {
        let length_squared = particle.pos.x * particle.pos.x + particle.pos.y * particle.pos.y;
        let length = f32::sqrt(length_squared);
        let norm = particle.pos / length;
        let norm_squared = particle.pos / length_squared;
        particle.acceleration = -(norm * (Self::G * Self::MASS)).div_element_wise(norm_squared);
        particle.velocity += particle.acceleration;
        particle.pos += particle.velocity;

        let velocity = particle.velocity + particle.acceleration * delta;
        let position = particle.pos + velocity * delta;
        sprite.set_pos(position.extend(0.0));
    }
}
