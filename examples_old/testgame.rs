extern crate storm;

use storm::cgmath::*;
use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::color;
use storm::render::color::Color;
use storm::render::message::*;
use storm::time::clock::*;

/// Run with: cargo run --example testgame --release
fn main() {
    storm::log::set_max_level(LevelFilter::Trace);
    storm::run::<TestGame>();
}

pub struct Textures {
    main: TextureReference,
    plank: TextureReference,
}

pub struct Descriptions {
    main: QuadDescription,
    plank_white: QuadDescription,
    plank_orange: QuadDescription,
    plank_purple: QuadDescription,
    plank_red: QuadDescription,
    plain_blue: QuadDescription,
}

pub struct TestGame {
    render: RenderMessenger,
    textures: Textures,
    quads: Descriptions,
    clock: Clock,
    translation: Vector2<f32>,
    square: MoveableSquare,
}

impl TestGame {
    pub fn generate_world(&mut self) {
        for x in -16..16 {
            let offset = x as f32;
            self.render
                .quad_create(Vector3::new(-1f32 + offset, 0f32, 0f32), self.quads.plank_orange);
            self.render
                .quad_create(Vector3::new(-0.5f32 + offset, 0.5f32, 0f32), self.quads.plank_red);
            self.render
                .quad_create(Vector3::new(0f32 + offset, 1f32, 0f32), self.quads.plank_purple);
            self.render
                .quad_create(Vector3::new(0.5f32 + offset, 1.5f32, 0f32), self.quads.plain_blue);
        }
    }
}

impl Game for TestGame {
    fn new(mut render: RenderMessenger) -> Self {
        let textures = Textures {
            main: render.texture_create("./examples/testgame/1.png"),
            plank: render.texture_create("./examples/testgame/2.png"),
        };
        let quads = Descriptions {
            main: QuadDescription {
                size: Vector2::new(1f32, 1f32),
                color: color::WHITE,
                texture: textures.main,
            },
            plank_white: QuadDescription {
                size: Vector2::new(0.5f32, 0.5f32),
                color: color::WHITE,
                texture: textures.plank,
            },
            plank_orange: QuadDescription {
                size: Vector2::new(0.5f32, 0.5f32),
                color: color::ORANGE,
                texture: textures.plank,
            },
            plank_purple: QuadDescription {
                size: Vector2::new(0.5f32, 0.5f32),
                color: color::PURPLE,
                texture: textures.plank,
            },
            plank_red: QuadDescription {
                size: Vector2::new(0.5f32, 0.5f32),
                color: color::RED,
                texture: textures.plank,
            },
            plain_blue: QuadDescription {
                size: Vector2::new(0.5f32, 0.5f32),
                color: Color::new(0f32, 0f32, 1f32, 0.75f32),
                texture: textures.plank,
            },
        };
        render.quad_create(Vector3::new(0.5f32, 1.5f32, 0f32), quads.plank_white);
        let square = MoveableSquare::new(&mut render, quads.main);
        let mut game = TestGame {
            render: render,
            textures: textures,
            quads: quads,
            clock: Clock::new(144),
            translation: Vector2::new(0f32, 0f32),
            square: square,
        };
        game.render.window_title("Game of Testing");
        // game.generate_world();
        game.render.send();
        game
    }

    fn input(&mut self, event: InputFrame) {
        let speed = 2f32;
        match event {
            InputFrame::KeyPressed(Key::C) => {
                self.render.quad_clear();
                self.square.generate_index(&mut self.render);
            },
            InputFrame::KeyPressed(Key::V) => {
                self.generate_world();
            },
            InputFrame::KeyPressed(Key::W) => self.square.velocity.y += speed,
            InputFrame::KeyReleased(Key::W) => self.square.velocity.y -= speed,
            InputFrame::KeyPressed(Key::A) => self.square.velocity.x -= speed,
            InputFrame::KeyReleased(Key::A) => self.square.velocity.x += speed,
            InputFrame::KeyPressed(Key::S) => self.square.velocity.y -= speed,
            InputFrame::KeyReleased(Key::S) => self.square.velocity.y += speed,
            InputFrame::KeyPressed(Key::D) => self.square.velocity.x += speed,
            InputFrame::KeyReleased(Key::D) => self.square.velocity.x -= speed,
            _ => {},
        }
    }

    fn tick(&mut self) {
        let delta = self.clock.get_delta();
        self.square.update(delta, &mut self.render);

        // Center the square
        self.translation.x = -self.square.pos.x - 0.5f32;
        self.translation.y = -self.square.pos.y - 0.5f32;
        self.render.translate(self.translation);

        self.render.send();
        self.clock.tick();
    }
}

pub struct MoveableSquare {
    index: QuadReference,
    pos: Vector3<f32>,
    velocity: Vector2<f32>,
    desc: QuadDescription,
}

impl MoveableSquare {
    pub fn new(render: &mut RenderMessenger, desc: QuadDescription) -> MoveableSquare {
        let pos = Vector3::new(-0.5f32, -0.5f32, -0.125f32);
        let index = render.quad_create(pos, desc);
        MoveableSquare {
            index: index,
            pos: pos,
            velocity: Vector2::zero(),
            desc: desc,
        }
    }

    pub fn generate_index(&mut self, render: &mut RenderMessenger) {
        self.index = render.quad_create(self.pos, self.desc);
    }

    pub fn update(&mut self, delta: f32, render: &mut RenderMessenger) {
        self.pos += (self.velocity * delta).extend(0f32);
        render.quad_update(self.index, self.pos, self.desc);
    }
}
