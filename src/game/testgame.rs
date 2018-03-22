use cgmath::*;
use engine::*;
use render::color;
use render::message::producer::*;
use time::clock::*;
use utility::slotmap::*;

pub struct TestGame {
    state: GameState,
    clock: Clock,
    translation: Vector3<f32>,
    triangle: IndexToken,
}

impl TestGame {
    pub fn new() -> TestGame {
        TestGame {
            state: GameState::Startup,
            clock: Clock::new(200),
            translation: Vector3::new(0f32, 0f32, 0f32),
            triangle: IndexToken::invalid(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum GameState {
    Startup,
    Running,
}

impl Game for TestGame {
    const TITLE: &'static str = "Test Game";

    fn tick(&mut self, render: &mut RenderProducer) {
        match self.state {
            GameState::Startup => {
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
                self.triangle = render.create_triangle(Vector2::new(0.0, 1.0), 1f32, color::YELLOW);
                render.send();
                self.state = GameState::Running;
            },
            GameState::Running => {
                let delta = self.clock.get_delta();
                if self.translation.x > 6f32 {
                    self.translation.x = 0f32;
                }
                self.translation.x += 1f32 * delta;
                render.update_triangle(
                    &self.triangle,
                    Vector2::new(self.translation.x - 6f32, 1.0),
                    -1f32,
                    color::GREEN,
                );
                render.set_translation(self.translation);
                render.send();
            },
        }
        self.clock.tick();
    }
}
