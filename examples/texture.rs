use storm::time::*;
use storm::*;

/// Run with: cargo run --example texture --release
fn main() {
    simple_logger::SimpleLogger::new().init().expect("Unable to init logger");
    Engine::start(
        WindowSettings {
            title: String::from("Storm: Texture"),
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
    let mut clock = Clock::new(144);
    engine.render.clear_color(storm::colors::BLUE);

    let screen = engine.render.batch_create(&BatchSettings::default());
    let texture_1 =
        engine.render.texture_create(include_bytes!("resources/1.png").as_ref(), TextureFormat::PNG);
    let texture_2 = engine.render.texture_load("./examples/resources/2.png", TextureFormat::PNG).unwrap();
    let texture_2 = texture_2.sub_texture(0, 0, 16, 16).unwrap();

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

    engine.render.sprite_set(&screen, &sprites);

    move |event, engine| match event {
        InputMessage::CloseRequested => engine.stop(),
        InputMessage::KeyPressed(key) => match key {
            KeyboardButton::Escape => engine.stop(),
            _ => {}
        },
        InputMessage::CursorPressed {
            ..
        } => {
            let sprite = &mut sprites[4];
            sprite.texture = sprite.texture.mirror_x();
            engine.render.sprite_set(&screen, &sprites);
        }
        InputMessage::CursorMoved {
            pos,
            ..
        } => {
            let sprite = &mut sprites[4];
            sprite.pos = pos.extend(0.1);
            engine.render.sprite_set(&screen, &sprites);
        }
        InputMessage::MainEventsCleared => {
            engine.render.draw();
            clock.tick();
        }
        _ => {}
    }
}
