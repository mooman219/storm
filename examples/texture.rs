use storm::time::*;
use storm::*;

/// Run with: cargo run --example texture --release
fn main() {
    simple_logger::init().unwrap();
    Engine::start::<TextureTest>(WindowSettings {
        title: String::from("Storm: Texture"),
        display_mode: DisplayMode::Windowed {
            width: 1280,
            height: 1024,
            resizable: true,
        },
        vsync: Vsync::Disabled,
    });
}

struct TextureTest {
    clock: Clock,
    screen: BatchToken,
    sprites: Vec<Sprite>,
}

impl Program for TextureTest {
    fn create(engine: &mut Engine) -> Result<Self, &'static str> {
        engine.window_clear_color(storm::colors::BLUE);

        let screen = engine.batch_create(&BatchSettings::default());
        let texture_1 = engine.texture_create(include_bytes!("resources/1.png").as_ref(), TextureFormat::PNG);
        let texture_2 = engine.texture_load("./examples/resources/2.png", TextureFormat::PNG)?;
        let texture_2 = texture_2.sub_texture(0, 0, 16, 16)?;

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

        Ok(TextureTest {
            clock: Clock::new(144),
            screen,
            sprites,
        })
    }

    fn input(&mut self, event: InputMessage, engine: &mut Engine) {
        match event {
            InputMessage::CloseRequested => engine.stop(),
            InputMessage::KeyPressed(key) => match key {
                KeyboardButton::Escape => engine.stop(),
                _ => {}
            },
            InputMessage::CursorPressed {
                ..
            } => {
                let sprite = &mut self.sprites[4];
                sprite.texture = sprite.texture.mirror_x();
                engine.sprite_set(&self.screen, &self.sprites);
            }
            InputMessage::CursorMoved {
                pos,
                ..
            } => {
                let sprite = &mut self.sprites[4];
                sprite.pos = pos.extend(0.1);
                engine.sprite_set(&self.screen, &self.sprites);
            }
            _ => {}
        }
    }

    fn update(&mut self, engine: &mut Engine) {
        engine.draw();
        self.clock.tick();
    }
}
