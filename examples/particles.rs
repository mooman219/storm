use crate::cgmath::prelude::*;
use crate::cgmath::*;
use storm::colors::*;
use storm::time::*;
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
            vsync: Vsync::Enabled,
        },
        run,
    );
}

fn run(engine: &mut Engine) -> impl FnMut(InputMessage, &mut Engine) {
    let mut clock = Clock::new(144);
    let mut is_dragging = false;
    engine.render.clear_color(WHITE);
    let mut screen_settings = BatchSettings {
        rotation: 0.125,
        ..BatchSettings::default()
    };
    let screen = engine.render.batch_create(&screen_settings);
    let mut sprites = Vec::new();
    let mut particles = Vec::new();
    let texture_1 =
        engine.render.texture_create(include_bytes!("resources/1.png").as_ref(), TextureFormat::PNG);
    const RANGE: i32 = 250;
    for x in -RANGE..RANGE {
        for y in -RANGE..RANGE {
            let (mut sprite, particle) = Particle::new(Vector3::new(x as f32 * 5.0, y as f32 * 5.0, 0.0));
            sprite.texture = texture_1;
            sprites.push(sprite);
            particles.push(particle);
        }
    }
    engine.render.sprite_set(&screen, &sprites);

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
                screen_settings.translation += delta / screen_settings.scale;
                engine.render.batch_update(&screen, &screen_settings);
            }
        }
        InputMessage::CursorScroll(direction) => {
            match direction {
                ScrollDirection::Up => screen_settings.scale *= 1.1,
                ScrollDirection::Down => screen_settings.scale /= 1.1,
                _ => {}
            }
            engine.render.batch_update(&screen, &screen_settings);
        }
        InputMessage::MainEventsCleared => {
            let delta = clock.get_delta();
            for index in 0..sprites.len() {
                Particle::tick(&mut sprites[index], &mut particles[index], delta);
            }
            engine.render.sprite_set(&screen, &sprites);
            engine.render.draw();
            clock.tick();
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
