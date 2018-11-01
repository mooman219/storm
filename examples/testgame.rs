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

pub struct MoveableSquare {
    pos: Vector2<f32>,
    velocity: Vector2<f32>,
    index: IndexToken,
}

impl MoveableSquare {
    pub fn new(render: &mut RenderMessenger) -> MoveableSquare {
        let index = render.create_rect(
            Vector3::new(0.0f32, 0.0f32, -0.125f32),
            Vector2::new(0.75f32, 0.75f32),
            color::YELLOW,
        );
        MoveableSquare {
            pos: Vector2::new(0f32, 0f32),
            velocity: Vector2::new(0f32, 0f32),
            index: index,
        }
    }

    pub fn update(&mut self, delta: f32, render: &mut RenderMessenger) {
        self.pos += self.velocity * delta;
        render.update_rect(
            self.index,
            self.pos.extend(-0.125f32),
            Vector2::new(0.75f32, 0.75f32),
            color::YELLOW,
        );
    }
}

impl Game for TestGame {
    const TITLE: &'static str = "Test Game";

    fn new(mut render: RenderMessenger) -> Self {
        let square = MoveableSquare::new(&mut render);
        for x in -16..4 {
            let offset = x as f32;
            render.create_rect(
                Vector3::new(-1f32 + offset, 0f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::ORANGE,
            );
            render.create_rect(
                Vector3::new(-0.5f32 + offset, 0.5f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::RED,
            );
            render.create_rect(
                Vector3::new(0f32 + offset, 1f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::PURPLE,
            );
            render.create_rect(
                Vector3::new(0.5f32 + offset, 1.5f32, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::BLUE,
            );
        }
        render.create_texture("./examples/test.png");
        render.set_scale(0.5f32);
        render.send();
        TestGame {
            render: render,
            clock: Clock::new(144),
            translation: Vector2::new(0f32, 0f32),
            square: square,
        }
    }

    fn input(&mut self, event: InputFrame) {
        match event {
            InputFrame::KeyPressed(Key::C) => {
                self.render.clear_rects();
                self.square.index = self.render.create_rect(
                    Vector3::new(0.0f32, 0.0f32, -0.125f32),
                    Vector2::new(0.75f32, 0.75f32),
                    color::YELLOW,
                );
            },
            InputFrame::KeyPressed(Key::W) => self.square.velocity.y += 1.5f32,
            InputFrame::KeyReleased(Key::W) => self.square.velocity.y -= 1.5f32,
            InputFrame::KeyPressed(Key::A) => self.square.velocity.x -= 1.5f32,
            InputFrame::KeyReleased(Key::A) => self.square.velocity.x += 1.5f32,
            InputFrame::KeyPressed(Key::S) => self.square.velocity.y -= 1.5f32,
            InputFrame::KeyReleased(Key::S) => self.square.velocity.y += 1.5f32,
            InputFrame::KeyPressed(Key::D) => self.square.velocity.x += 1.5f32,
            InputFrame::KeyReleased(Key::D) => self.square.velocity.x -= 1.5f32,
            _ => {},
        }
    }

    fn tick(&mut self) {
        let delta = self.clock.get_delta();
        if self.translation.x > 6f32 {
            self.translation.x = 0f32;
        }
        self.translation.x += 0.05f32 * delta;
        self.square.update(delta, &mut self.render);
        self.render.set_translation(self.translation);
        self.render.send();
        self.clock.tick();
    }
}
