use core::time::Duration;
use storm::*;

static TEXTURE_A: &[u8] = include_bytes!("resources/1.png");
static TEXTURE_B: &[u8] = include_bytes!("resources/1.png");

/// Run with: cargo run --example texture --release
fn main() {
    simple_logger::SimpleLogger::new().init().expect("Unable to init logger");
    Context::start(
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

fn run(ctx: &mut Context) -> impl FnMut(InputMessage, &mut Context) {
    ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));

    let mut screen = ctx.layer_sprite();
    screen.clear_mode(Some(ClearMode::color_depth(colors::BLUE)));
    let texture_1 = ctx.texture_create(TEXTURE_A.as_ref(), TextureFormat::PNG);
    let texture_2 = ctx.texture_create(TEXTURE_B.as_ref(), TextureFormat::PNG);
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

    screen.set_sprites(&sprites);

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
            screen.set_sprites(&sprites);
        }
        InputMessage::CursorMoved {
            pos,
            ..
        } => {
            let sprite = &mut sprites[4];
            sprite.pos = pos.extend(0.1);
            screen.set_sprites(&sprites);
        }
        InputMessage::Update(_delta) => {
            screen.execute();
        }
        _ => {}
    }
}
