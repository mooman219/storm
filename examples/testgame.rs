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

    let mut clock = Clock::new(20000);
    let mut engine = Engine::new();
    let layer_bg = engine.layer_create(0, &LayerDescription::default());
    for x in 0..1000 {
        for y in -500..500 {
            let color = if x & 1 != 0 {
                if y & 1 != 0 {
                    ORANGE
                } else {
                    BLUE
                }
            } else {
                if y & 1 != 0 {
                    GREEN
                } else {
                    RED
                }
            };
            engine.sprite_create(
                &layer_bg,
                &SpriteDescription {
                    pos: Vector3::new(x as f32 * 1.0, y as f32 * 1.0, 0f32),
                    size: Vector2::new(1.0, 1.0),
                    color: color,
                    texture: DEFAULT_TEXTURE,
                },
            );
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
        // let string = format!("{}fps Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Fusce id velit ut tortor pretium viverra suspendisse. Eu sem integer vitae justo. Nec feugiat nisl pretium fusce id velit. Porttitor massa id neque aliquam vestibulum morbi blandit cursus. Fermentum et sollicitudin ac orci phasellus egestas tellus rutrum tellus. Sed id semper risus in hendrerit gravida. Et molestie ac feugiat sed lectus vestibulum mattis. Sed risus ultricies tristique nulla aliquet enim tortor at. Donec ultrices tincidunt arcu non sodales neque. Massa tincidunt nunc pulvinar sapien et. Ultrices mi tempus imperdiet nulla malesuada pellentesque. Nec ullamcorper sit amet risus nullam eget felis eget. Ac turpis egestas integer eget aliquet. Aenean pharetra magna ac placerat. Sollicitudin tempor id eu nisl nunc mi ipsum faucibus. Mi bibendum neque egestas congue quisque egestas diam in. Mattis pellentesque id nibh tortor id aliquet. Mauris a diam maecenas sed. Sed odio morbi quis commodo odio aenean sed. Vulputate dignissim suspendisse in est ante in nibh mauris. Condimentum mattis pellentesque id nibh tortor id aliquet. Non odio euismod lacinia at quis risus. Vitae tortor condimentum lacinia quis vel eros donec ac. Tellus integer feugiat scelerisque varius morbi enim nunc faucibus a. Amet nisl suscipit adipiscing bibendum est ultricies integer. Fermentum leo vel orci porta non pulvinar neque laoreet. Diam vulputate ut pharetra sit amet. Viverra mauris in aliquam sem fringilla ut morbi tincidunt. Molestie nunc non blandit massa enim nec dui. Purus gravida quis blandit turpis cursus in hac habitasse. Arcu risus quis varius quam quisque id. Vulputate dignissim suspendisse in est. Diam sollicitudin tempor id eu nisl nunc. Eu sem integer vitae justo eget magna fermentum. In fermentum et sollicitudin ac. Aenean sed adipiscing diam donec adipiscing. Non enim praesent elementum facilisis leo. Aenean et tortor at risus viverra adipiscing at. Ut sem viverra aliquet eget sit. Non diam phasellus vestibulum lorem sed risus ultricies tristique. Nullam vehicula ipsum a arcu cursus vitae congue mauris. Nunc aliquet bibendum enim facilisis gravida neque. A diam sollicitudin tempor id eu nisl nunc mi. Ornare aenean euismod elementum nisi quis eleifend quam adipiscing vitae. Nunc scelerisque viverra mauris in aliquam sem fringilla ut morbi. Interdum varius sit amet mattis vulputate enim nulla aliquet. Placerat vestibulum lectus mauris ultrices eros in cursus turpis massa. Quis varius quam quisque id diam vel quam elementum pulvinar. Ultrices tincidunt arcu non sodales neque sodales ut etiam sit. Morbi tincidunt ornare massa eget egestas purus. At tellus at urna condimentum mattis. Sed egestas egestas fringilla phasellus. A arcu cursus vitae congue mauris rhoncus aenean. Purus in massa tempor nec feugiat nisl pretium fusce. In cursus turpis massa tincidunt dui. Quis imperdiet massa tincidunt nunc pulvinar sapien et ligula ullamcorper. Bibendum est ultricies integer quis auctor elit sed vulputate mi. Vulputate ut pharetra sit amet aliquam id diam maecenas. Habitant morbi tristique senectus et netus et. Ante in nibh mauris cursus mattis molestie a. Odio tempor orci dapibus ultrices. Purus gravida quis blandit turpis cursus in hac. Felis eget nunc lobortis mattis aliquam faucibus. Vestibulum lectus mauris ultrices eros in. Eget nunc scelerisque viverra mauris in aliquam sem fringilla. Gravida cum sociis natoque penatibus et magnis. Tortor pretium viverra suspendisse potenti. In nisl nisi scelerisque eu ultrices. Egestas purus viverra accumsan in nisl. Cras ornare arcu dui vivamus arcu felis bibendum ut tristique. Mus mauris vitae ultricies leo. Ante in nibh mauris cursus mattis molestie a iaculis at. Odio pellentesque diam volutpat commodo sed egestas egestas fringilla phasellus. Faucibus purus in massa tempor nec. Sollicitudin nibh sit amet commodo nulla. Sed ullamcorper morbi tincidunt ornare. Integer quis auctor elit sed vulputate mi sit amet. Id aliquet risus feugiat in ante metus dictum at tempor. At imperdiet dui accumsan sit.", fps);
        let string = format!("{}fps", fps);
        engine.text_update(
            &text,
            &string,
            &TextDescription::default().max_width(Some(1000.0)).pos(Vector3::new(0.0, 485.0, 0.0)),
        );
        translation.x -= 25.0 * clock.get_delta();
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
