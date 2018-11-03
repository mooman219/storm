extern crate storm;

use storm::cgmath::*;
use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::color;
use storm::render::message::*;
use storm::time::clock::*;
use storm::utility::indexmap::*;

/// Run with: cargo run --example testgame --release
fn main() {
    storm::log::set_max_level(LevelFilter::Trace);
    storm::run::<TestGame>();
}

pub struct TestGame {
    render: RenderMessenger,
    clock: Clock,
    translation: Vector2<f32>,
    square: MoveableSquare,
}

impl TestGame {
    pub fn generate_world(&mut self) {
        for x in -16..16 {
            let offset = x as f32;
            self.render.create_rect(
                Vector3::new(-1f32 + offset, 0f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::ORANGE,
            );
            self.render.create_rect(
                Vector3::new(-0.5f32 + offset, 0.5f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::RED,
            );
            self.render.create_rect(
                Vector3::new(0f32 + offset, 1f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::PURPLE,
            );
            self.render.create_rect(
                Vector3::new(0.5f32 + offset, 1.5f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::BLUE,
            );
        }
    }
}

impl Game for TestGame {
    const TITLE: &'static str = "Test Game";

    fn new(mut render: RenderMessenger) -> Self {
        let square = MoveableSquare::new(&mut render);
        let mut game = TestGame {
            render: render,
            clock: Clock::new(144),
            translation: Vector2::new(0f32, 0f32),
            square: square,
        };
        game.render.create_texture("./examples/test.png");
        game.generate_world();
        game.render.send();
        game
    }

    fn input(&mut self, event: InputFrame) {
        let speed = 2f32;
        match event {
            InputFrame::KeyPressed(Key::C) => {
                self.render.clear_rects();
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
        self.render.set_translation(self.translation);

        self.render.send();
        self.clock.tick();
    }
}

pub struct MoveableSquare {
    pos: Vector3<f32>,
    size: Vector2<f32>,
    velocity: Vector2<f32>,
    index: IndexToken,
}

impl MoveableSquare {
    pub fn new(render: &mut RenderMessenger) -> MoveableSquare {
        let pos = Vector3::new(-0.5f32, -0.5f32, 0.125f32);
        let size = Vector2::new(1f32, 1f32);
        let index = render.create_rect(pos, size, color::YELLOW);
        MoveableSquare {
            pos: pos,
            size: size,
            velocity: Vector2::zero(),
            index: index,
        }
    }

    pub fn generate_index(&mut self, render: &mut RenderMessenger) {
        self.index = render.create_rect(self.pos, self.size, color::YELLOW);
    }

    pub fn update(&mut self, delta: f32, render: &mut RenderMessenger) {
        self.pos += (self.velocity * delta).extend(0f32);
        render.update_rect(self.index, self.pos, self.size, color::YELLOW);
    }
}
