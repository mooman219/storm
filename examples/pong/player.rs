use storm::cgmath::{Vector2, Vector3};
use storm::math::aabb::AABB2D;
use storm::render::color;
use storm::render::message::*;
use storm::utility::indexmap::*;

use pong::Ball;
//Writing this for record keepign
pub struct Player {
    pub box_token: IndexToken,
    pub box_position: Vector3<f32>,
    pub box_shape: Vector2<f32>,
    pub col: color::Color,
}

impl Player {
    pub fn new(
        box_token: IndexToken,
        box_position: Vector3<f32>,
        box_shape: Vector2<f32>,
        col: color::Color,
    ) -> Player {
        Player {
            box_token,
            box_position,
            box_shape,
            col,
        }
    }

    pub fn overlaps_box(&self, ball: &Ball) -> bool {
        let my_aabb = AABB2D::new(
            self.box_position.x,
            self.box_position.y,
            self.box_position.x + self.box_shape.x,
            self.box_position.y + self.box_shape.y,
        );

        let thier_aabb = AABB2D::new(
            ball.ball_position.x,
            ball.ball_position.y,
            ball.ball_position.x + ball.ball_shape.x,
            ball.ball_position.y + ball.ball_shape.y,
        );

        return my_aabb.intersects(&thier_aabb);
    }

    pub fn render(&self, render: &mut RenderMessenger) {
        render.quad_update(self.box_token, self.box_position, self.box_shape, self.col);
    }
}
