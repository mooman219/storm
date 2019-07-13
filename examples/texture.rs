use crate::cgmath::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example texture --release
fn main() {
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Texture"),
            size: Vector2::new(1280, 1024),
            resizable: true,
        },
        game,
    );
}

fn game(mut engine: Engine) {
    let mut clock = Clock::new(144);

    let texture_1_bytes = include_bytes!("resources/1.png");
    let texture_1 = engine.texture_load_bytes(texture_1_bytes, TextureFormat::PNG);
    let texture_2 = engine.texture_load("./examples/resources/2.png");
    let texture_2 = texture_2.sub_texture(0, 0, 16, 16).unwrap();

    engine.window_clear_color(storm::color::BLUE);
    let screen = engine.batch_create(&BatchSettings::default());
    let mut sprites = Vec::new();
    let mut sprite = Sprite::default();
    sprite.texture = texture_1;
    sprites.push(sprite);
    sprite.texture = texture_1.mirror_x();
    sprite.pos.y -= 100.0;
    sprites.push(sprite);
    sprite.texture = texture_1.mirror_y();
    sprite.pos.y += 100.0;
    sprite.pos.x += 100.0;
    sprites.push(sprite);
    sprite.texture = texture_1.mirror_x().mirror_y();
    sprite.pos.y -= 100.0;
    sprites.push(sprite);

    sprite.texture = texture_2;
    sprite.pos.z = 0.1;
    sprites.push(sprite);
    engine.sprite_set(&screen, &sprites);

    let mut is_active = true;
    let mut moved = false;
    while is_active {
        while let Some(message) = engine.input_poll() {
            match message {
                InputMessage::CloseRequested => is_active = false,
                InputMessage::KeyPressed(key) => match key {
                    KeyboardButton::Escape => is_active = false,
                    _ => {},
                },
                InputMessage::CursorPressed(..) => {
                    let sprite = &mut sprites[4];
                    sprite.texture = sprite.texture.mirror_x();
                    moved = true;
                },
                InputMessage::CursorMoved(pos) => {
                    let sprite = &mut sprites[4];
                    sprite.pos = pos.extend(0.1);
                    moved = true;
                },
                _ => {},
            }
        }
        if moved {
            moved = false;
            engine.sprite_set(&screen, &sprites);
        }
        engine.window_commit();
        clock.tick();
    }
}
