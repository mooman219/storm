use crate::cgmath::*;
use storm::time::*;
use storm::*;

/// Run with: cargo run --example texture --release
fn main() {
    let mut clock = Clock::new(144);
    let mut engine = Engine::new(WindowDescription {
        title: String::from("Storm: Texture"),
        size: Vector2::new(1280.0, 1024.0),
        resizable: true,
    });

    let texture_1 = engine.texture_load("./examples/resources/1.png");
    let texture_2 = engine.texture_load("./examples/resources/2.png");

    let screen = engine.batch_create(&BatchDescription::default());
    let mut sprites = Vec::new();
    let mut sprite = SpriteDescription::default();
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
        engine.input_poll(|message| match message {
            InputMessage::CloseRequested => is_active = false,
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => is_active = false,
                _ => {},
            },
            InputMessage::CursorMoved(pos) => {
                let sprite = &mut sprites[4];
                sprite.pos = pos.extend(0.1);
                moved = true;
            },
            _ => {},
        });
        if moved {
            moved = false;
            engine.sprite_set(&screen, &sprites);
        }
        engine.window_commit();
        clock.tick();
    }
}
