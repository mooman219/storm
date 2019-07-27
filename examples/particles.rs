use crate::cgmath::prelude::*;
use crate::cgmath::*;
use storm::color::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example particles --release
fn main() {
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Particles"),
            display_mode: DisplayMode::Windowed {
                width: 1280,
                height: 1024,
                resizable: true,
            },
            vsync: Vsync::Enabled,
        },
        game,
    );
}

fn game(mut engine: Engine) {
    let mut clock = Clock::new(144);
    engine.window_clear_color(WHITE);

    let mut screen_settings = BatchSettings::default();
    screen_settings.rotation = 0.1;
    let screen = engine.batch_create(&screen_settings);
    let mut sprites = Vec::new();
    let mut particles = Vec::new();
    for x in -1000..1000 {
        for y in -250..250 {
            let (sprite, particle) = Particle::new(Vector3::new(x as f32 * 5.0, y as f32 * 5.0, 0.0));
            sprites.push(sprite);
            particles.push(particle);
        }
    }

    let mut is_active = true;
    let mut is_dragging = false;
    while is_active {
        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                InputMessage::KeyPressed(key) => match key {
                    KeyboardButton::Escape => is_active = false,
                    _ => {}
                },
                InputMessage::CursorPressed {
                    button,
                    ..
                } => match button {
                    CursorButton::Left => is_dragging = true,
                    _ => {}
                },
                InputMessage::CursorReleased {
                    button,
                    ..
                } => match button {
                    CursorButton::Left => is_dragging = false,
                    _ => {}
                },
                InputMessage::CursorMoved {
                    delta,
                    ..
                } => {
                    if is_dragging {
                        screen_settings.translation += delta;
                        engine.batch_update(&screen, &screen_settings);
                    }
                }
                InputMessage::CursorScroll(direction) => {
                    match direction {
                        ScrollDirection::Up => screen_settings.scale *= 1.1,
                        ScrollDirection::Down => screen_settings.scale /= 1.1,
                        _ => {}
                    }
                    engine.batch_update(&screen, &screen_settings);
                }
                _ => {}
            }
        }

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
    const G: f32 = 6.674;
    const MASS: f32 = 200.0;

    pub fn new(pos: Vector3<f32>) -> (Sprite, Particle) {
        let sprite = Sprite::new(pos, Vector2::new(2.0, 2.0), Texture::default(), BLACK, 0.0);
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
        particle.acceleration = -(norm * (Self::G * Self::MASS)) / length_squared.max(500.0);
        particle.velocity += particle.acceleration;
        particle.pos += particle.velocity * delta;
        sprite.pos = particle.pos.extend(0.0);
    }
}
