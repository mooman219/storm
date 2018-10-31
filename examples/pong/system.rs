use rand;

use rand::distributions::{Range, Sample};
use storm::cgmath::{Vector2, Vector3};
use storm::input::message::*;
use storm::input::message::InputFrame::KeyPressed;
use storm::render::color;
use storm::render::message::*;
use storm::utility::indexmap::*;

enum Player {
    A,
    B
}

pub struct System {
    player_a: IndexToken,
    player_a_position: Vector3<f32>,
    player_a_shape: Vector2<f32>
    player_b: IndexToken,
    player_b_position: Vector3<f32>,
    player_b_shape: Vector2<f32>
    count: f32,
    ball: IndexToken,
    ball_postion: Vector3<f32>,
    ball_shape: Vector2<f32>,
    direction: f32,
}

impl System  {
    pub fn new(render: &mut RenderMessenger) -> System {
        render.set_scale(0.001f32);

        let player_a = render.create_rect(Vector3::new(0.0, 500.0, 0.0), Vector2::new(100.0, 100.0), color::PURPLE);
        let player_b = render.create_rect(Vector3::new(910.0, 500.0, 0.0), Vector2::new(100.0, 100.0), color::ORANGE);
        let ball_postion = Vector3::new(500.0, 500.0, 0.0);
        let ball_shape = Vector2::new(50.0, 50.0);
        let ball = render.create_rect(ball_postion.clone(), ball_shape, color::RED);
        

        render.send();

        System {
            player_a,
            player_b,
            ball,
            ball_postion,
            ball_shape,
            count: 500.0,
            direction: -1.0
        }
    }

    pub fn input(&mut self, event: InputFrame) {

        match event {
            KeyPressed(k) => {
                if k == Key::Up {
                    self.count += 10.0;
                }
                else if k == Key::Down {
                    self.count -= 10.0;
                }
            },
            _ => {

            }
        }
    }

    fn is_ball_overlapping() -> Option<Player> {



        return None;
    }

    pub fn tick(&mut self, render: &mut RenderMessenger)  {
        render.update_rect(self.ball, self.ball_postion, self.ball_shape, color::RED);
        self.ball_postion.x = self.count;
        self.count += 10.0 * self.direction;
    }
}