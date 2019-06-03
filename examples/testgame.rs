#![allow(dead_code)]

extern crate log;
extern crate storm;

mod logger;

use cgmath::*;
use log::LevelFilter;
use logger::*;
use storm::color::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example testgame --release
fn main() {
    SimpleLogger::init(LevelFilter::Trace);

    let mut clock = Clock::new(14400);
    let mut engine = Engine::new();

    let background = engine.batch_create(&BatchDescription::default());
    let mut sprites = Vec::new();
    for x in -500..500 {
        for y in -500..500 {
            sprites.push(SpriteDescription::new(
                Vector3::new(x as f32 * 1.0, y as f32 * 1.0, 0f32),
                Vector2::new(1.0, 1.0),
                DEFAULT_TEXTURE,
                BLUE,
                0.125,
            ));
        }
    }

    // let speed = 200f32;
    // let mut velocity = Vector2::zero()
    // let mut translation = Vector2::zero();
    let mut is_active = true;

    while is_active {
        engine.input_poll(|message| match message {
            InputMessage::CloseRequested => is_active = false,
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => is_active = false,
                // KeyboardButton::W => sprite.velocity(Vector2::new(0f32, speed)),
                // KeyboardButton::S => sprite.velocity(Vector2::new(0f32, -speed)),
                // KeyboardButton::A => sprite.velocity(Vector2::new(-speed, 0f32)),
                // KeyboardButton::D => sprite.velocity(Vector2::new(speed, 0f32)),
                _ => {},
            },
            InputMessage::KeyReleased(key) => match key {
                // KeyboardButton::W => sprite.velocity(Vector2::new(0f32, -speed)),
                // KeyboardButton::S => sprite.velocity(Vector2::new(0f32, speed)),
                // KeyboardButton::A => sprite.velocity(Vector2::new(speed, 0f32)),
                // KeyboardButton::D => sprite.velocity(Vector2::new(-speed, 0f32)),
                _ => {},
            },
            _ => {},
        });
        // let fps = (1.0 / clock.get_delta()) as u32;
        // let string = format!("{}fps", fps);
        // engine.text_update(
        //     &text,
        //     &string,
        //     &StringDescription::default().max_width(Some(2000.0)).pos(Vector3::new(-1000.0, 650.0, 0.5)),
        // );
        // translation.x -= 25.0 * clock.get_delta();
        for desc in &mut sprites {
            desc.rotation(0.25 * clock.get_delta());
        }
        engine.sprite_set(&background, &sprites);
        engine.window_commit();
        clock.tick();
    }
}
