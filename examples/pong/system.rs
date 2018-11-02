use storm::cgmath::{Vector2, Vector3};
use storm::input::message::InputFrame::{KeyPressed, KeyReleased};
use storm::input::message::*;
use storm::render::color;
use storm::render::message::*;

use pong::Ball;
use pong::Player;

const BALL_X_SPEED: f32 = 0.1;
const PLAYER_Y_SPEED: f32 = 0.05;

enum PlayerType {
    A,
    B,
}

pub struct System {
    player_a: Player,
    player_a_direction: f32,
    player_b: Player,
    ball: Ball,
    count: f32,
    direction: f32,
}

impl System {
    pub fn new(render: &mut RenderMessenger) -> System {
        let player_a_position = Vector3::new(-2.5, -0.5, 0.0);
        let player_a_shape = Vector2::new(0.5, 1.0);
        let player_a_token = render.create_rect(player_a_position, player_a_shape, color::PURPLE);
        let player_a = Player::new(player_a_token, player_a_position, player_a_shape, color::PURPLE);

        let player_b_position = Vector3::new(2.0, -0.5, 0.0);
        let player_b_shape = Vector2::new(0.5, 1.0);
        let player_b_token = render.create_rect(player_b_position, player_b_shape, color::ORANGE);
        let player_b = Player::new(player_b_token, player_b_position, player_b_shape, color::ORANGE);

        let ball_postion = Vector3::new(-0.25, -0.25, 0.0);
        let ball_shape = Vector2::new(0.5, 0.5);
        let ball_token = render.create_rect(ball_postion, ball_shape, color::RED);
        let ball = Ball::new(ball_token, ball_postion, ball_shape);

        render.send();

        System {
            player_a,
            player_a_direction: 0.0,
            player_b,
            ball,
            count: 0.5,
            direction: -1.0,
        }
    }

    pub fn input(&mut self, event: InputFrame) {
        match event {
            KeyPressed(k) => {
                if k == Key::Up {
                    self.player_a_direction = 1.0;
                } else if k == Key::Down {
                    self.player_a_direction = -1.0;
                }
            },
            KeyReleased(k) => {
                if k == Key::Up || k == Key::Down {
                    self.player_a_direction = 0.0;
                }
            },
            _ => {},
        }
    }

    fn is_ball_overlapping(&self) -> Option<PlayerType> {
        if self.player_a.overlaps_box(&self.ball) {
            return Some(PlayerType::A);
        }

        if self.player_b.overlaps_box(&self.ball) {
            return Some(PlayerType::B);
        }
        return None;
    }

    pub fn tick(&mut self, render: &mut RenderMessenger) {
        let result = self.is_ball_overlapping();
        if result.is_some() {
            self.direction = -1.0 * self.direction;
            self.count += (BALL_X_SPEED * self.direction) * 5.0;
        }

        self.ball.ball_position.x = self.count;
        self.count += BALL_X_SPEED * self.direction;

        self.player_a.box_position.y += self.player_a_direction * PLAYER_Y_SPEED;

        render.update_rect(
            self.ball.ball_token,
            self.ball.ball_position,
            self.ball.ball_shape,
            color::RED,
        );
        self.player_a.render(render);
    }
}
