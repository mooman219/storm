use cgmath::{Vector2, Vector3};
use core::time::Duration;
use storm::*;

static TEXTURE_A: &[u8] = include_bytes!("resources/4.png");

/// Run with: cargo run --example texture --release
fn main() {
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

    let back_sprite = Sprite::default();
    let slider = Sprite {
        pos: Vector3::new(-200.0, -62.0, 0.0),
        size: Vector2::new(25, 25),
        color: RGBA8::WHITE,
        ..Sprite::default()
    };
    let line = Sprite {
        pos: Vector3::new(-200.0, -50.0, 0.0),
        size: Vector2::new(400, 3),
        color: RGBA8::BLACK,
        ..Sprite::default()
    };

    let mut back = ctx.sprite_layer();
    back.set_atlas(&ctx.load_png(TEXTURE_A));
    let mut back_sprites = Vec::new();
    back_sprites.push(back_sprite);
    back_sprites.push(slider);
    back_sprites.push(line);

    back.set_sprites(&back_sprites);

    let mut clicking = false;

    move |event, ctx| match event {
        Event::CloseRequested => ctx.stop(),
        Event::KeyPressed(key) => match key {
            KeyboardButton::Escape => ctx.stop(),
            _ => {}
        },
        Event::CursorPressed {
            pos,
            ..
        } => {
            if pos.x >= back_sprites[1].pos.x
                && pos.x <= back_sprites[1].pos.x + back_sprites[1].size.x as f32
                && pos.y >= back_sprites[1].pos.y
                && pos.y <= back_sprites[1].pos.y + back_sprites[1].size.y as f32
            {
                clicking = true;
            }
        }
        Event::CursorReleased {
            ..
        } => {
            clicking = false;
        }
        Event::CursorMoved {
            pos,
            ..
        } => {
            let x = pos.x - 12.0;
            if clicking && x >= -200.0 && x <= 175.0 {
                back_sprites[1].pos.x = x;
                back.set_sprites(&back_sprites);
            }
        }
        Event::Update(_delta) => {
            ctx.clear(ClearMode::color_depth(RGBA8::BLUE));
            back.draw();
        }
        _ => {}
    }
}
