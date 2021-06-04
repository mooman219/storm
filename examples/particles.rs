use crate::cgmath::prelude::*;
use crate::cgmath::*;
use core::time::Duration;
use storm::colors::*;
use storm::*;

/// Run with: cargo run --example particles --release
fn main() {
    simple_logger::SimpleLogger::new().init().expect("Unable to init logger");
    Engine::start(
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

fn run(engine: &mut Engine) -> impl FnMut(InputMessage, &mut Engine) {
    engine.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));
    engine.clear_color(BLACK);
    let mut is_dragging = false;

    let mut screen = engine.batch_create();
    let mut screen_transform = BatchTransform::new();
    screen_transform.rotation = 0.125;
    screen.set_transform(&screen_transform);

    let mut sprites = Vec::new();
    let mut particles = Vec::new();
    const RANGE: i32 = 500;
    for x in -RANGE..RANGE {
        for y in -RANGE..RANGE {
            let (sprite, particle) = Particle::new(Vector3::new(x as f32 * 5.0, y as f32 * 5.0, 0.0));
            sprites.push(sprite);
            particles.push(particle);
        }
    }
    screen.set_sprites(&sprites);

    move |event, engine| match event {
        InputMessage::CloseRequested => engine.stop(),
        InputMessage::KeyPressed(key) => match key {
            KeyboardButton::Escape => engine.stop(),
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
                screen_transform.translation += delta / screen_transform.scale;
                screen.set_transform(&screen_transform);
            }
        }
        InputMessage::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => screen_transform.scale *= 1.1,
                ScrollDirection::Down => screen_transform.scale /= 1.1,
                _ => {}
            }
            screen.set_transform(&screen_transform);
        }
        InputMessage::Update(delta) => {
            for index in 0..sprites.len() {
                Particle::tick(&mut sprites[index], &mut particles[index], delta);
            }
            screen.set_sprites(&sprites);
            engine.draw();
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
        let sprite = Sprite::new(pos, Vector2::new(2.0, 2.0), Texture::default(), WHITE, 0.0);
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
