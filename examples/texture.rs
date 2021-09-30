use core::time::Duration;
use storm::*;

static TEXTURE_A: &[u8] = include_bytes!("resources/1.png");
static TEXTURE_B: &[u8] = include_bytes!("resources/2.png");

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

fn run(ctx: &mut Context) -> impl FnMut(Event, &mut Context) {
    ctx.wait_periodic(Some(Duration::from_secs_f32(1.0 / 144.0)));

    let mut sprite = Sprite::default();

    let mut back = ctx.sprite_layer();
    let back_texture = ctx.texture(&Image::from_bytes(TEXTURE_A, ImageFormat::PNG));
    back.set_atlas(&back_texture);
    let back_texture_section = TextureSection::full();
    let mut back_sprites = Vec::new();
    sprite.texture = back_texture_section;
    back_sprites.push(sprite);
    sprite.texture = back_texture_section.mirror_x();
    sprite.pos.y -= 100.0;
    back_sprites.push(sprite);
    sprite.texture = back_texture_section.mirror_y();
    sprite.pos.y += 100.0;
    sprite.pos.x += 100.0;
    back_sprites.push(sprite);
    sprite.texture = back_texture_section.mirror_x().mirror_y();
    sprite.pos.y -= 100.0;
    back_sprites.push(sprite);
    sprite.texture = back_texture_section;
    sprite.pos.z = 0.1;
    back_sprites.push(sprite);
    back.set_sprites(&back_sprites);

    let mut front = ctx.sprite_layer();
    let front_texture = ctx.texture(&Image::from_bytes(TEXTURE_B, ImageFormat::PNG));
    front.set_atlas(&front_texture);
    let front_texture_section = front_texture.subsection(16, 32, 0, 16);
    let mut front_sprites = Vec::new();
    sprite.texture = front_texture_section;
    sprite.pos.y -= 100.0;
    front_sprites.push(sprite);
    sprite.texture = TextureSection::full();
    sprite.pos.y -= 100.0;
    front_sprites.push(sprite);
    front.set_sprites(&front_sprites);

    move |event, ctx| match event {
        Event::CloseRequested => ctx.stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Escape => ctx.stop(),
            _ => {}
        },
        Event::CursorPressed {
            ..
        } => {
            let sprite = &mut back_sprites[4];
            sprite.texture = sprite.texture.mirror_x();
            back.set_sprites(&back_sprites);
        }
        Event::CursorMoved {
            pos,
            ..
        } => {
            let sprite = &mut back_sprites[4];
            sprite.pos = pos.extend(0.1);
            back.set_sprites(&back_sprites);
        }
        Event::Update(_delta) => {
            ctx.clear(ClearMode::color_depth(colors::BLUE));
            back.draw();
            front.draw();
        }
        _ => {}
    }
}
