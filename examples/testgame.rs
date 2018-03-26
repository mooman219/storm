extern crate storm;

use storm::cgmath::*;
use storm::engine;
use storm::engine::*;
use storm::render::color;
use storm::render::message::producer::*;
use storm::time::clock::*;
use storm::utility::slotmap::*;

/// Run with: cargo run --example testgame --release
/// Ideally the game code would exist in examples/, but it is difficult to develop on because
/// files in the examples folder are not referenced by the RLS. Game code will exist in this sub
/// directory for the time being while this project is being prototyped.
///
/// See https://github.com/rust-lang-nursery/rls/issues/269
fn main() {
    engine::run::<TestGame>();
}

pub struct TestGame {
    state: GameState,
    render: RenderProducer,
    clock: Clock,
    translation: Vector3<f32>,
    triangle: IndexToken,
}

#[derive(Copy, Clone)]
pub enum GameState {
    Startup,
    Running,
}

impl Game for TestGame {
    const TITLE: &'static str = "Test Game";

    fn new(render: RenderProducer) -> Self {
        TestGame {
            state: GameState::Startup,
            render: render,
            clock: Clock::new(200),
            translation: Vector3::new(0f32, 0f32, 0f32),
            triangle: IndexToken::invalid(),
        }
    }

    fn tick(&mut self) {
        match self.state {
            GameState::Startup => {
                for x in -16..4 {
                    let offset = x as f32;
                    self.render.create_rect(
                        Vector2::new(-1f32 + offset, 0f32),
                        Vector2::new(0.5f32, 0.5f32),
                        color::ORANGE,
                    );
                    self.render.create_rect(
                        Vector2::new(-0.5f32 + offset, 0.5f32),
                        Vector2::new(0.5f32, 0.5f32),
                        color::RED,
                    );
                    self.render.create_rect(
                        Vector2::new(0f32 + offset, 1f32),
                        Vector2::new(0.5f32, 0.5f32),
                        color::PURPLE,
                    );
                    self.render.create_rect(
                        Vector2::new(0.5f32 + offset, 1.5f32),
                        Vector2::new(0.5f32, 0.5f32),
                        color::BLUE,
                    );
                }
                self.triangle = self.render
                    .create_triangle(Vector2::new(0.0, 1.0), 1f32, color::YELLOW);
                self.render.send();
                self.state = GameState::Running;
            },
            GameState::Running => {
                let delta = self.clock.get_delta();
                if self.translation.x > 6f32 {
                    self.translation.x = 0f32;
                }
                self.translation.x += 1f32 * delta;
                self.render.update_triangle(
                    &self.triangle,
                    Vector2::new(self.translation.x - 6f32, 1.0),
                    -1f32,
                    color::GREEN,
                );
                self.render.set_translation(self.translation);
                self.render.send();
            },
        }
        self.clock.tick();
    }
}
