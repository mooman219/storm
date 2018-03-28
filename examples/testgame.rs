extern crate storm;

use storm::cgmath::*;
use storm::game::*;
use storm::input::message::*;
use storm::log::LevelFilter;
use storm::render::color;
use storm::render::message::*;
use storm::time::clock::*;
use storm::utility::slotmap::*;

/// Run with: cargo run --example testgame --release
fn main() {
    storm::log::set_max_level(LevelFilter::Trace);
    storm::run::<TestGame>();
}

pub struct TestGame {
    render: RenderProducer,
    clock: Clock,
    translation: Vector2<f32>,
    triangle: MovableTriangle,
}

pub struct MovableTriangle {
    pos: Vector2<f32>,
    velocity: Vector2<f32>,
    index: IndexToken,
}

impl MovableTriangle {
    pub fn new(render: &mut RenderProducer) -> MovableTriangle {
        let index = render.create_triangle(Vector2::new(0.0, 0.0), 1f32, color::YELLOW);
        MovableTriangle {
            pos: Vector2::new(0f32, 0f32),
            velocity: Vector2::new(0f32, 0f32),
            index: index,
        }
    }

    pub fn update(&mut self, delta: f32, render: &mut RenderProducer) {
        self.pos += self.velocity * delta;
        render.update_triangle(&self.index, self.pos, 1f32, color::YELLOW);
    }
}

impl Game for TestGame {
    const TITLE: &'static str = "Test Game";

    fn new(mut render: RenderProducer) -> Self {
        let triangle = MovableTriangle::new(&mut render);
        for x in -16..4 {
            let offset = x as f32;
            render.create_rect(
                Vector2::new(-1f32 + offset, 0f32),
                Vector2::new(0.5f32, 0.5f32),
                color::ORANGE,
            );
            render.create_rect(
                Vector2::new(-0.5f32 + offset, 0.5f32),
                Vector2::new(0.5f32, 0.5f32),
                color::RED,
            );
            render.create_rect(
                Vector2::new(0f32 + offset, 1f32),
                Vector2::new(0.5f32, 0.5f32),
                color::PURPLE,
            );
            render.create_rect(
                Vector2::new(0.5f32 + offset, 1.5f32),
                Vector2::new(0.5f32, 0.5f32),
                color::BLUE,
            );
        }
        render.set_scale(0.5f32);
        render.send();
        TestGame {
            render: render,
            clock: Clock::new(200),
            translation: Vector2::new(0f32, 0f32),
            triangle: triangle,
        }
    }

    fn input(&mut self, event: InputFrame) {
        match event {
            InputFrame::KeyPressed(Key::W) => self.triangle.velocity.y += 1.5f32,
            InputFrame::KeyReleased(Key::W) => self.triangle.velocity.y -= 1.5f32,
            InputFrame::KeyPressed(Key::A) => self.triangle.velocity.x -= 1.5f32,
            InputFrame::KeyReleased(Key::A) => self.triangle.velocity.x += 1.5f32,
            InputFrame::KeyPressed(Key::S) => self.triangle.velocity.y -= 1.5f32,
            InputFrame::KeyReleased(Key::S) => self.triangle.velocity.y += 1.5f32,
            InputFrame::KeyPressed(Key::D) => self.triangle.velocity.x += 1.5f32,
            InputFrame::KeyReleased(Key::D) => self.triangle.velocity.x -= 1.5f32,
            _ => {},
        }
    }

    fn tick(&mut self) {
        let delta = self.clock.get_delta();
        if self.translation.x > 6f32 {
            self.translation.x = 0f32;
        }
        self.translation.x += 0.1f32 * delta;
        self.triangle.update(delta, &mut self.render);
        self.render.set_translation(self.translation);
        self.render.send();
        self.clock.tick();
    }
}
