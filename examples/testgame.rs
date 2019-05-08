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

    let mut clock = Clock::new(144000);
    let mut engine = Engine::new();
    // let texture = engine.texture_load("./examples/resources/2.png");
    let layer_bg = engine.layer_create(0, &LayerDescription::default());
    let mut sprites = Vec::new();
    for x in 0..250 {
        for y in -500..500 {
            let desc = SpriteDescription {
                pos: Vector3::new(x as f32 * 5.0, y as f32 * 5.0, 0f32),
                size: Vector2::new(5.0, 5.0),
                color: BLUE,
                texture: DEFAULT_TEXTURE,
                rotation: 0.125,
            };
            sprites.push((engine.sprite_create(&layer_bg, &desc), desc));
        }
    }
    let layer_fg = engine.layer_create(1, &LayerDescription::default());
    let text = engine.text_create(
        &layer_fg,
        "A Storm Engine",
        &TextDescription::default().pos(Vector3::new(-250.0, 0.0, 0.0)),
    );
    let speed = 200f32;
    let mut translation = Vector2::zero();
    let mut sprite = Sprite::new(&layer_fg);
    sprite.size(Vector2::new(100.0, 100.0));
    sprite.color(color::BLACK);

    let mut is_active = true;
    while is_active {
        engine.input_poll(|message| match message {
            InputMessage::CloseRequested => is_active = false,
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => is_active = false,
                KeyboardButton::W => sprite.velocity(Vector2::new(0f32, speed)),
                KeyboardButton::S => sprite.velocity(Vector2::new(0f32, -speed)),
                KeyboardButton::A => sprite.velocity(Vector2::new(-speed, 0f32)),
                KeyboardButton::D => sprite.velocity(Vector2::new(speed, 0f32)),
                _ => {},
            },
            InputMessage::KeyReleased(key) => match key {
                KeyboardButton::W => sprite.velocity(Vector2::new(0f32, -speed)),
                KeyboardButton::S => sprite.velocity(Vector2::new(0f32, speed)),
                KeyboardButton::A => sprite.velocity(Vector2::new(speed, 0f32)),
                KeyboardButton::D => sprite.velocity(Vector2::new(-speed, 0f32)),
                _ => {},
            },
            _ => {},
        });
        let fps = (1.0 / clock.get_delta()) as u32;
        let string = format!("{}fps", fps);
        engine.text_update(
            &text,
            &string,
            &TextDescription::default().max_width(Some(2000.0)).pos(Vector3::new(-1000.0, 650.0, 0.0)),
        );
        translation.x -= 25.0 * clock.get_delta();
        for (refer, desc) in &mut sprites {
            desc.rotation = desc.rotation + 0.1 * clock.get_delta();
            engine.sprite_update(refer, &desc);
        }
        engine.layer_update(&layer_bg, &LayerDescription::default().translation(translation));
        sprite.update(clock.get_delta());
        sprite.sync(&mut engine);
        engine.window_commit();
        clock.tick();
    }
}
struct Sprite {
    layer: LayerReference,
    key: Option<SpriteReference>,
    desc: SpriteDescription,
    velocity: Vector2<f32>,
}

impl Sprite {
    pub fn new(layer: &LayerReference) -> Sprite {
        Sprite {
            layer: *layer,
            key: None,
            desc: SpriteDescription::default(),
            velocity: Vector2::zero(),
        }
    }

    pub fn sync(&mut self, engine: &mut Engine) {
        match self.key {
            Some(key) => {
                engine.sprite_update(&key, &self.desc);
            },
            None => {
                self.key = Some(engine.sprite_create(&self.layer, &self.desc));
            },
        }
    }

    pub fn size(&mut self, size: Vector2<f32>) {
        self.desc.size = size;
    }

    pub fn texture(&mut self, texture: &TextureReference) {
        self.desc.texture = *texture;
    }

    pub fn color(&mut self, color: Color) {
        self.desc.color = color;
    }

    pub fn velocity(&mut self, velocity: Vector2<f32>) {
        self.velocity += velocity;
    }

    pub fn update(&mut self, delta: f32) {
        self.desc.pos += (self.velocity * delta).extend(0f32);
    }
}
